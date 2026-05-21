use fua_core::config::FormatterConfig;
use glob::glob;
use regex::Regex;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Decides which project paths are eligible for formatting.
pub(crate) struct FileSelector {
    include_globs: Vec<String>,
    exclude_rules: Vec<ExcludeRule>,
}

enum ExcludeRule {
    Regex(Regex),
    Glob(glob::Pattern),
    Substring(String),
}

impl FileSelector {
    pub(crate) fn from_config(config: &FormatterConfig) -> Self {
        let include_globs = if config.include.is_empty() {
            vec!["**/*".to_string()]
        } else {
            config
                .include
                .iter()
                .map(|pattern| normalize_include_glob(pattern))
                .collect()
        };

        let exclude_rules = config
            .exclude
            .iter()
            .filter_map(|pattern| ExcludeRule::parse(pattern))
            .collect();

        Self {
            include_globs,
            exclude_rules,
        }
    }

    pub(crate) fn should_format(&self, path: &Path) -> bool {
        if !is_html_file(path) {
            return false;
        }

        let normalized = normalize_path_for_matching(path);
        self.matches_include(&normalized) && !self.matches_exclude(&normalized)
    }

    fn should_format_relative_to(&self, path: &Path, repo_root: &Path) -> bool {
        let relative = path.strip_prefix(repo_root).unwrap_or(path);
        self.should_format(relative)
    }

    pub(crate) fn collect_staged_files(&self, repo_root: &Path) -> Result<Vec<PathBuf>, String> {
        let output = Command::new("git")
            .args([
                "diff",
                "--cached",
                "--name-only",
                "--diff-filter=ACMR",
                "-z",
            ])
            .current_dir(repo_root)
            .output()
            .map_err(|error| format!("failed to run git: {error}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("git diff --cached failed: {stderr}"));
        }

        let paths = parse_null_separated_paths(&output.stdout);
        Ok(self
            .filter_existing(repo_root, paths)
            .into_iter()
            .filter(|path| self.should_format_relative_to(path, repo_root))
            .collect())
    }

    pub(crate) fn collect_changed_files(
        &self,
        repo_root: &Path,
        base_ref: &str,
    ) -> Result<Vec<PathBuf>, String> {
        let merge_base = git_merge_base(repo_root, base_ref)?;
        let output = Command::new("git")
            .args([
                "diff",
                "--name-only",
                "--diff-filter=ACMR",
                "-z",
                &merge_base,
                "HEAD",
            ])
            .current_dir(repo_root)
            .output()
            .map_err(|error| format!("failed to run git: {error}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("git diff failed: {stderr}"));
        }

        let paths = parse_null_separated_paths(&output.stdout);
        Ok(self
            .filter_existing(repo_root, paths)
            .into_iter()
            .filter(|path| self.should_format_relative_to(path, repo_root))
            .collect())
    }

    pub(crate) fn collect_all_files(&self, repo_root: &Path) -> Result<Vec<PathBuf>, String> {
        let mut paths = Vec::new();
        for pattern in &self.include_globs {
            let full_pattern = repo_root.join(pattern);
            let pattern_str = full_pattern.to_string_lossy();
            let entries = glob(&pattern_str).map_err(|error| {
                format!("invalid include glob '{pattern}': {error}")
            })?;
            for entry in entries {
                let path = entry.map_err(|error| format!("glob error: {error}"))?;
                if path.is_file() && self.should_format(&path) {
                    paths.push(path);
                }
            }
        }

        paths.sort();
        paths.dedup();
        Ok(paths)
    }

    fn filter_existing(&self, repo_root: &Path, paths: Vec<PathBuf>) -> Vec<PathBuf> {
        paths
            .into_iter()
            .map(|path| {
                if path.is_absolute() {
                    path
                } else {
                    repo_root.join(path)
                }
            })
            .filter(|path| path.is_file())
            .collect()
    }

    fn matches_include(&self, normalized_path: &str) -> bool {
        self.include_globs.iter().any(|pattern| {
            glob::Pattern::new(pattern)
                .map(|compiled| compiled.matches(normalized_path))
                .unwrap_or(false)
        })
    }

    fn matches_exclude(&self, normalized_path: &str) -> bool {
        self.exclude_rules
            .iter()
            .any(|rule| rule.matches(normalized_path))
    }
}

impl ExcludeRule {
    fn parse(pattern: &str) -> Option<Self> {
        if let Ok(regex) = Regex::new(pattern) {
            return Some(Self::Regex(regex));
        }

        if pattern.contains('*') || pattern.contains('?') {
            return glob::Pattern::new(pattern)
                .ok()
                .map(Self::Glob);
        }

        Some(Self::Substring(pattern.to_string()))
    }

    fn matches(&self, normalized_path: &str) -> bool {
        match self {
            Self::Regex(regex) => regex.is_match(normalized_path),
            Self::Glob(pattern) => pattern.matches(normalized_path),
            Self::Substring(fragment) => normalized_path.contains(fragment),
        }
    }
}

pub(crate) fn resolve_base_ref() -> String {
    if let Ok(base) = env::var("GITHUB_BASE_REF") {
        return format!("origin/{base}");
    }
    if let Ok(base) = env::var("FUA_BASE_REF") {
        return base;
    }
    "origin/master".to_string()
}

fn git_merge_base(repo_root: &Path, base_ref: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(["merge-base", "HEAD", base_ref])
        .current_dir(repo_root)
        .output()
        .map_err(|error| format!("failed to run git: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "git merge-base HEAD {base_ref} failed: {stderr}\n  \
             fetch the base branch or set FUA_BASE_REF"
        ));
    }

    Ok(String::from_utf8(output.stdout)
        .map_err(|error| format!("invalid git output: {error}"))?
        .trim()
        .to_string())
}

pub(crate) fn git_repo_root() -> Result<PathBuf, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|error| format!("failed to run git: {error}"))?;

    if !output.status.success() {
        return Err("not inside a git repository".to_string());
    }

    let root = String::from_utf8(output.stdout)
        .map_err(|error| format!("invalid git output: {error}"))?
        .trim()
        .to_string();

    Ok(PathBuf::from(root))
}

