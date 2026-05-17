use crate::args::Args;
use crate::file_selection::{self, FileSelector};
use fua_core::config::{FormatterConfig, PluginConfig};
use fua_core::engine::FormatEngine;
use glob::glob;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

type CliResult<T> = Result<T, String>;

pub(crate) fn run(args: Args) -> CliResult<()> {
    let mut config = load_formatter_config(args.config.as_deref())?;
    apply_cli_overrides(&mut config, &args);
    let plugin_configs = resolve_plugin_configs(&config, args.config.as_deref(), &args.plugin);

    if args.only_staged || args.all || args.changed {
        return run_batch_mode(&args, config, &plugin_configs);
    }

    if args.input.is_empty() {
        // stdin → stdout / --output
        let input = read_stdin()?;
        ensure_input_not_empty(&input)?;
        let output = run_engine(&input, config, &plugin_configs)?;
        write_output(args.output.as_deref(), &output)?;
    } else {
        let paths = expand_globs(&args.input)?;

        if paths.len() == 1 {
            // Single file: support --output like before
            let input = read_file(&paths[0])?;
            ensure_input_not_empty(&input)?;
            let output = run_engine(&input, config, &plugin_configs)?;
            write_output(args.output.as_deref(), &output)?;
        } else {
            // Multiple files: format each one in-place
            if args.output.is_some() {
                return Err(
                    "--output cannot be used when multiple input files are matched; \
                     files are formatted in-place."
                        .to_string(),
                );
            }
            let total = paths.len();
            for path in &paths {
                let input = read_file(path)?;
                if input.trim().is_empty() {
                    continue;
                }
                let formatted = run_engine(&input, config.clone(), &plugin_configs)?;
                fs::write(path, &formatted).map_err(|e| {
                    format!("failed to write '{}': {e}", path.display())
                })?;
                println!("formatted: {}", path.display());
            }
            println!("done — {total} file(s) formatted.");
        }
    }

    Ok(())
}

fn run_batch_mode(
    args: &Args,
    config: FormatterConfig,
    plugin_configs: &[PluginConfig],
) -> CliResult<()> {
    if args.output.is_some() {
        return Err(
            "--output cannot be used with --only-staged or --all; files are formatted in-place."
                .to_string(),
        );
    }

    let repo_root = file_selection::git_repo_root()?;
    let selector = FileSelector::from_config(&config);
    let paths = if args.only_staged {
        selector.collect_staged_files(&repo_root)?
    } else if args.changed {
        let base_ref = file_selection::resolve_base_ref();
        selector.collect_changed_files(&repo_root, &base_ref)?
    } else {
        selector.collect_all_files(&repo_root)?
    };

    if paths.is_empty() {
        println!("no files to format.");
        return Ok(());
    }

    let total_candidates = paths.len();
    let mut formatted_paths = Vec::new();
    let mut needs_formatting = Vec::new();
    for path in paths {
        let input = read_file(&path)?;
        if input.trim().is_empty() {
            continue;
        }
        let output = run_engine(&input, config.clone(), plugin_configs)?;
        if args.check {
            if output != input {
                needs_formatting.push(path.display().to_string());
            }
            continue;
        }
        fs::write(&path, &output).map_err(|error| {
            format!("failed to write '{}': {error}", path.display())
        })?;
        println!("formatted: {}", path.display());
        formatted_paths.push(path);
    }

    if args.check {
        if needs_formatting.is_empty() {
            println!("all {total_candidates} file(s) are formatted.");
            return Ok(());
        }
        return Err(format!(
            "{} file(s) need formatting:\n  {}\n\nRun: fua-fua --only-staged --config .fua/config.json",
            needs_formatting.len(),
            needs_formatting.join("\n  ")
        ));
    }

    if args.only_staged {
        file_selection::restage_files(&formatted_paths, &repo_root)?;
    }

    println!(
        "done — {} file(s) formatted.",
        formatted_paths.len()
    );
    Ok(())
}

fn expand_globs(patterns: &[String]) -> CliResult<Vec<PathBuf>> {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("<unknown>"));
    let mut paths = Vec::new();
    for pattern in patterns {
        let entries = glob(pattern).map_err(|e| {
            format!(
                "invalid glob pattern '{}': {e}\n  cwd: {}",
                pattern,
                cwd.display()
            )
        })?;
        for entry in entries {
            let path = entry.map_err(|e| format!("glob error: {e}"))?;
            if path.is_file() {
                paths.push(path);
            }
        }
    }
    if paths.is_empty() {
        return Err(format!(
            "no files matched the pattern(s): {}\n  cwd: {}",
            patterns.join(", "),
            cwd.display()
        ));
    }
    Ok(paths)
}

fn read_file(path: &Path) -> CliResult<String> {
    fs::read_to_string(path)
        .map_err(|error| format!("failed to read input file '{}': {error}", path.display()))
}

fn read_stdin() -> CliResult<String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read stdin: {error}"))?;
    Ok(input)
}

fn ensure_input_not_empty(input: &str) -> CliResult<()> {
    if input.trim().is_empty() {
        Err("No HTML provided.".to_string())
    } else {
        Ok(())
    }
}

fn load_formatter_config(path: Option<&Path>) -> CliResult<FormatterConfig> {
    let Some(path) = path else {
        return Ok(FormatterConfig::default());
    };

    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("<unknown>"));
    let normalized = normalize_path(path);
    let abs = if normalized.is_absolute() {
        normalized
    } else {
        cwd.join(normalized)
    };

    let config_str = fs::read_to_string(&abs).map_err(|error| {
        format!(
            "failed to read config file '{}' (resolved to '{}'): {error}\n  cwd: {}",
            path.display(),
            abs.display(),
            cwd.display(),
        )
    })?;
    serde_json::from_str(&config_str)
        .map_err(|error| format!("failed to parse config file '{}': {error}", path.display()))
}

fn normalize_path(path: &Path) -> PathBuf {
    path.components().collect()
}

fn apply_cli_overrides(config: &mut FormatterConfig, args: &Args) {
    if let Some(indent_size) = args.indent_size {
        config.indent_size = indent_size;
    }

    if let Some(use_tabs) = args.use_tabs {
        config.use_tabs = use_tabs;
    }
}

fn resolve_plugin_configs(
    config: &FormatterConfig,
    config_path: Option<&Path>,
    cli_plugins: &[PathBuf],
) -> Vec<PluginConfig> {
    let mut plugins = resolve_configured_plugins(config, config_path);
    plugins.extend(cli_plugin_paths_to_configs(cli_plugins));
    plugins
}

fn resolve_configured_plugins(
    config: &FormatterConfig,
    config_path: Option<&Path>,
) -> Vec<PluginConfig> {
    config
        .plugin_configs()
        .into_iter()
        .map(|mut plugin| {
            if let Some(path) = plugin.path.as_deref() {
                plugin.path = Some(
                    resolve_plugin_path(Path::new(path), config_path)
                        .to_string_lossy()
                        .into_owned(),
                );
            }
            plugin
        })
        .collect()
}

fn cli_plugin_paths_to_configs(plugin_paths: &[PathBuf]) -> Vec<PluginConfig> {
    plugin_paths
        .iter()
        .map(|path| PluginConfig {
            path: Some(path.to_string_lossy().into_owned()),
            options: None,
        })
        .collect()
}

fn resolve_plugin_path(plugin_path: &Path, config_path: Option<&Path>) -> PathBuf {
    let path = plugin_path;
    if path.is_absolute() {
        return path.to_path_buf();
    }

    let Some(config_path) = config_path else {
        return path.to_path_buf();
    };

    let config_dir = config_path.parent().unwrap_or(Path::new("."));
    config_dir.join(path)
}

fn run_engine(input: &str, config: FormatterConfig, plugins: &[PluginConfig]) -> CliResult<String> {
    let (masked_input, comments) = mask_html_comments(input);
    let mut engine = FormatEngine::new(config);
    load_plugin_configs(&mut engine, plugins)?;
    let formatted = engine.format(&masked_input);
    Ok(restore_html_comments(formatted, &comments))
}