pub(crate) fn restage_files(paths: &[PathBuf], repo_root: &Path) -> Result<(), String> {
    if paths.is_empty() {
        return Ok(());
    }

    let mut command = Command::new("git");
    command.arg("add").current_dir(repo_root);
    for path in paths {
        let relative = path
            .strip_prefix(repo_root)
            .unwrap_or(path)
            .to_string_lossy();
        command.arg(relative.as_ref());
    }

    let output = command
        .output()
        .map_err(|error| format!("failed to run git add: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git add failed: {stderr}"));
    }

    Ok(())
}

fn normalize_include_glob(pattern: &str) -> String {
    match pattern {
        "*" => "**/*".to_string(),
        other => other.replace('\\', "/"),
    }
}

fn normalize_path_for_matching(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn is_html_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("html") || ext.eq_ignore_ascii_case("htm"))
}

fn parse_null_separated_paths(bytes: &[u8]) -> Vec<PathBuf> {
    bytes
        .split(|byte| *byte == 0)
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| PathBuf::from(String::from_utf8_lossy(chunk).as_ref()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selector(include: &[&str], exclude: &[&str]) -> FileSelector {
        FileSelector::from_config(&FormatterConfig {
            include: include.iter().map(|s| (*s).to_string()).collect(),
            exclude: exclude.iter().map(|s| (*s).to_string()).collect(),
            ..FormatterConfig::default()
        })
    }

    #[test]
    fn default_include_matches_html_files() {
        let selector = FileSelector::from_config(&FormatterConfig::default());
        assert!(selector.should_format(Path::new("src/app.component.html")));
        assert!(!selector.should_format(Path::new("src/app.component.ts")));
    }

    #[test]
    fn include_html_glob_limits_candidates() {
        let selector = selector(&["**/*.html"], &[]);
        assert!(selector.should_format(Path::new("templates/page.html")));
        assert!(!selector.should_format(Path::new("templates/page.ts")));
    }

    #[test]
    fn exclude_substring_skips_matching_paths() {
        let selector = selector(&["**/*"], &["pop-up.component.html"]);
        assert!(!selector.should_format(Path::new("src/pop-up.component.html")));
        assert!(selector.should_format(Path::new("src/home.component.html")));
    }

    #[test]
    fn exclude_regex_skips_extension() {
        let selector = selector(&["**/*"], &[r"\.ts$"]);
        assert!(!selector.should_format(Path::new("src/app.ts")));
        assert!(selector.should_format(Path::new("src/app.html")));
    }

    #[test]
    fn star_include_normalizes_to_recursive_glob() {
        let selector = selector(&["*"], &[]);
        assert!(selector.matches_include("nested/dir/file.html"));
    }

    #[test]
    fn demo_include_exclude_rules() {
        let selector = selector(
            &["examples/demo-include-exclude/**/*.html"],
            &["pop-up\\.component\\.html", "legacy/"],
        );
        assert!(selector.should_format(Path::new(
            "examples/demo-include-exclude/src/checkout.component.html",
        )));
        assert!(selector.should_format(Path::new(
            "examples/demo-include-exclude/src/home.component.html",
        )));
        assert!(!selector.should_format(Path::new(
            "examples/demo-include-exclude/src/pop-up.component.html",
        )));
        assert!(!selector.should_format(Path::new(
            "examples/demo-include-exclude/legacy/old-page.html",
        )));
        assert!(!selector.should_format(Path::new(
            "examples/demo-include-exclude/src/notes.ts",
        )));
    }
}