fn mask_html_comments(input: &str) -> (String, Vec<String>) {
    let mut output = String::with_capacity(input.len());
    let mut comments = Vec::new();
    let mut cursor = 0usize;

    while let Some(relative_start) = input[cursor..].find("<!--") {
        let start = cursor + relative_start;
        output.push_str(&input[cursor..start]);

        let after_open = start + 4;
        let end = input[after_open..]
            .find("-->")
            .map(|relative_end| after_open + relative_end + 3)
            .unwrap_or(input.len());

        comments.push(input[start..end].to_string());
        let placeholder = format!("__FUA_COMMENT_BLOCK_{}__", comments.len() - 1);
        output.push_str(&placeholder);
        cursor = end;
    }

    output.push_str(&input[cursor..]);
    (output, comments)
}

fn restore_html_comments(mut formatted: String, comments: &[String]) -> String {
    for (index, comment) in comments.iter().enumerate() {
        let placeholder = format!("__FUA_COMMENT_BLOCK_{index}__");
        formatted = formatted.replace(&placeholder, comment);
    }
    formatted
}

fn load_plugin_configs(engine: &mut FormatEngine, plugins: &[PluginConfig]) -> CliResult<()> {
    for plugin in plugins {
        if let Err(error) = engine.load_plugin_config(plugin) {
            let path = plugin.path.as_deref().unwrap_or("<missing>");
            return Err(format!("failed to load plugin '{path}': {error}"));
        }
    }

    Ok(())
}

fn write_output(path: Option<&Path>, output: &str) -> CliResult<()> {
    if let Some(path) = path {
        fs::write(path, output).map_err(|error| {
            format!("failed to write output file '{}': {error}", path.display())
        })?;
        println!("Successfully formatted into: {}", path.display());
    } else {
        print!("{output}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_args() -> Args {
        Args {
            input: Vec::new(),
            output: None,
            config: None,
            indent_size: None,
            use_tabs: None,
            plugin: Vec::new(),
            only_staged: false,
            all: false,
            changed: false,
            check: false,
        }
    }

    #[test]
    fn cli_overrides_replace_config_values() {
        let mut config = FormatterConfig::default();
        let mut args = sample_args();
        args.indent_size = Some(4);
        args.use_tabs = Some(true);

        apply_cli_overrides(&mut config, &args);

        assert_eq!(config.indent_size, 4);
        assert!(config.use_tabs);
    }

    #[test]
    fn relative_plugin_paths_are_resolved_from_config_directory() {
        let config_dir = Path::new("/project");
        let resolved = resolve_plugin_path(
            Path::new("plugins/angular.wasm"),
            Some(config_dir.join("config.json").as_path()),
        );

        assert_eq!(resolved, config_dir.join("plugins/angular.wasm"));
    }

    #[test]
    fn requested_plugins_merge_configured_and_cli_sources() {
        let config_dir = Path::new("/project");
        let config = FormatterConfig {
            plugins: vec![PluginConfig {
                path: Some("plugins/configured.wasm".to_string()),
                options: None,
            }],
            plugin: PluginConfig {
                path: Some("plugins/legacy.wasm".to_string()),
                options: None,
            },
            ..FormatterConfig::default()
        };

        let plugins = resolve_plugin_configs(
            &config,
            Some(config_dir.join("config.json").as_path()),
            &[PathBuf::from("/plugins/cli.wasm")],
        );

        assert_eq!(plugins.len(), 3);
        assert_eq!(
            plugins[0].path.as_deref().map(PathBuf::from),
            Some(config_dir.join("plugins/configured.wasm"))
        );
        assert_eq!(
            plugins[1].path.as_deref().map(PathBuf::from),
            Some(config_dir.join("plugins/legacy.wasm"))
        );
        assert_eq!(
            plugins[2].path.as_deref().map(PathBuf::from),
            Some(PathBuf::from("/plugins/cli.wasm"))
        );
    }

    #[test]
    fn comment_masking_round_trips_verbatim_comment_blocks() {
        let input = "<div></div>\n<!-- \n<div class=\"flex gap-4 p-4\">x</div>\n-->\n<p>after</p>";
        let (masked, comments) = mask_html_comments(input);
        assert!(!masked.contains("<!--"));
        assert_eq!(comments.len(), 1);

        let restored = restore_html_comments(masked, &comments);
        assert_eq!(restored, input);
    }
}
