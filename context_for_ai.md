# PROJECT CONTEXT REPORT
Generated: 2026-04-18 18:32:25

## 1. PROJECT STRUCTURE
```text
fua-fua-format/
|-- crates/
|   |-- cli/
|   |   |-- npm/
|   |   |   |-- package.json
|   |   |   `-- run.js
|   |   |-- src/
|   |   |   |-- app.rs
|   |   |   |-- args.rs
|   |   |   `-- main.rs
|   |   `-- Cargo.toml
|   |-- core/
|   |   |-- src/
|   |   |   |-- formatter/
|   |   |   |   |-- content.rs
|   |   |   |   |-- context.rs
|   |   |   |   |-- hooks.rs
|   |   |   |   |-- output.rs
|   |   |   |   |-- tags.rs
|   |   |   |   `-- traversal.rs
|   |   |   |-- architecture_tests.rs
|   |   |   |-- config.rs
|   |   |   |-- engine.rs
|   |   |   |-- formatter.rs
|   |   |   |-- lexer.rs
|   |   |   |-- lib.rs
|   |   |   |-- parser.rs
|   |   |   |-- plugins.rs
|   |   |   `-- syntax.rs
|   |   `-- Cargo.toml
|   |-- fua-plugin-angular/
|   |   |-- src/
|   |   |   |-- attributes.rs
|   |   |   |-- context.rs
|   |   |   |-- expressions.rs
|   |   |   |-- hooks.rs
|   |   |   |-- lib.rs
|   |   |   |-- response.rs
|   |   |   `-- state.rs
|   |   `-- Cargo.toml
|   `-- fua-plugin-api/
|       |-- src/
|       |   `-- lib.rs
|       `-- Cargo.toml
|-- examples/
|   |-- config.json
|   |-- formatted.html
|   |-- formatted_angular.html
|   |-- formatted_ideal.html
|   |-- formatted_sample.html
|   |-- ideal.html
|   |-- react_example.jsx
|   |-- react_formatted.jsx
|   |-- sample.html
|   |-- test_plain_formatted.html
|   |-- vue_example.html
|   `-- vue_formatted.html
|-- .gitignore
|-- Cargo.toml
|-- generate-context.py
|-- LICENSE
`-- README.md
```

---
## 2. PROJECT STATISTICS
**Total Files Scanned:** 49

| Extension | Count |
|---|---|
| .rs | 26 |
| .html | 9 |
| .toml | 5 |
| (no extension) | 2 |
| .json | 2 |
| .jsx | 2 |
| .js | 1 |
| .md | 1 |
| .py | 1 |

---
## 3. FILE CONTENTS

# FILE: .gitignore
```text
# Cargo build directories
/target/

# Editor and IDE directories
.idea/
.vscode/
*.swp
*.swo
*~
*.bak

# Operating System files
.DS_Store
Thumbs.db

```

# FILE: Cargo.toml
```toml
[workspace]
members = [
    "crates/cli",
    "crates/core",
    "crates/fua-plugin-api",
    # extism-pdk uses WASM-only host imports and cannot link for native targets.
    # This crate is in the workspace so it can use workspace deps, but is
    # excluded from the default build.  Compile it explicitly with:
    #   cargo build -p fua-plugin-angular --target wasm32-wasip1 --release
    "crates/fua-plugin-angular",
]
# Only cli + core compile by default; the WASM plugin is opt-in.
default-members = ["crates/cli", "crates/core"]
resolver = "2"

[workspace.dependencies]
rowan = "0.15"
num-traits = "0.2"
num-derive = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

```

# FILE: LICENSE
```text
MIT License

Copyright (c) 2024 Candid Moon _Max_

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

```

# FILE: README.md
```markdown
# Fua Fua Format

A lightning-fast, highly-permissive HTML formatter written in Rust.

Fua Fua Format is built around a lossless HTML core plus optional WASM plugins. The core parses and formats plain HTML rules only. Framework-specific behavior lives in separate plugin crates and is reached through a generic host hook API, so the formatter can grow without folding framework branches back into the parser, formatter, or CLI.

## Features
- **Framework-Agnostic Core**: The default formatter applies only generic HTML rules.
- **Optional WASM Plugins**: Framework-specific formatting lives in separate plugin crates.
- **Lossless Syntax Tree**: Guarantees zero data loss or layout corruption during formatting. 
- **Highly Configurable**: Control behavior with extensive config files or CLI arguments.
- **Microsecond Performance**: Built on top of ultra-fast Rust lexers utilized by `rust-analyzer`.

## Usage

You can use the formatter directly through the CLI:

```bash
# Format a file and output to stdout
cargo run -p cli -- --input my_file.html

# Format a file explicitly overriding tab behavior and indent size
cargo run -p cli -- --input my_file.html --output formatted.html --use-tabs true

# Format via a configuration file
cargo run -p cli -- --input examples/sample.html --config examples/config.json --output examples/formatted.html
```

### CLI Arguments
* `-i, --input <file>`: Input file path. Reads from `stdin` if not provided.
* `-o, --output <file>`: Output file path. Writes to `stdout` by default.
* `-c, --config <json file>`: Path to your formatting configuration definitions.
* `--indent-size <number>`: Override the indent size explicitly.
* `--use-tabs <bool>`: Override the whitespace strategy explicitly.
* `--plugin <file>`: Load a compiled WASM plugin. Repeat to load multiple plugins.

## Configuration (`config.json`)

Fua Fua Format supports the following configuration properties directly fed via JSON:

```json
{
  "indent_size": 4,
  "use_tabs": false,
  "print_width": 100,
  "bracket_same_line": false,
  "wrap_attributes": true,
  "single_quotes": false,
  "wrap_content": true,
  "plugins": [
    {
      "path": "../target/wasm32-wasip1/release/fua_plugin_angular.wasm",
      "options": {
        "wrap_conditions_in_parens": false,
        "indent_condition_groups": false
      }
    }
  ]
}
```

### Configuration Options:

* `indent_size` *(Integer, Default: 2)*
  Number of spaces to use per indentation level. Replaced entirely if `use_tabs` is enabled.
* `use_tabs` *(Boolean, Default: false)*
  Whether to format code using `\t` (tabs) instead of whitespace spaces.
* `print_width` *(Integer, Default: 80)*
  The line length limit that triggers dynamic wrapping.
* `bracket_same_line` *(Boolean, Default: false)*
  When tags break into multiple lines, determines whether the closing bracket `>` goes on the last line next to the attribute, or visually pops onto a new indented line.
* `wrap_attributes` *(Boolean, Default: false)*
  Forces elements to break attributes onto newly indented multi-lines instead of preserving them inline.
* `single_quotes` *(Boolean, Default: false)*
  Convert all HTML standard `"` double-quotes into `'` single-quotes natively (escapes strictly preserved).
* `wrap_content` *(Boolean, Default: false)*
  If an opening tag breaks into multiple lines, this ensures the internal raw text (or immediate child string) drops symmetrically to the next appropriate line down.
* `plugins` *(Array, Default: empty)*
  Ordered list of optional WASM plugins to load after the default HTML formatter pass.
* `plugin` *(Object, Legacy)*
  Backward-compatible single-plugin entry. New configs should prefer `plugins`.
* `plugins[].options` *(Object, Plugin-specific)*
  Arbitrary JSON options forwarded to the selected plugin on each hook request.

## Architecture

Fua Fua Format is split into four workspace crates:
- `fua-core`: Generic HTML lexer, parser, formatter, plugin host, and formatting engine.
- `fua-plugin-api`: Stable hook contract shared by the core host and every plugin crate.
- `fua-plugin-angular`: Angular-specific formatting rules compiled to WASM.
- `cli`: Thin Clap-based command runner for file I/O, config loading, and plugin wiring.

### Module layout

The project is organized so the top-level flow stays simple:

`read input -> load config -> load plugins -> parse -> format -> write output`

The main responsibilities are separated like this:

- `crates/cli/src/main.rs`: Minimal binary entry point that parses arguments and delegates to the app runner.
- `crates/cli/src/app.rs`: CLI orchestration for I/O, config loading, plugin path resolution, and engine execution.
- `crates/core/src/engine.rs`: High-level formatter pipeline that turns input text into formatted output.
- `crates/core/src/parser.rs`, `lexer.rs`, `syntax.rs`: Lossless HTML tokenization and syntax tree construction.
- `crates/core/src/formatter/`: Core formatting implementation split by responsibility:
  - traversal through the syntax tree,
  - tag formatting,
  - content/token formatting,
  - plugin hook dispatch,
  - output/indentation emission,
  - syntax-context helpers.
- `crates/core/src/plugins.rs`: WASM plugin host and dispatch order.
- `crates/fua-plugin-angular/src/`: Angular-specific attribute wrapping, condition handling, shared expression parsing helpers, response builders, and plugin state.

```

# FILE: generate-context.py
```python
import datetime
import fnmatch
import os
from collections import Counter
from pathlib import Path
from typing import Iterable

# --- CONFIGURATION ---
OUTPUT_FILE = "context_for_ai.md"

# Directories to ignore completely.
IGNORE_DIRS = {
    ".git",
    ".idea",
    ".vscode",
    ".vs",
    ".venv",
    "venv",
    "env",
    "__pycache__",
    "node_modules",
    "dist",
    "build",
    "target",
    "vendor",
    "bin",
    "obj",
    "out",
    "debug",
    "release",
    "coverage",
    ".nuxt",
    ".next",
    "cmake-build-debug",
    ".angular",
    ".husky",
    ".storybook",
    ".pytest_cache",
    ".trunk",
    "logs",
    ".cursor",
    "postgres-data",
    "pgdata",
}

# Files or path globs to exclude from the report entirely.
# These are usually lockfiles, generated artifacts, local crash dumps, or the
# report itself.
IGNORE_PATH_PATTERNS = {
    OUTPUT_FILE,
    "Cargo.lock",
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "composer.lock",
    "go.sum",
    "Gemfile.lock",
    ".env",
    ".env.local",
    ".env.development",
    ".env.production",
    ".env.test",
    "secrets.yaml",
    "*.log",
    "*.exe",
    "*.dll",
    "*.so",
    "*.dylib",
    "*.pyc",
    "*.pyo",
    "*.pyd",
    "*.db",
    "*.csv",
    "*.pdf",
    "*.zip",
    "*.tar.gz",
    "*.swp",
    "*.swo",
    "*.bak",
    "*~",
    ".eslintcache",
    ".DS_Store",
    "Thumbs.db",
    "documentation.json",
    "audio-backend",
    "main",
    # Repo-specific scratch files that do not help model context.
    "panic*.txt",
    "test_output.html",
}

# Max file size to read (in bytes) to prevent bloating.
MAX_FILE_SIZE = 500 * 1024

# Map extensions to Markdown languages.
EXT_TO_LANG = {
    ".py": "python",
    ".js": "javascript",
    ".ts": "typescript",
    ".jsx": "jsx",
    ".tsx": "tsx",
    ".java": "java",
    ".c": "c",
    ".cpp": "cpp",
    ".cs": "csharp",
    ".go": "go",
    ".rs": "rust",
    ".php": "php",
    ".rb": "ruby",
    ".html": "html",
    ".css": "css",
    ".scss": "scss",
    ".json": "json",
    ".yaml": "yaml",
    ".yml": "yaml",
    ".xml": "xml",
    ".md": "markdown",
    ".sql": "sql",
    ".sh": "bash",
    ".bat": "batch",
    ".dockerfile": "dockerfile",
    "Dockerfile": "dockerfile",
    ".toml": "toml",
    ".ini": "ini",
    ".csproj": "xml",
}


def normalize_rel_path(path: Path) -> str:
    """Return relative paths in a stable, markdown-friendly format."""
    return path.as_posix()


def matches_patterns(rel_path: str, filename: str, patterns: Iterable[str]) -> bool:
    """Match both basename and relative path against glob patterns."""
    return any(
        fnmatch.fnmatch(filename, pattern) or fnmatch.fnmatch(rel_path, pattern)
        for pattern in patterns
    )


def should_skip_file(file_path: Path, root_dir: Path) -> bool:
    """Return True when a file should be excluded from the report."""
    rel_path = normalize_rel_path(file_path.relative_to(root_dir))
    return matches_patterns(rel_path, file_path.name, IGNORE_PATH_PATTERNS)


def iter_report_files(root_dir: Path):
    """Yield report-worthy files while applying one shared filter pipeline."""
    for current_root, dirs, files in os.walk(root_dir):
        dirs[:] = sorted(d for d in dirs if d not in IGNORE_DIRS)

        current_path = Path(current_root)
        for filename in sorted(files):
            file_path = current_path / filename
            if should_skip_file(file_path, root_dir):
                continue
            yield file_path


def build_tree(files, root_dir: Path) -> str:
    """Generate an ASCII tree from the already-filtered file list."""
    nested_tree = {}

    for file_path in files:
        rel_parts = file_path.relative_to(root_dir).parts
        cursor = nested_tree
        for directory in rel_parts[:-1]:
            cursor = cursor.setdefault(directory, {})
        cursor[rel_parts[-1]] = None

    lines = ["```text", f"{root_dir.name}/"]
    render_tree(nested_tree, lines)
    lines.append("```")
    return "\n".join(lines) + "\n"


def render_tree(tree, lines, prefix: str = "") -> None:
    """Render a nested dictionary produced by build_tree."""
    items = sorted(tree.items(), key=lambda item: (item[1] is None, item[0].lower()))

    for index, (name, child) in enumerate(items):
        is_last = index == len(items) - 1
        branch = "`-- " if is_last else "|-- "

        if child is None:
            lines.append(f"{prefix}{branch}{name}")
            continue

        lines.append(f"{prefix}{branch}{name}/")
        next_prefix = prefix + ("    " if is_last else "|   ")
        render_tree(child, lines, next_prefix)


def is_binary_file(filepath: Path) -> bool:
    """Check if a file is binary by reading a small chunk."""
    try:
        with filepath.open("rb") as handle:
            chunk = handle.read(1024)
            return b"\0" in chunk
    except OSError:
        return True


def get_language(filename: str) -> str:
    """Return the markdown language tag based on extension."""
    _, ext = os.path.splitext(filename)
    if filename in EXT_TO_LANG:
        return EXT_TO_LANG[filename]
    return EXT_TO_LANG.get(ext.lower(), "text")


def read_file_content(filepath: Path) -> str:
    """Read file content with size limit and binary checks."""
    if filepath.stat().st_size > MAX_FILE_SIZE:
        return f"[NOTE: File content skipped (Size > {MAX_FILE_SIZE / 1024:.1f} KB)]"

    if is_binary_file(filepath):
        return "[NOTE: Binary file detected and skipped]"

    for encoding in ("utf-8", "utf-16", "latin-1", "cp1252"):
        try:
            with filepath.open("r", encoding=encoding) as handle:
                return handle.read()
        except (OSError, UnicodeDecodeError):
            continue

    return "[ERROR: Could not decode file content]"


def get_project_stats(files) -> str:
    """Count filtered files by extension."""
    stats = Counter()

    for file_path in files:
        ext = file_path.suffix.lower() or "(no extension)"
        stats[ext] += 1

    report = f"**Total Files Scanned:** {len(files)}\n\n"
    report += "| Extension | Count |\n|---|---|\n"

    for ext, count in sorted(stats.items(), key=lambda item: (-item[1], item[0])):
        report += f"| {ext} | {count} |\n"

    return report


def collect_file_contents(files, root_dir: Path) -> str:
    """Collect content for each filtered file."""
    sections = []

    for file_path in files:
        rel_path = normalize_rel_path(file_path.relative_to(root_dir))
        lang = get_language(file_path.name)
        file_content = read_file_content(file_path)
        sections.append(f"\n# FILE: {rel_path}\n```{lang}\n{file_content}\n```\n")

    return "".join(sections)


def main() -> None:
    root_dir = Path.cwd()
    timestamp = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    print(f">>> Generating context file for project in: {root_dir}")
    print(">>> Please wait...")

    report_files = list(iter_report_files(root_dir))

    final_output = "# PROJECT CONTEXT REPORT\n"
    final_output += f"Generated: {timestamp}\n\n"

    final_output += "## 1. PROJECT STRUCTURE\n"
    final_output += build_tree(report_files, root_dir)
    final_output += "\n---\n"

    final_output += "## 2. PROJECT STATISTICS\n"
    final_output += get_project_stats(report_files)
    final_output += "\n---\n"

    final_output += "## 3. FILE CONTENTS\n"
    final_output += collect_file_contents(report_files, root_dir)

    output_path = root_dir / OUTPUT_FILE
    with output_path.open("w", encoding="utf-8") as handle:
        handle.write(final_output)

    size_mb = output_path.stat().st_size / (1024 * 1024)
    print(f"\n[SUCCESS] Context saved to: {OUTPUT_FILE}")
    print(f"   Size: {size_mb:.2f} MB")
    print("   You can now upload this file to ChatGPT/Claude/LLM.")


if __name__ == "__main__":
    main()

```

# FILE: crates/cli/Cargo.toml
```toml
[package]
name = "cli"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "fua-fua"
path = "src/main.rs"

[dependencies]
fua-core = { path = "../core" }
clap = { version = "4.5", features = ["derive"] }
serde_json = "1.0"

```

# FILE: crates/cli/npm/package.json
```json
{
  "name": "fua-fua-format",
  "version": "0.1.0-local",
  "description": "Blazing fast HTML formatter",
  "bin": {
    "fua-fua": "./run.js"
  }
}

```

# FILE: crates/cli/npm/run.js
```javascript
#!/usr/bin/env node
const { spawnSync } = require('child_process');
const path = require('path');
const os = require('os');

// Point directly to your local Rust build output
const isWin = os.platform() === 'win32';
const binaryName = isWin ? 'fua-fua.exe' : 'fua-fua';
const binaryPath = path.resolve(__dirname, '../../../target/release/', binaryName);

// Pass all arguments from the user directly to the Rust CLI
const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit'
});

process.exit(result.status || 0);

```

# FILE: crates/cli/src/app.rs
```rust
use crate::args::Args;
use fua_core::config::{FormatterConfig, PluginConfig};
use fua_core::engine::FormatEngine;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

type CliResult<T> = Result<T, String>;

pub(crate) fn run(args: Args) -> CliResult<()> {
    let input = read_input(args.input.as_deref())?;
    ensure_input_not_empty(&input)?;

    let mut config = load_formatter_config(args.config.as_deref())?;
    apply_cli_overrides(&mut config, &args);

    let plugin_configs = resolve_plugin_configs(&config, args.config.as_deref(), &args.plugin);
    let output = run_engine(&input, config, &plugin_configs)?;
    write_output(args.output.as_deref(), &output)?;

    Ok(())
}

fn read_input(path: Option<&Path>) -> CliResult<String> {
    if let Some(path) = path {
        fs::read_to_string(path)
            .map_err(|error| format!("failed to read input file '{}': {error}", path.display()))
    } else {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .map_err(|error| format!("failed to read stdin: {error}"))?;
        Ok(input)
    }
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

    let config_str = fs::read_to_string(path)
        .map_err(|error| format!("failed to read config file '{}': {error}", path.display()))?;
    serde_json::from_str(&config_str)
        .map_err(|error| format!("failed to parse config file '{}': {error}", path.display()))
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
    let mut engine = FormatEngine::new(config);
    load_plugin_configs(&mut engine, plugins)?;
    Ok(engine.format(input))
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
            input: None,
            output: None,
            config: None,
            indent_size: None,
            use_tabs: None,
            plugin: Vec::new(),
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
        let resolved = resolve_plugin_path(
            Path::new("plugins/angular.wasm"),
            Some(Path::new(r"C:\project\config.json")),
        );

        assert_eq!(resolved, PathBuf::from(r"C:\project\plugins\angular.wasm"));
    }

    #[test]
    fn requested_plugins_merge_configured_and_cli_sources() {
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
            Some(Path::new(r"C:\project\config.json")),
            &[PathBuf::from(r"C:\plugins\cli.wasm")],
        );

        assert_eq!(plugins.len(), 3);
        assert_eq!(
            plugins[0].path.as_deref().map(PathBuf::from),
            Some(PathBuf::from(r"C:\project\plugins\configured.wasm"))
        );
        assert_eq!(
            plugins[1].path.as_deref().map(PathBuf::from),
            Some(PathBuf::from(r"C:\project\plugins\legacy.wasm"))
        );
        assert_eq!(
            plugins[2].path.as_deref().map(PathBuf::from),
            Some(PathBuf::from(r"C:\plugins\cli.wasm"))
        );
    }
}

```

# FILE: crates/cli/src/args.rs
```rust
use clap::Parser as ClapParser;
use std::path::PathBuf;

#[derive(ClapParser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Input file path (or stdin if not provided)
    #[arg(short, long)]
    pub(crate) input: Option<PathBuf>,

    /// Output file path (or stdout if not provided)
    #[arg(short, long)]
    pub(crate) output: Option<PathBuf>,

    /// Path to a JSON configuration file
    #[arg(short, long)]
    pub(crate) config: Option<PathBuf>,

    /// Override indent size
    #[arg(long)]
    pub(crate) indent_size: Option<usize>,

    /// Use tabs instead of spaces
    #[arg(long)]
    pub(crate) use_tabs: Option<bool>,

    /// Path to a compiled .wasm formatter plugin. Repeat to load multiple plugins.
    #[arg(long)]
    pub(crate) plugin: Vec<PathBuf>,
}

```

# FILE: crates/cli/src/main.rs
```rust
mod app;
mod args;

use args::Args;
use clap::Parser as _;

fn main() {
    let args = Args::parse();

    if let Err(error) = app::run(args) {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

```

# FILE: crates/core/Cargo.toml
```toml
[package]
name = "fua-core"
version = "0.1.0"
edition = "2024"

[dependencies]
logos = "0.14"
rowan = { workspace = true }
num-traits = { workspace = true }
num-derive = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
extism = "1"
fua-plugin-api = { path = "../fua-plugin-api" }

```

# FILE: crates/core/src/architecture_tests.rs
```rust
#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    fn collect_rust_sources(dir: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        collect_rust_sources_into(dir, &mut files);
        files
    }

    fn collect_rust_sources_into(dir: &Path, files: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(dir).expect("read core source directory") {
            let entry = entry.expect("directory entry");
            let path = entry.path();
            if path.is_dir() {
                collect_rust_sources_into(&path, files);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }

    fn source_path(relative: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join(relative)
    }

    fn read_source(path: &Path) -> String {
        fs::read_to_string(path).expect("read source file")
    }

    #[test]
    fn core_source_stays_framework_agnostic() {
        let src_root = source_path("");
        let files = collect_rust_sources(&src_root);

        let forbidden_terms = [
            "angular", "vue", "jsx", "tailwind", "react", "svelte", "ngif", "ngclass", "ngmodel",
        ];

        for file in files {
            if file.file_name().and_then(|name| name.to_str()) == Some("architecture_tests.rs") {
                continue;
            }
            let contents = read_source(&file);
            let lower = contents.to_lowercase();
            for term in forbidden_terms {
                assert!(
                    !lower.contains(term),
                    "found forbidden framework term '{term}' in {}",
                    file.display()
                );
            }
        }
    }

    #[test]
    fn engine_stays_free_of_cli_and_io_concerns() {
        let engine_source = read_source(&source_path("engine.rs")).to_lowercase();
        let forbidden_terms = [
            "std::fs",
            "stdin",
            "stdout",
            "println!",
            "eprintln!",
            "clap::",
        ];

        for term in forbidden_terms {
            assert!(
                !engine_source.contains(term),
                "engine.rs should remain an orchestration layer without '{term}'"
            );
        }
    }

    #[test]
    fn formatter_modules_do_not_depend_on_the_parser_layer() {
        let formatter_root = source_path("formatter");

        for file in collect_rust_sources(&formatter_root) {
            let contents = read_source(&file);
            assert!(
                !contents.contains("crate::parser"),
                "formatter module should not import the parser directly: {}",
                file.display()
            );
            assert!(
                !contents.contains("Parser::new"),
                "formatter module should not construct parsers directly: {}",
                file.display()
            );
        }
    }
}

```

# FILE: crates/core/src/config.rs
```rust
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Plugin-specific section of the formatter config.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PluginConfig {
    /// Path to the compiled `.wasm` plugin file.
    /// When present the CLI will load it automatically (no `--plugin` flag needed).
    pub path: Option<String>,
    /// Arbitrary plugin-specific options forwarded verbatim to the WASM guest
    /// as a JSON string on each generic hook request.
    pub options: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatterConfig {
    pub indent_size: usize,
    pub use_tabs: bool,
    pub print_width: usize,
    pub bracket_same_line: bool,
    pub wrap_attributes: bool,
    pub single_quotes: bool,
    pub wrap_content: bool,
    pub inline_short_elements_max_len: usize,
    /// Preferred plugin list for the formatter host.
    pub plugins: Vec<PluginConfig>,
    /// Legacy single-plugin configuration kept for backward compatibility.
    /// Optional WASM plugin configuration.
    pub plugin: PluginConfig,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 2,
            use_tabs: false,
            print_width: 80,
            bracket_same_line: false,
            wrap_attributes: false,
            single_quotes: false,
            wrap_content: false,
            inline_short_elements_max_len: 80,
            plugins: Vec::new(),
            plugin: PluginConfig::default(),
        }
    }
}

impl FormatterConfig {
    pub fn plugin_configs(&self) -> Vec<PluginConfig> {
        let mut plugins = self.plugins.clone();
        if self.plugin.path.is_some() {
            plugins.push(self.plugin.clone());
        }
        plugins
    }
}

```

# FILE: crates/core/src/engine.rs
```rust
use crate::config::{FormatterConfig, PluginConfig};
use crate::formatter::Formatter;
use crate::parser::Parser;
use crate::plugins::PluginHost;
use crate::syntax::SyntaxNode;

pub struct FormatEngine {
    config: FormatterConfig,
    plugin_host: PluginHost,
}

impl FormatEngine {
    pub fn new(config: FormatterConfig) -> Self {
        Self {
            config,
            plugin_host: PluginHost::new(),
        }
    }

    pub fn load_plugin(
        &mut self,
        path: &str,
        plugin_options: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.plugin_host.load_wasm(path, plugin_options)
    }

    pub fn load_plugin_config(
        &mut self,
        plugin: &PluginConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let Some(path) = plugin.path.as_deref() else {
            return Ok(());
        };

        self.load_plugin(path, plugin.options.as_ref().map(|value| value.to_string()))
    }

    pub fn format(self, input: &str) -> String {
        let syntax_root = parse_syntax_tree(input);
        self.into_formatter().format(&syntax_root)
    }

    fn into_formatter(self) -> Formatter {
        Formatter::with_plugin_host(self.config, self.plugin_host)
    }
}

fn parse_syntax_tree(input: &str) -> SyntaxNode {
    let parser = Parser::new(input);
    SyntaxNode::new_root(parser.parse())
}

```

# FILE: crates/core/src/formatter.rs
```rust
mod content;
mod context;
mod hooks;
mod output;
mod tags;
mod traversal;

use crate::config::FormatterConfig;
use crate::plugins::PluginHost;
use crate::syntax::SyntaxNode;

pub struct Formatter {
    config: FormatterConfig,
    plugin_host: PluginHost,
}

struct FormatSession {
    config: FormatterConfig,
    plugin_host: PluginHost,
    output: String,
    current_indent: usize,
}

impl Formatter {
    pub fn new(config: FormatterConfig) -> Self {
        Self::with_plugin_host(config, PluginHost::new())
    }

    pub fn with_plugin_host(config: FormatterConfig, plugin_host: PluginHost) -> Self {
        Self {
            config,
            plugin_host,
        }
    }

    pub fn format(self, root: &SyntaxNode) -> String {
        FormatSession::new(self.config, self.plugin_host).format(root)
    }
}

impl FormatSession {
    fn new(config: FormatterConfig, plugin_host: PluginHost) -> Self {
        Self {
            config,
            plugin_host,
            output: String::new(),
            current_indent: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::plugins::{FormatPlugin, PluginHost};
    use crate::syntax::SyntaxNode;
    use fua_plugin_api::{HookRequest, HookResponse, Replacement};

    #[test]
    fn formats_plain_html_without_plugins() {
        let input = "<div id=\"app\">   <p>Hello <span>world</span></p></div>";

        let parser = Parser::new(input);
        let green_node = parser.parse();
        let syntax_node = SyntaxNode::new_root(green_node);

        let formatter = Formatter::new(FormatterConfig::default());
        let output = formatter.format(&syntax_node);

        let expected = "\n<div id=\"app\">\n  <p>Hello <span>world</span>\n  </p>\n</div>";
        assert_eq!(output, expected);
    }

    struct UppercaseTextPlugin;

    impl FormatPlugin for UppercaseTextPlugin {
        fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse {
            match request {
                HookRequest::Token(token) if token.kind == "IDENT" && token.text == "Hello" => {
                    HookResponse::replace(Replacement::text("HELLO"))
                }
                _ => HookResponse::Continue,
            }
        }
    }

    #[test]
    fn plugin_hooks_only_apply_when_a_plugin_is_registered() {
        let input = "<p>Hello</p>";
        let parser = Parser::new(input);
        let syntax_node = SyntaxNode::new_root(parser.parse());

        let without_plugins = Formatter::new(FormatterConfig::default()).format(&syntax_node);
        assert_eq!(without_plugins, "\n<p>Hello\n</p>");

        let mut plugin_host = PluginHost::new();
        plugin_host.register(Box::new(UppercaseTextPlugin));
        let with_plugins = Formatter::with_plugin_host(FormatterConfig::default(), plugin_host)
            .format(&syntax_node);

        assert_eq!(with_plugins, "\n<p>HELLO\n</p>");
    }

    struct AttributeValuePlugin;

    impl FormatPlugin for AttributeValuePlugin {
        fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse {
            match request {
                HookRequest::Token(token)
                    if token.kind == "STRING_DOUBLE"
                        && token.context.attribute_name.as_deref() == Some("data-role") =>
                {
                    HookResponse::replace(Replacement::text("\"widget\""))
                }
                _ => HookResponse::Continue,
            }
        }
    }

    #[test]
    fn attribute_value_hooks_receive_the_current_attribute_name() {
        let input = r#"<div id="app" data-role="card"></div>"#;
        let parser = Parser::new(input);
        let syntax_node = SyntaxNode::new_root(parser.parse());

        let mut plugin_host = PluginHost::new();
        plugin_host.register(Box::new(AttributeValuePlugin));
        let output = Formatter::with_plugin_host(FormatterConfig::default(), plugin_host)
            .format(&syntax_node);

        assert_eq!(output, "\n<div id=\"app\" data-role=\"widget\">\n</div>");
    }
}

```

# FILE: crates/core/src/lexer.rs
```rust
use logos::{Lexer, Logos};

fn lex_comment<'a>(lex: &mut Lexer<'a, Token>) -> Option<()> {
    let remainder = lex.remainder();
    if let Some(pos) = remainder.find("-->") {
        lex.bump(pos + 3);
        Some(())
    } else {
        // Permissive: if comment is not closed, consume to EOF
        lex.bump(remainder.len());
        Some(())
    }
}

// In Logos 0.14, the error token is replaced by returning Result<Token, ()> or custom error.
// We'll define a simple Error type or just use default.
#[derive(Logos, Debug, PartialEq, Clone)]
// #[logos(skip "")] // Do not skip anything to remain lossless
pub enum Token {
    #[regex(r"[ \t\n\r\f]+")]
    Whitespace,

    #[token("<!--", lex_comment)]
    Comment,

    #[token("<")]
    OpenAngle,

    #[token(">")]
    CloseAngle,

    #[token("</")]
    OpenAngleSlash,

    #[token("/>")]
    SlashCloseAngle,

    #[token("=")]
    Equals,

    /// Permissive identifiers for tag names, attribute names, and common
    /// template-extension sigils without hard-coding any framework semantics.
    #[regex(r"[a-zA-Z0-9_\-\*\[\]\(\)\@\:\$\#\.]+")]
    Ident,

    #[regex(r#""[^"]*""#)]
    StringDouble,

    #[regex(r"'[^']*'")]
    StringSingle,

    /// Permissive text nodes: capture any sequence of characters not handled by other tokens.
    /// This includes punctuation, non-ascii characters, etc.
    #[regex(r#"[^a-zA-Z0-9_\-\*\[\]\(\)\@\:\$\#\.<>= \t\n\r\f"']+"#)]
    Text,

    /// Fallback for unbalanced quotes or other strict errors
    #[regex(r#"["']"#)]
    UnclosedQuote,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<(Token, &str)> {
        let mut lexer = Token::lexer(input);
        let mut tokens = Vec::new();
        while let Some(res) = lexer.next() {
            let token = res.expect("Should not return error with permissive lexer");
            tokens.push((token, lexer.slice()));
        }
        tokens
    }

    #[test]
    fn test_standard_html() {
        let input = r#"<div id="main">hello</div>"#;
        let tokens = lex(input);

        assert_eq!(
            tokens,
            vec![
                (Token::OpenAngle, "<"),
                (Token::Ident, "div"),
                (Token::Whitespace, " "),
                (Token::Ident, "id"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"main\""),
                (Token::CloseAngle, ">"),
                (Token::Ident, "hello"),
                (Token::OpenAngleSlash, "</"),
                (Token::Ident, "div"),
                (Token::CloseAngle, ">"),
            ]
        );
    }

    #[test]
    fn test_extension_attribute_syntax() {
        let input = r#"<button @event="doIt" *show="visible" [(model)]="val" :disabled="true" />"#;
        let tokens = lex(input);

        assert_eq!(
            tokens,
            vec![
                (Token::OpenAngle, "<"),
                (Token::Ident, "button"),
                (Token::Whitespace, " "),
                (Token::Ident, "@event"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"doIt\""),
                (Token::Whitespace, " "),
                (Token::Ident, "*show"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"visible\""),
                (Token::Whitespace, " "),
                (Token::Ident, "[(model)]"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"val\""),
                (Token::Whitespace, " "),
                (Token::Ident, ":disabled"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"true\""),
                (Token::Whitespace, " "),
                (Token::SlashCloseAngle, "/>"),
            ]
        );
    }

    #[test]
    fn test_lossless_properties() {
        let input = "<!-- some comment -->\n <p>text</p>";
        let tokens = lex(input);

        assert_eq!(
            tokens,
            vec![
                (Token::Comment, "<!-- some comment -->"),
                (Token::Whitespace, "\n "),
                (Token::OpenAngle, "<"),
                (Token::Ident, "p"),
                (Token::CloseAngle, ">"),
                (Token::Ident, "text"),
                (Token::OpenAngleSlash, "</"),
                (Token::Ident, "p"),
                (Token::CloseAngle, ">"),
            ]
        );

        // Ensure total length matches input length
        let mut total_len = 0;
        for (_, slice) in &tokens {
            total_len += slice.len();
        }
        assert_eq!(total_len, input.len());
    }

    #[test]
    fn test_lossless_unclosed_comment() {
        let input = "<!-- unclosed";
        let tokens = lex(input);

        assert_eq!(tokens, vec![(Token::Comment, "<!-- unclosed")]);
    }

    #[test]
    fn test_raw_text() {
        let input = "hello, world! 你好!";
        let tokens = lex(input);

        assert_eq!(
            tokens,
            vec![
                (Token::Ident, "hello"),
                (Token::Text, ","),
                (Token::Whitespace, " "),
                (Token::Ident, "world"),
                (Token::Text, "!"),
                (Token::Whitespace, " "),
                (Token::Text, "你好!"),
            ]
        );
    }
}

```

# FILE: crates/core/src/lib.rs
```rust
#[cfg(test)]
mod architecture_tests;
pub mod config;
pub mod engine;
pub mod formatter;
pub mod lexer;
pub mod parser;
pub mod plugins;
pub mod syntax;

```

# FILE: crates/core/src/parser.rs
```rust
use logos::{Logos, SpannedIter};
use rowan::{GreenNode, GreenNodeBuilder};
use std::iter::Peekable;

use crate::lexer::Token;
use crate::syntax::SyntaxKind;

fn token_to_kind(token: &Token) -> SyntaxKind {
    match token {
        Token::Whitespace => SyntaxKind::WHITESPACE,
        Token::Comment => SyntaxKind::COMMENT,
        Token::OpenAngle => SyntaxKind::OPEN_ANGLE,
        Token::CloseAngle => SyntaxKind::CLOSE_ANGLE,
        Token::OpenAngleSlash => SyntaxKind::OPEN_ANGLE_SLASH,
        Token::SlashCloseAngle => SyntaxKind::SLASH_CLOSE_ANGLE,
        Token::Equals => SyntaxKind::EQUALS,
        Token::Ident => SyntaxKind::IDENT,
        Token::StringDouble => SyntaxKind::STRING_DOUBLE,
        Token::StringSingle => SyntaxKind::STRING_SINGLE,
        Token::Text => SyntaxKind::TEXT,
        Token::UnclosedQuote => SyntaxKind::UNCLOSED_QUOTE,
    }
}

pub struct Parser<'a> {
    source: &'a str,
    lexer: Peekable<SpannedIter<'a, Token>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Token::lexer(source).spanned().peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    /// Consumes the next token and adds it to the builder
    fn bump(&mut self) {
        if let Some((res, span)) = self.lexer.next() {
            let token = res.unwrap_or(Token::Text);
            let kind = token_to_kind(&token);
            let text = &self.source[span];
            self.builder.token(kind.into(), text);
        }
    }

    pub fn parse(mut self) -> GreenNode {
        self.builder.start_node(SyntaxKind::ROOT.into());

        while self.lexer.peek().is_some() {
            self.parse_node();
        }

        self.builder.finish_node();
        self.builder.finish()
    }

    fn peek_token(&mut self) -> Option<Token> {
        self.lexer
            .peek()
            .map(|(res, _)| res.as_ref().unwrap_or(&Token::Text).clone())
    }

    fn parse_node(&mut self) {
        match self.peek_token() {
            Some(Token::OpenAngle) => self.parse_element(),
            Some(Token::OpenAngleSlash) => {
                self.parse_tag(SyntaxKind::CLOSE_TAG);
            }
            Some(_) => self.bump(),
            None => {}
        }
    }

    fn parse_element(&mut self) {
        self.builder.start_node(SyntaxKind::ELEMENT.into());

        // Peek tag name to identify HTML void elements
        let mut is_void = false;
        let mut peek_iter = self.lexer.clone();
        peek_iter.next(); // skip `<`
        if let Some((Ok(Token::Ident), span)) = peek_iter.next() {
            let tag_name = &self.source[span];
            let lower = tag_name.to_lowercase();
            is_void = matches!(
                lower.as_str(),
                "area"
                    | "base"
                    | "br"
                    | "col"
                    | "embed"
                    | "hr"
                    | "img"
                    | "input"
                    | "link"
                    | "meta"
                    | "source"
                    | "track"
                    | "wbr"
            );
        }

        // Parse open tag
        let is_self_closing = self.parse_tag(SyntaxKind::OPEN_TAG);

        if !is_self_closing && !is_void {
            // Parse children recursively
            while let Some(token) = self.peek_token() {
                if token == Token::OpenAngleSlash {
                    break;
                }
                self.parse_node();
            }

            // Parse close tag if it exists
            if matches!(self.peek_token(), Some(Token::OpenAngleSlash)) {
                self.parse_tag(SyntaxKind::CLOSE_TAG);
            }
        }

        self.builder.finish_node();
    }

    fn parse_tag(&mut self, kind: SyntaxKind) -> bool {
        self.builder.start_node(kind.into());
        let mut is_self_closing = false;

        // bump `<` or `</`
        self.bump();

        // bump everything until `>` or `/>`
        while let Some(token) = self.peek_token() {
            let is_close = token == Token::CloseAngle || token == Token::SlashCloseAngle;

            if token == Token::SlashCloseAngle {
                is_self_closing = true;
            }

            self.bump();

            if is_close {
                break;
            }
        }

        self.builder.finish_node();
        is_self_closing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::SyntaxNode;

    #[test]
    fn test_lossless_parser() {
        let input = r#"
            <!-- Component wrapper -->
            <div id="app" *show="visible" @event="handle">
                hello world! 
                <button [(model)]="value" :disabled="false" />
                <Closing / > </ Closing >
            </div>
        "#;

        let parser = Parser::new(input);
        let green_node = parser.parse();
        let syntax_node = SyntaxNode::new_root(green_node);

        let reconstructed = syntax_node.to_string();

        assert_eq!(
            input, reconstructed,
            "The rebuilt source text must be byte-for-byte identical to the original input."
        );
    }

    #[test]
    fn test_nested_elements_and_lossless() {
        let input = "<div>\n  <p>Hello, <span>world</span>!</p>\n</div>";
        let parser = Parser::new(input);
        let green_node = parser.parse();
        let syntax_node = SyntaxNode::new_root(green_node);

        // Print tree so we can verify visually that ELEMENT nodes are nested correctly
        println!("{:#?}", syntax_node);

        // The Ultimate Check
        let reconstructed = syntax_node.to_string();
        assert_eq!(
            input, reconstructed,
            "The rebuilt source text must be byte-for-byte identical to the original input."
        );
    }
}

```

# FILE: crates/core/src/plugins.rs
```rust
use std::borrow::Cow;

use extism::{Manifest, Plugin, Wasm};
use fua_plugin_api::{HANDLE_HOOK_EXPORT, HookRequest, HookResponse, Replacement};

pub trait FormatPlugin: Send {
    fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse;
}

struct WasmPluginInstance {
    plugin: Plugin,
    plugin_options: Option<String>,
}

impl WasmPluginInstance {
    fn new(path: &str, plugin_options: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let manifest = Manifest::new([Wasm::file(path)]);
        let plugin = Plugin::new(&manifest, [], true)?;
        Ok(Self {
            plugin,
            plugin_options,
        })
    }
}

impl FormatPlugin for WasmPluginInstance {
    fn handle_hook(&mut self, request: &HookRequest<'_>) -> HookResponse {
        let request = request
            .clone()
            .with_plugin_options(self.plugin_options.as_deref().map(Cow::Borrowed));

        let input = match serde_json::to_string(&request) {
            Ok(input) => input,
            Err(_) => return HookResponse::Continue,
        };

        let response: String = match self.plugin.call(HANDLE_HOOK_EXPORT, input) {
            Ok(response) => response,
            Err(_) => return HookResponse::Continue,
        };

        if response.trim().is_empty() {
            return HookResponse::Continue;
        }

        serde_json::from_str(&response).unwrap_or(HookResponse::Continue)
    }
}

#[derive(Default)]
pub struct PluginHost {
    plugins: Vec<Box<dyn FormatPlugin>>,
}

impl PluginHost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    pub fn register(&mut self, plugin: Box<dyn FormatPlugin>) {
        self.plugins.push(plugin);
    }

    pub fn load_wasm(
        &mut self,
        path: &str,
        plugin_options: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let plugin = WasmPluginInstance::new(path, plugin_options)?;
        self.register(Box::new(plugin));
        Ok(())
    }

    pub fn dispatch(&mut self, request: &HookRequest<'_>) -> Option<Replacement> {
        for plugin in &mut self.plugins {
            match plugin.handle_hook(request) {
                HookResponse::Continue => {}
                HookResponse::Replace(replacement) => return Some(replacement),
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fua_plugin_api::{HookContext, LeadingSpacing};

    struct PassThroughPlugin;

    impl FormatPlugin for PassThroughPlugin {
        fn handle_hook(&mut self, _request: &HookRequest<'_>) -> HookResponse {
            HookResponse::Continue
        }
    }

    struct ReplacingPlugin;

    impl FormatPlugin for ReplacingPlugin {
        fn handle_hook(&mut self, _request: &HookRequest<'_>) -> HookResponse {
            HookResponse::replace(Replacement::text("handled").with_leading(LeadingSpacing::Space))
        }
    }

    #[test]
    fn dispatch_returns_the_first_replacement() {
        let context = HookContext::new("ROOT", None, None, 0, 2, false);
        let request = HookRequest::token("TEXT", "hello", context);

        let mut host = PluginHost::new();
        host.register(Box::new(PassThroughPlugin));
        host.register(Box::new(ReplacingPlugin));

        let replacement = host.dispatch(&request).expect("replacement");
        assert_eq!(replacement.output, "handled");
        assert_eq!(replacement.leading_spacing, LeadingSpacing::Space);
    }
}

```

# FILE: crates/core/src/syntax.rs
```rust
use num_derive::{FromPrimitive, ToPrimitive};
use rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    // Lexer tokens (matching `lexer::Token` variants)
    WHITESPACE = 0,
    COMMENT,
    OPEN_ANGLE,
    CLOSE_ANGLE,
    OPEN_ANGLE_SLASH,
    SLASH_CLOSE_ANGLE,
    EQUALS,
    IDENT,
    STRING_DOUBLE,
    STRING_SINGLE,
    TEXT,
    UNCLOSED_QUOTE,

    // Structural concepts (Parser nodes)
    ROOT,
    ELEMENT,
    OPEN_TAG,
    CLOSE_TAG,
    SELF_CLOSING_TAG,
    ATTRIBUTE,

    // Catch-all for rowan
    ERROR,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HtmlLang;

impl Language for HtmlLang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        num_traits::FromPrimitive::from_u16(raw.0).unwrap_or(SyntaxKind::ERROR)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<HtmlLang>;
pub type SyntaxToken = rowan::SyntaxToken<HtmlLang>;
pub type SyntaxElement = rowan::SyntaxElement<HtmlLang>;

```

# FILE: crates/core/src/formatter/content.rs
```rust
use super::FormatSession;
use super::context::{is_after_open_tag, is_before_close_tag, is_compact_element};
use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

impl FormatSession {
    pub(super) fn format_content_token(
        &mut self,
        parent: &SyntaxNode,
        element: &SyntaxElement,
        token: &SyntaxToken,
        inline_mode: bool,
    ) {
        if self.try_handle_token(parent, token, None) {
            return;
        }

        match token.kind() {
            SyntaxKind::WHITESPACE => {
                self.format_content_whitespace(element, token.text(), inline_mode)
            }
            SyntaxKind::COMMENT => self.format_comment(token.text()),
            SyntaxKind::IDENT => self.format_ident(parent, element, token.text(), inline_mode),
            SyntaxKind::TEXT => self.format_text(parent, element, token.text(), inline_mode),
            _ => self.output.push_str(token.text()),
        }
    }

    fn format_content_whitespace(
        &mut self,
        element: &SyntaxElement,
        text: &str,
        inline_mode: bool,
    ) {
        let newline_count = text.matches('\n').count();
        let next = element.next_sibling_or_token();

        if self.should_omit_content_whitespace(next.as_ref()) {
            self.preserve_blank_line_gap(newline_count);
            return;
        }

        if self.should_wrap_content_whitespace(element, text, inline_mode) {
            self.push_content_break(newline_count);
            return;
        }

        if inline_mode {
            self.write_inline_content_spacing(element);
            return;
        }

        self.ensure_space();
    }

    fn should_omit_content_whitespace(&self, next: Option<&SyntaxElement>) -> bool {
        next.is_some_and(|sibling| {
            sibling.kind() == SyntaxKind::CLOSE_TAG
                || sibling.as_node().is_some_and(|node| {
                    node.kind() == SyntaxKind::ELEMENT
                        && !is_compact_element(node, self.config.inline_short_elements_max_len)
                })
        })
    }

    fn should_wrap_content_whitespace(
        &self,
        element: &SyntaxElement,
        text: &str,
        inline_mode: bool,
    ) -> bool {
        (text.contains('\n') || (is_after_open_tag(element) && self.config.wrap_content))
            && !inline_mode
    }

    fn preserve_blank_line_gap(&mut self, newline_count: usize) {
        if newline_count > 1 {
            self.push_newlines_with_indent(2);
        }
    }

    fn push_content_break(&mut self, newline_count: usize) {
        let break_count = if newline_count > 1 { 2 } else { 1 };
        self.push_newlines_with_indent(break_count);
    }

    fn write_inline_content_spacing(&mut self, element: &SyntaxElement) {
        if !is_after_open_tag(element) && !is_before_close_tag(element) {
            self.ensure_space();
        }
    }

    fn format_comment(&mut self, text: &str) {
        self.push_newlines_with_indent(1);
        self.output.push_str(text);
    }

    fn format_ident(
        &mut self,
        parent: &SyntaxNode,
        element: &SyntaxElement,
        text: &str,
        inline_mode: bool,
    ) {
        if self.should_wrap_text_after_open_tag(parent, element, inline_mode) {
            self.push_newlines_with_indent(1);
        }

        self.output.push_str(text);
    }

    fn format_text(
        &mut self,
        parent: &SyntaxNode,
        element: &SyntaxElement,
        text: &str,
        inline_mode: bool,
    ) {
        let mut collapsed = if inline_mode {
            collapse_inline_text(
                text,
                is_after_open_tag(element),
                is_before_close_tag(element),
            )
        } else {
            text.replace("  ", " ")
        };

        if self.should_wrap_text_after_open_tag(parent, element, inline_mode)
            && !collapsed.trim().is_empty()
        {
            self.output.push('\n');
            self.push_current_indent();
            collapsed = collapsed.trim_start().to_string();
        }

        self.output.push_str(&collapsed);
    }

    fn should_wrap_text_after_open_tag(
        &self,
        parent: &SyntaxNode,
        element: &SyntaxElement,
        inline_mode: bool,
    ) -> bool {
        parent.kind() == SyntaxKind::ELEMENT
            && is_after_open_tag(element)
            && self.config.wrap_content
            && !inline_mode
    }

    pub(super) fn format_double_quoted_string(&mut self, text: &str) {
        if self.config.single_quotes && text.len() >= 2 {
            let inner = &text[1..text.len() - 1];
            let escaped = inner.replace('\'', "&apos;");
            self.output.push('\'');
            self.output.push_str(&escaped);
            self.output.push('\'');
        } else {
            self.output.push_str(text);
        }
    }

    pub(super) fn format_single_quoted_string(&mut self, text: &str) {
        if !self.config.single_quotes && text.len() >= 2 {
            let inner = &text[1..text.len() - 1];
            let escaped = inner.replace('"', "&quot;");
            self.output.push('"');
            self.output.push_str(&escaped);
            self.output.push('"');
        } else {
            self.output.push_str(text);
        }
    }
}

fn collapse_inline_text(text: &str, trim_start: bool, trim_end: bool) -> String {
    let mut collapsed = text.replace('\r', "").replace(['\n', '\t'], " ");

    while collapsed.contains("  ") {
        collapsed = collapsed.replace("  ", " ");
    }

    if trim_start {
        collapsed = collapsed.trim_start().to_string();
    }

    if trim_end {
        collapsed = collapsed.trim_end().to_string();
    }

    collapsed
}

```

# FILE: crates/core/src/formatter/context.rs
```rust
use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxNode};
use rowan::NodeOrToken;

pub(super) const ROOT_PARENT_KIND: &str = "NONE";

pub(super) fn node_tag_name(node: &SyntaxNode) -> Option<String> {
    match node.kind() {
        SyntaxKind::ELEMENT => element_tag_name(node),
        SyntaxKind::OPEN_TAG | SyntaxKind::CLOSE_TAG | SyntaxKind::SELF_CLOSING_TAG => {
            tag_node_name(node)
        }
        _ => None,
    }
}

pub(super) fn syntax_kind_label(kind: SyntaxKind) -> &'static str {
    match kind {
        SyntaxKind::WHITESPACE => "WHITESPACE",
        SyntaxKind::COMMENT => "COMMENT",
        SyntaxKind::OPEN_ANGLE => "OPEN_ANGLE",
        SyntaxKind::CLOSE_ANGLE => "CLOSE_ANGLE",
        SyntaxKind::OPEN_ANGLE_SLASH => "OPEN_ANGLE_SLASH",
        SyntaxKind::SLASH_CLOSE_ANGLE => "SLASH_CLOSE_ANGLE",
        SyntaxKind::EQUALS => "EQUALS",
        SyntaxKind::IDENT => "IDENT",
        SyntaxKind::STRING_DOUBLE => "STRING_DOUBLE",
        SyntaxKind::STRING_SINGLE => "STRING_SINGLE",
        SyntaxKind::TEXT => "TEXT",
        SyntaxKind::UNCLOSED_QUOTE => "UNCLOSED_QUOTE",
        SyntaxKind::ROOT => "ROOT",
        SyntaxKind::ELEMENT => "ELEMENT",
        SyntaxKind::OPEN_TAG => "OPEN_TAG",
        SyntaxKind::CLOSE_TAG => "CLOSE_TAG",
        SyntaxKind::SELF_CLOSING_TAG => "SELF_CLOSING_TAG",
        SyntaxKind::ATTRIBUTE => "ATTRIBUTE",
        SyntaxKind::ERROR => "ERROR",
    }
}

pub(super) fn can_inline_element(node: &SyntaxNode) -> bool {
    node_tag_name(node)
        .as_deref()
        .map(|tag_name| !is_block_element(tag_name))
        .unwrap_or(false)
}

pub(super) fn is_compact_element(node: &SyntaxNode, max_len: usize) -> bool {
    let has_complex_child = node.children_with_tokens().any(|element| {
        element.kind() == SyntaxKind::ELEMENT || element.kind() == SyntaxKind::COMMENT
    });
    if has_complex_child {
        return false;
    }

    let total_len: usize = node
        .descendants_with_tokens()
        .filter_map(|element| match element {
            NodeOrToken::Token(token) if token.kind() != SyntaxKind::WHITESPACE => {
                Some(token.text().len())
            }
            _ => None,
        })
        .sum();

    total_len <= max_len
}

pub(super) fn is_after_open_tag(element: &SyntaxElement) -> bool {
    previous_non_whitespace(element)
        .as_ref()
        .is_some_and(|sibling| sibling.kind() == SyntaxKind::OPEN_TAG)
}

pub(super) fn is_before_close_tag(element: &SyntaxElement) -> bool {
    next_non_whitespace(element)
        .as_ref()
        .is_some_and(|sibling| sibling.kind() == SyntaxKind::CLOSE_TAG)
}

pub(super) fn previous_non_whitespace(element: &SyntaxElement) -> Option<SyntaxElement> {
    let mut current = element.prev_sibling_or_token();
    while let Some(sibling) = current.clone() {
        if sibling.kind() == SyntaxKind::WHITESPACE {
            current = sibling.prev_sibling_or_token();
        } else {
            break;
        }
    }
    current
}

pub(super) fn next_non_whitespace(element: &SyntaxElement) -> Option<SyntaxElement> {
    let mut current = element.next_sibling_or_token();
    while let Some(sibling) = current.clone() {
        if sibling.kind() == SyntaxKind::WHITESPACE {
            current = sibling.next_sibling_or_token();
        } else {
            break;
        }
    }
    current
}

pub(super) fn token_text(element: &SyntaxElement) -> Option<&str> {
    match element {
        NodeOrToken::Node(_) => None,
        NodeOrToken::Token(token) => Some(token.text()),
    }
}

fn element_tag_name(node: &SyntaxNode) -> Option<String> {
    for element in node.children_with_tokens() {
        if let NodeOrToken::Node(tag) = element
            && matches!(
                tag.kind(),
                SyntaxKind::OPEN_TAG | SyntaxKind::SELF_CLOSING_TAG
            )
        {
            return tag_node_name(&tag);
        }
    }

    None
}

fn tag_node_name(node: &SyntaxNode) -> Option<String> {
    node.children_with_tokens()
        .find_map(|element| match element {
            NodeOrToken::Token(token) if token.kind() == SyntaxKind::IDENT => {
                Some(token.text().to_string())
            }
            _ => None,
        })
}

pub(super) fn is_block_element(tag_name: &str) -> bool {
    !matches!(
        tag_name,
        "a" | "abbr"
            | "b"
            | "code"
            | "em"
            | "i"
            | "img"
            | "input"
            | "label"
            | "small"
            | "span"
            | "strong"
            | "sub"
            | "sup"
            | "textarea"
    )
}

```

# FILE: crates/core/src/formatter/hooks.rs
```rust
use super::FormatSession;
use super::context::{ROOT_PARENT_KIND, node_tag_name, syntax_kind_label, token_text};
use crate::syntax::{SyntaxNode, SyntaxToken};
use fua_plugin_api::{HookContext, HookRequest, LeadingSpacing, NodePhase, Replacement};

impl FormatSession {
    pub(super) fn try_handle_node(&mut self, node: &SyntaxNode, phase: NodePhase) -> bool {
        if self.plugin_host.is_empty() {
            return false;
        }

        let tag_name = node_tag_name(node);
        let text = node.to_string();
        let request = HookRequest::node(
            phase,
            syntax_kind_label(node.kind()),
            &text,
            tag_name.as_deref(),
            self.build_node_context(node, tag_name.as_deref()),
        );

        if let Some(replacement) = self.plugin_host.dispatch(&request) {
            self.write_replacement(replacement);
            return true;
        }

        false
    }

    pub(super) fn try_handle_token(
        &mut self,
        parent: &SyntaxNode,
        token: &SyntaxToken,
        attribute_name: Option<&str>,
    ) -> bool {
        if self.plugin_host.is_empty() {
            return false;
        }

        let tag_name = node_tag_name(parent);
        let previous = token.prev_sibling_or_token();
        let next = token.next_sibling_or_token();
        let request = HookRequest::token(
            syntax_kind_label(token.kind()),
            token.text(),
            HookContext::new(
                syntax_kind_label(parent.kind()),
                tag_name.as_deref(),
                attribute_name,
                self.current_indent,
                self.config.indent_size,
                self.config.use_tabs,
            )
            .with_neighbors(
                previous
                    .as_ref()
                    .map(|element| syntax_kind_label(element.kind())),
                previous.as_ref().and_then(token_text),
                next.as_ref()
                    .map(|element| syntax_kind_label(element.kind())),
                next.as_ref().and_then(token_text),
            ),
        );

        if let Some(replacement) = self.plugin_host.dispatch(&request) {
            self.write_replacement(replacement);
            return true;
        }

        false
    }

    fn build_node_context<'a>(
        &self,
        node: &SyntaxNode,
        tag_name: Option<&'a str>,
    ) -> HookContext<'a> {
        let parent_kind = node
            .parent()
            .map(|parent| syntax_kind_label(parent.kind()))
            .unwrap_or(ROOT_PARENT_KIND);

        HookContext::new(
            parent_kind,
            tag_name,
            None,
            self.current_indent,
            self.config.indent_size,
            self.config.use_tabs,
        )
    }

    fn write_replacement(&mut self, replacement: Replacement) {
        self.adjust_indent(replacement.indent_before);
        self.apply_leading_spacing(replacement.leading_spacing);
        self.output.push_str(&replacement.output);
        self.adjust_indent(replacement.indent_after);
    }

    fn apply_leading_spacing(&mut self, spacing: LeadingSpacing) {
        match spacing {
            LeadingSpacing::None => {}
            LeadingSpacing::Space => self.ensure_space(),
            LeadingSpacing::LineBreak => self.push_newlines_with_indent(1),
            LeadingSpacing::BlankLine => self.push_newlines_with_indent(2),
        }
    }
}

```

# FILE: crates/core/src/formatter/output.rs
```rust
use super::FormatSession;

impl FormatSession {
    pub(super) fn finish(self) -> String {
        self.output
    }

    pub(super) fn adjust_indent(&mut self, delta: i32) {
        if delta < 0 {
            self.current_indent = self.current_indent.saturating_sub((-delta) as usize);
        } else {
            self.current_indent += delta as usize;
        }
    }

    pub(super) fn push_current_indent(&mut self) {
        self.push_indent(self.current_indent);
    }

    fn push_indent(&mut self, depth: usize) {
        if self.config.use_tabs {
            self.output.push_str(&"\t".repeat(depth));
        } else {
            self.output
                .push_str(&" ".repeat(depth * self.config.indent_size));
        }
    }

    pub(super) fn ensure_space(&mut self) {
        if self
            .output
            .chars()
            .next_back()
            .is_some_and(|ch| ch == ' ' || ch == '\t' || ch == '\n')
        {
            return;
        }

        self.output.push(' ');
    }

    pub(super) fn push_single_indent_unit(&mut self) {
        self.push_indent(1);
    }

    pub(super) fn push_newlines_with_indent(&mut self, count: usize) {
        trim_trailing_horizontal_whitespace(&mut self.output);

        let trailing_newlines = self
            .output
            .chars()
            .rev()
            .take_while(|ch| *ch == '\n')
            .count();
        for _ in trailing_newlines..count {
            self.output.push('\n');
        }

        self.push_current_indent();
    }
}

fn trim_trailing_horizontal_whitespace(output: &mut String) {
    let mut trim_len = output.len();
    for ch in output.chars().rev() {
        if ch == ' ' || ch == '\t' {
            trim_len -= ch.len_utf8();
        } else {
            break;
        }
    }
    output.truncate(trim_len);
}

```

# FILE: crates/core/src/formatter/tags.rs
```rust
use super::FormatSession;
use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

#[derive(Default)]
struct TagState {
    saw_tag_name: bool,
    current_attribute_name: Option<String>,
}

impl TagState {
    fn attribute_name(&self) -> Option<&str> {
        self.current_attribute_name.as_deref()
    }

    fn observe_ident(&mut self, node_kind: SyntaxKind, text: &str) {
        if !matches!(
            node_kind,
            SyntaxKind::OPEN_TAG | SyntaxKind::SELF_CLOSING_TAG
        ) {
            return;
        }

        if self.saw_tag_name {
            self.current_attribute_name = Some(text.to_string());
        } else {
            self.saw_tag_name = true;
        }
    }
}

impl FormatSession {
    pub(super) fn format_open_tag(&mut self, node: &SyntaxNode, inline_mode: bool) {
        self.format_tag(node, inline_mode);
    }

    pub(super) fn format_close_tag(&mut self, node: &SyntaxNode, inline_mode: bool) {
        self.format_tag(node, inline_mode);
    }

    pub(super) fn format_self_closing_tag(&mut self, node: &SyntaxNode, inline_mode: bool) {
        self.format_tag(node, inline_mode);
    }

    fn format_tag(&mut self, node: &SyntaxNode, inline_mode: bool) {
        let mut state = TagState::default();

        for element in node.children_with_tokens() {
            let NodeOrToken::Token(token) = &element else {
                continue;
            };

            self.format_tag_token(node, &element, token, inline_mode, &mut state);
        }
    }

    fn format_tag_token(
        &mut self,
        node: &SyntaxNode,
        element: &SyntaxElement,
        token: &SyntaxToken,
        inline_mode: bool,
        state: &mut TagState,
    ) {
        if self.try_handle_token(node, token, state.attribute_name()) {
            return;
        }

        match token.kind() {
            SyntaxKind::WHITESPACE => self.format_attribute_spacing(element, inline_mode),
            SyntaxKind::IDENT => {
                state.observe_ident(node.kind(), token.text());
                self.output.push_str(token.text());
            }
            SyntaxKind::STRING_DOUBLE => self.format_double_quoted_string(token.text()),
            SyntaxKind::STRING_SINGLE => self.format_single_quoted_string(token.text()),
            SyntaxKind::CLOSE_ANGLE | SyntaxKind::SLASH_CLOSE_ANGLE => {
                self.format_tag_closing_bracket(node.kind(), element, token.text(), inline_mode)
            }
            _ => self.output.push_str(token.text()),
        }
    }

    fn format_attribute_spacing(&mut self, element: &SyntaxElement, inline_mode: bool) {
        let next = element.next_sibling_or_token();
        let next_is_bracket = next.as_ref().is_some_and(|sibling| {
            matches!(
                sibling.kind(),
                SyntaxKind::CLOSE_ANGLE | SyntaxKind::SLASH_CLOSE_ANGLE
            )
        });

        if next_is_bracket {
            if self.config.bracket_same_line || inline_mode {
                if next
                    .as_ref()
                    .is_some_and(|sibling| sibling.kind() == SyntaxKind::SLASH_CLOSE_ANGLE)
                {
                    self.ensure_space();
                }
                return;
            }

            if self.config.wrap_attributes {
                self.push_newlines_with_indent(1);
                return;
            }
        }

        if self.config.wrap_attributes {
            if inline_mode {
                self.ensure_space();
            } else {
                self.push_newlines_with_indent(1);
                self.push_single_indent_unit();
            }
            return;
        }

        self.ensure_space();
    }

    fn format_tag_closing_bracket(
        &mut self,
        node_kind: SyntaxKind,
        element: &SyntaxElement,
        text: &str,
        inline_mode: bool,
    ) {
        let prev = element.prev_sibling_or_token();
        let prev_is_whitespace = prev
            .as_ref()
            .is_some_and(|sibling| sibling.kind() == SyntaxKind::WHITESPACE);

        if matches!(
            node_kind,
            SyntaxKind::OPEN_TAG | SyntaxKind::SELF_CLOSING_TAG
        ) && !self.config.bracket_same_line
            && self.config.wrap_attributes
            && !prev_is_whitespace
            && !inline_mode
        {
            self.push_newlines_with_indent(1);
        }

        self.output.push_str(text);
    }
}

```

# FILE: crates/core/src/formatter/traversal.rs
```rust
use super::FormatSession;
use super::context::{can_inline_element, is_block_element, is_compact_element, node_tag_name};
use crate::syntax::{SyntaxKind, SyntaxNode};
use fua_plugin_api::NodePhase;
use rowan::NodeOrToken;

impl FormatSession {
    pub(super) fn format(mut self, root: &SyntaxNode) -> String {
        self.format_node(root, false);
        self.finish()
    }

    fn format_node(&mut self, node: &SyntaxNode, inline_mode: bool) {
        if self.try_handle_node(node, NodePhase::Enter) {
            return;
        }

        match node.kind() {
            SyntaxKind::ROOT => self.format_children(node, inline_mode),
            SyntaxKind::ELEMENT => self.format_element(node, inline_mode),
            SyntaxKind::OPEN_TAG => self.format_open_tag(node, inline_mode),
            SyntaxKind::CLOSE_TAG => self.format_close_tag(node, inline_mode),
            SyntaxKind::SELF_CLOSING_TAG => self.format_self_closing_tag(node, inline_mode),
            _ => self.format_children(node, inline_mode),
        }

        self.try_handle_node(node, NodePhase::Exit);
    }

    fn format_children(&mut self, node: &SyntaxNode, inline_mode: bool) {
        for element in node.children_with_tokens() {
            match element {
                NodeOrToken::Node(child) => self.format_nested_node(&child, inline_mode),
                NodeOrToken::Token(ref token) => {
                    self.format_content_token(node, &element, token, inline_mode)
                }
            }
        }
    }

    fn format_element(&mut self, node: &SyntaxNode, inherited_inline: bool) {
        let inline_mode = self.inline_mode_for(node, inherited_inline);
        let is_block = self.is_block_element_node(node);
        let starting_indent = self.current_indent;

        for element in node.children_with_tokens() {
            match element {
                NodeOrToken::Node(child) => match child.kind() {
                    SyntaxKind::OPEN_TAG => {
                        self.format_open_tag(&child, inline_mode);
                        if is_block && !inline_mode {
                            self.current_indent += 1;
                        }
                    }
                    SyntaxKind::CLOSE_TAG => {
                        if is_block && !inline_mode {
                            self.current_indent = self.current_indent.saturating_sub(1);
                            self.push_newlines_with_indent(1);
                        }
                        self.format_close_tag(&child, inline_mode);
                    }
                    SyntaxKind::SELF_CLOSING_TAG => {
                        self.format_self_closing_tag(&child, inline_mode)
                    }
                    _ => self.format_nested_node(&child, inline_mode),
                },
                NodeOrToken::Token(ref token) => {
                    self.format_content_token(node, &element, token, inline_mode)
                }
            }
        }

        self.current_indent = starting_indent;
    }

    fn format_nested_node(&mut self, node: &SyntaxNode, inline_mode: bool) {
        let child_inline = self.inline_mode_for(node, inline_mode);

        if self.should_break_before_child(node, inline_mode, child_inline) {
            self.push_newlines_with_indent(1);
        }

        self.format_node(node, child_inline);
    }

    fn inline_mode_for(&self, node: &SyntaxNode, inherited_inline: bool) -> bool {
        inherited_inline
            || (can_inline_element(node)
                && is_compact_element(node, self.config.inline_short_elements_max_len))
    }

    fn is_block_element_node(&self, node: &SyntaxNode) -> bool {
        node_tag_name(node)
            .as_deref()
            .map(is_block_element)
            .unwrap_or(true)
    }

    fn should_break_before_child(
        &self,
        node: &SyntaxNode,
        inline_mode: bool,
        child_inline: bool,
    ) -> bool {
        matches!(node.kind(), SyntaxKind::ELEMENT)
            && self.is_block_element_node(node)
            && !inline_mode
            && !child_inline
    }
}

```

# FILE: crates/fua-plugin-angular/Cargo.toml
```toml
[package]
name = "fua-plugin-angular"
version = "0.1.0"
edition = "2024"

# cdylib produces the .wasm binary when compiled with --target wasm32-wasip1
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
fua-plugin-api = { path = "../fua-plugin-api" }

[target.'cfg(target_arch = "wasm32")'.dependencies]
extism-pdk = "1"

```

# FILE: crates/fua-plugin-angular/src/attributes.rs
```rust
use crate::context::{is_class_attr, is_ngclass_attr};
use crate::expressions::{
    count_top_level_ops, is_wrappable_condition_expr, split_on_top_level_ops,
    split_top_level_commas, split_trailing_punctuation, unwrap_negated_group,
};
use serde_json::Value;

pub(crate) fn process_attribute_string(
    text: &str,
    attr_name: &str,
    options: &Value,
    current_indent: usize,
    indent_size: usize,
    use_tabs: bool,
) -> Option<String> {
    if text.len() < 2 {
        return None;
    }

    let (quote, inner) = if text.starts_with('"') && text.ends_with('"') {
        ('"', &text[1..text.len() - 1])
    } else if text.starts_with('\'') && text.ends_with('\'') {
        ('\'', &text[1..text.len() - 1])
    } else {
        return None;
    };

    if inner.contains('\n')
        && attr_name.trim().starts_with('[')
        && attr_name.trim().ends_with(']')
        && !is_ngclass_attr(attr_name)
        && !inner.contains('{')
        && !inner.contains('?')
    {
        let compact = inner.split_whitespace().collect::<Vec<_>>().join(" ");
        return Some(format!("{quote}{compact}{quote}"));
    }

    let class_wrap_tokens_min = options
        .get("class_wrap_tokens_min")
        .and_then(Value::as_u64)
        .map(|value| value as usize)
        .unwrap_or(usize::MAX);
    if is_class_attr(attr_name) {
        if let Some(wrapped) = format_class_tokens(
            inner,
            class_wrap_tokens_min,
            current_indent,
            use_tabs,
            indent_size,
        ) {
            return Some(format!("{quote}{wrapped}{quote}"));
        }
    }

    let ngclass_wrap_entries_min = options
        .get("ngclass_wrap_entries_min")
        .and_then(Value::as_u64)
        .map(|value| value as usize)
        .unwrap_or(usize::MAX);
    let force_wrap_ngclass = is_ngclass_attr(attr_name)
        && split_top_level_commas(
            inner
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim(),
        )
        .len()
            >= ngclass_wrap_entries_min;

    if force_wrap_ngclass && !inner.contains('\n') {
        if let Some(wrapped) =
            force_wrap_ngclass_object(inner, current_indent, use_tabs, indent_size)
        {
            return Some(format!("{quote}{wrapped}{quote}"));
        }
    }

    let min_ops = options
        .get("wrap_conditions_min")
        .and_then(Value::as_u64)
        .unwrap_or(2)
        .saturating_sub(1) as usize;
    let wrap_in_parens = options
        .get("wrap_conditions_in_parens")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    if let Some(wrapped) = format_object_literal(
        inner,
        min_ops,
        indent_size,
        use_tabs,
        wrap_in_parens,
        force_wrap_ngclass,
    ) {
        return Some(format!("{quote}{wrapped}{quote}"));
    }

    if options
        .get("wrap_ternary")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        if !inner.contains('\n') {
            if let Some(wrapped) =
                wrap_ternary(inner, min_ops, indent_size, use_tabs, current_indent)
            {
                return Some(format!("{quote}{wrapped}{quote}"));
            }
        }
    }

    if !inner.contains('\n') {
        return None;
    }

    let mut result_lines = Vec::new();
    let mut changed = false;
    for line in inner.split('\n') {
        if changed && line.trim().is_empty() {
            continue;
        }
        match wrap_line_conditions(line, min_ops, indent_size, use_tabs, 0, wrap_in_parens) {
            Some(wrapped) => {
                result_lines.push(wrapped);
                changed = true;
            }
            None => result_lines.push(line.trim_end_matches('\r').to_string()),
        }
    }

    if !changed {
        return None;
    }

    Some(format!("{quote}{}{quote}", result_lines.join("\n")))
}

fn wrap_ternary(
    expr: &str,
    min_ops: usize,
    indent_size: usize,
    use_tabs: bool,
    current_indent: usize,
) -> Option<String> {
    let (condition, then_expr, else_expr) = split_top_level_ternary(expr)?;
    let continuation = if use_tabs {
        "\t".repeat(current_indent + 2)
    } else {
        " ".repeat((current_indent + 2) * indent_size)
    };

    let should_wrap_condition = count_top_level_ops(&condition) >= min_ops
        || (min_ops > 0 && (condition.contains("&&") || condition.contains("||")));

    let wrapped_condition = if should_wrap_condition {
        let parts = split_on_top_level_ops(&condition);
        if parts.len() >= 2 {
            if use_tabs {
                let group_indent = continuation.clone();
                let inner_indent = format!("{group_indent}\t");
                let mut lines = Vec::with_capacity(parts.len() + 2);
                lines.push(format!("{group_indent}("));
                lines.push(format!("{inner_indent}   {}", parts[0].1));
                for (op, item) in parts.into_iter().skip(1) {
                    lines.push(format!("{inner_indent}{op} {item}"));
                }
                lines.push(format!("{group_indent})"));
                format!("\n{}", lines.join("\n"))
            } else {
                let mut lines = vec![parts[0].1.clone()];
                for (op, item) in parts.into_iter().skip(1) {
                    lines.push(format!("{continuation}{op} {item}"));
                }
                lines.join("\n")
            }
        } else {
            condition
        }
    } else {
        condition
    };

    Some(format!(
        "{wrapped_condition}\n{continuation}? {then_expr}\n{continuation}: {else_expr}"
    ))
}

fn format_object_literal(
    inner: &str,
    min_ops: usize,
    indent_size: usize,
    use_tabs: bool,
    wrap_in_parens: bool,
    force_wrap_single_line: bool,
) -> Option<String> {
    let trimmed = inner.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return None;
    }
    if !inner.contains('\n') && !force_wrap_single_line {
        return None;
    }

    let mut entry_ws = "";
    for line in inner.split('\n').skip(1) {
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() {
            continue;
        }
        entry_ws = leading_ws_of_line(line);
        break;
    }
    if entry_ws.is_empty() && use_tabs {
        entry_ws = "\t\t\t";
    }

    let body = trimmed[1..trimmed.len() - 1].trim();
    let entries = split_top_level_commas(body);
    if entries.is_empty() {
        return None;
    }

    let close_ws = entry_ws;
    let mut lines = vec!["{".to_string()];

    for (index, entry) in entries.iter().enumerate() {
        let Some(colon_index) = find_kv_colon(entry) else {
            lines.push(format!("{entry_ws}{entry}"));
            continue;
        };

        let key_part = entry[..=colon_index].trim_end();
        let value = entry[colon_index + 1..].trim_start();
        let suffix = if index + 1 == entries.len() { "" } else { "," };

        if wrap_in_parens && use_tabs && is_wrappable_condition_expr(value, min_ops) {
            let leading_tabs = entry_ws.bytes().take_while(|b| *b == b'\t').count();
            let synthetic = format!("{}{} {}", "\t".repeat(leading_tabs), key_part, value);
            if let Some(wrapped) =
                wrap_line_conditions(&synthetic, min_ops, indent_size, use_tabs, 0, true)
            {
                let mut wrapped_lines: Vec<String> =
                    wrapped.split('\n').map(ToString::to_string).collect();
                if suffix == "," {
                    if let Some(last) = wrapped_lines.last_mut() {
                        last.push(',');
                    }
                }
                lines.extend(wrapped_lines);
                continue;
            }
        }

        lines.push(format!("{entry_ws}{} {}", key_part, value.trim_end_matches(',')) + suffix);
    }

    lines.push(format!("{close_ws}}}"));
    Some(lines.join("\n"))
}

fn format_class_tokens(
    inner: &str,
    min_tokens: usize,
    current_indent: usize,
    use_tabs: bool,
    indent_size: usize,
) -> Option<String> {
    if inner.contains('\n') {
        return None;
    }

    let tokens: Vec<&str> = inner.split_whitespace().collect();
    if tokens.len() < min_tokens || tokens.is_empty() {
        return None;
    }

    let token_indent = if use_tabs {
        "\t".repeat(current_indent + 2)
    } else {
        " ".repeat((current_indent + 2) * indent_size)
    };
    let close_indent = if use_tabs {
        "\t".repeat(current_indent + 1)
    } else {
        " ".repeat((current_indent + 1) * indent_size)
    };

    let mut out = String::from("\n");
    for token in tokens {
        out.push_str(&token_indent);
        out.push_str(token);
        out.push('\n');
    }
    out.push_str(&close_indent);
    Some(out)
}

fn force_wrap_ngclass_object(
    inner: &str,
    current_indent: usize,
    use_tabs: bool,
    indent_size: usize,
) -> Option<String> {
    let trimmed = inner.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return None;
    }

    let body = trimmed[1..trimmed.len() - 1].trim();
    let entries = split_top_level_commas(body);
    if entries.len() < 2 {
        return None;
    }

    let entry_indent = if use_tabs {
        "\t".repeat(current_indent + 2)
    } else {
        " ".repeat((current_indent + 2) * indent_size)
    };

    let mut lines = Vec::with_capacity(entries.len() + 2);
    lines.push("{".to_string());
    for (index, entry) in entries.iter().enumerate() {
        let suffix = if index + 1 == entries.len() { "" } else { "," };
        lines.push(format!("{entry_indent}{}{}", entry.trim(), suffix));
    }
    lines.push(format!("{entry_indent}}}"));
    Some(lines.join("\n"))
}

fn wrap_line_conditions(
    raw_line: &str,
    min_ops: usize,
    indent_size: usize,
    use_tabs: bool,
    fallback_cont_indent: usize,
    wrap_in_parens: bool,
) -> Option<String> {
    let line = raw_line.trim_end_matches('\r');
    let leading_tabs = line.bytes().take_while(|b| *b == b'\t').count();
    let leading_ws = &line[..leading_tabs];
    let content = &line[leading_tabs..];

    if content.is_empty() {
        return None;
    }

    let (key_part, conditions_str) = match find_kv_colon(content) {
        Some(colon_index) => {
            let key = &content[..=colon_index];
            let after = content[colon_index + 1..].trim_start();
            (key, after)
        }
        None => ("", content),
    };

    let op_count = count_top_level_ops(conditions_str);
    let negated_group = if op_count < min_ops {
        unwrap_negated_group(conditions_str)
    } else {
        None
    };
    if op_count < min_ops && negated_group.is_none() {
        return None;
    }

    let parts = if let Some(inner) = negated_group {
        split_on_top_level_ops(inner)
    } else {
        split_on_top_level_ops(conditions_str)
    };
    if parts.len() < 2 {
        return None;
    }

    let first = &parts[0].1;

    if use_tabs && indent_size > 0 {
        if wrap_in_parens && !key_part.is_empty() {
            let group_indent = format!("{leading_ws}\t");
            let inner_indent = format!("{group_indent}\t");
            let mut lines = Vec::with_capacity(parts.len() + 2);
            let mut parts = parts.clone();
            let last_index = parts.len() - 1;
            let (body, suffix) = split_trailing_punctuation(&parts[last_index].1);
            parts[last_index].1 = body;
            lines.push(format!("{leading_ws}{key_part}"));
            lines.push(format!("{group_indent}("));
            if negated_group.is_some() {
                let negation_indent = format!("{group_indent}\t");
                let nested_indent = format!("{negation_indent}\t");
                lines.push(format!("{negation_indent}!("));
                lines.push(format!("{nested_indent}   {first}"));
                for (op, item) in parts[1..].iter() {
                    lines.push(format!("{nested_indent}{op} {item}"));
                }
                lines.push(format!("{negation_indent})"));
            } else {
                lines.push(format!("{inner_indent}   {first}"));
                for (op, item) in parts[1..].iter() {
                    lines.push(format!("{inner_indent}{op} {item}"));
                }
            }
            lines.push(format!("{group_indent}){suffix}"));
            return Some(lines.join("\n"));
        }

        let tab_width = indent_size;
        let base_col = leading_tabs * tab_width;
        let after_key_col = base_col + key_part.len();
        let min_op_col = (after_key_col + 1).saturating_sub(3);
        let next_tab_col = if min_op_col % tab_width == 0 {
            min_op_col
        } else {
            (min_op_col / tab_width + 1) * tab_width
        };
        let first_col = next_tab_col + 3;
        let padding = first_col - after_key_col;
        let continuation = "\t".repeat(next_tab_col / tab_width);

        let mut lines = Vec::with_capacity(parts.len());
        lines.push(format!(
            "{leading_ws}{key_part}{}{} ",
            " ".repeat(padding),
            first
        ));
        for (index, (op, item)) in parts[1..].iter().enumerate() {
            let is_last = index == parts.len() - 2;
            if is_last {
                lines.push(format!("{continuation}{op} {item}"));
            } else {
                lines.push(format!("{continuation}{op} {item} "));
            }
        }
        return Some(lines.join("\n"));
    }

    if use_tabs && leading_tabs == 0 && key_part.is_empty() && fallback_cont_indent > 0 {
        let continuation = "\t".repeat(fallback_cont_indent);
        let mut lines = vec![first.clone()];
        for (index, (op, item)) in parts[1..].iter().enumerate() {
            let is_last = index == parts.len() - 2;
            if is_last {
                lines.push(format!("{continuation}{op} {item}"));
            } else {
                lines.push(format!("{continuation}{op} {item} "));
            }
        }
        return Some(lines.join("\n"));
    }

    let leading_spaces = line.bytes().take_while(|b| *b == b' ').count();
    let after_key_col = leading_spaces + key_part.len();
    let first_col = after_key_col + 1;
    let op_col = first_col.saturating_sub(3);
    let continuation = " ".repeat(op_col);
    let mut lines = Vec::with_capacity(parts.len());
    lines.push(format!(
        "{}{}{}{} ",
        " ".repeat(leading_spaces),
        key_part,
        " ".repeat(first_col - after_key_col),
        first,
    ));
    for (index, (op, item)) in parts[1..].iter().enumerate() {
        let is_last = index == parts.len() - 2;
        if is_last {
            lines.push(format!("{continuation}{op} {item}"));
        } else {
            lines.push(format!("{continuation}{op} {item} "));
        }
    }
    Some(lines.join("\n"))
}

fn leading_ws_of_line(s: &str) -> &str {
    let ws_len = s.bytes().take_while(|b| matches!(b, b' ' | b'\t')).count();
    &s[..ws_len]
}

fn split_top_level_ternary(s: &str) -> Option<(String, String, String)> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut in_quote: Option<u8> = None;
    let mut question_index: Option<usize> = None;
    let mut colon_index: Option<usize> = None;
    let mut index = 0usize;

    while index < bytes.len() {
        match in_quote {
            Some(quote) => {
                if bytes[index] == b'\\' {
                    index += 2;
                    continue;
                }
                if bytes[index] == quote {
                    in_quote = None;
                }
                index += 1;
            }
            None => match bytes[index] {
                b'"' | b'\'' | b'`' => {
                    in_quote = Some(bytes[index]);
                    index += 1;
                }
                b'(' | b'[' | b'{' => {
                    depth += 1;
                    index += 1;
                }
                b')' | b']' | b'}' => {
                    depth = depth.saturating_sub(1);
                    index += 1;
                }
                b'?' if depth == 0 && question_index.is_none() => {
                    question_index = Some(index);
                    index += 1;
                }
                b':' if depth == 0 && question_index.is_some() => {
                    colon_index = Some(index);
                    index += 1;
                }
                _ => index += 1,
            },
        }
    }

    let question_index = question_index?;
    let colon_index = colon_index?;
    if question_index >= colon_index {
        return None;
    }

    let condition = s[..question_index].trim().to_string();
    let then_expr = s[question_index + 1..colon_index].trim().to_string();
    let else_expr = s[colon_index + 1..].trim().to_string();

    if condition.is_empty() || then_expr.is_empty() || else_expr.is_empty() {
        return None;
    }

    Some((condition, then_expr, else_expr))
}

fn find_kv_colon(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut in_quote: Option<u8> = None;
    let mut index = 0usize;

    while index < bytes.len() {
        match in_quote {
            Some(quote) => {
                if bytes[index] == b'\\' {
                    index += 2;
                    continue;
                }
                if bytes[index] == quote {
                    in_quote = None;
                }
                index += 1;
            }
            None => match bytes[index] {
                b'"' | b'\'' | b'`' => {
                    in_quote = Some(bytes[index]);
                    index += 1;
                }
                b'(' | b'[' | b'{' => {
                    depth += 1;
                    index += 1;
                }
                b')' | b']' | b'}' => {
                    depth = depth.saturating_sub(1);
                    index += 1;
                }
                b':' if depth == 0 => {
                    let next = bytes.get(index + 1).copied().unwrap_or_default();
                    if next == b' ' || next == b'\t' {
                        return Some(index);
                    }
                    index += 1;
                }
                _ => index += 1,
            },
        }
    }

    None
}

```

# FILE: crates/fua-plugin-angular/src/context.rs
```rust
pub(crate) fn is_content_context(parent_kind: &str) -> bool {
    parent_kind == "ELEMENT" || parent_kind == "ROOT"
}

pub(crate) fn is_block_opener(text: &str) -> bool {
    matches!(
        text,
        "@if" | "@for" | "@switch" | "@case" | "@default" | "@empty"
    )
}

pub(crate) fn is_else_like(text: &str) -> bool {
    text == "@else" || text.starts_with("@else ")
}

pub(crate) fn is_angular_binding(attr_name: &str) -> bool {
    let name = attr_name.trim();
    (name.starts_with('[') && name.ends_with(']'))
        || (name.starts_with('(') && name.ends_with(')'))
        || name.starts_with("*ng")
        || name.starts_with("*cdk")
}

pub(crate) fn is_ngclass_attr(attr_name: &str) -> bool {
    let name = attr_name.trim();
    name.eq_ignore_ascii_case("[ngclass]") || name.eq_ignore_ascii_case("ngclass")
}

pub(crate) fn is_class_attr(attr_name: &str) -> bool {
    attr_name.trim().eq_ignore_ascii_case("class")
}

```

# FILE: crates/fua-plugin-angular/src/expressions.rs
```rust
pub(crate) fn count_top_level_ops(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut count = 0usize;
    let mut index = 0usize;

    while index < bytes.len() {
        match bytes[index] {
            b'(' | b'[' | b'{' => {
                depth += 1;
                index += 1;
            }
            b')' | b']' | b'}' => {
                depth = depth.saturating_sub(1);
                index += 1;
            }
            b'"' | b'\'' | b'`' => {
                let quote = bytes[index];
                index += 1;
                while index < bytes.len() {
                    if bytes[index] == b'\\' {
                        index += 2;
                        continue;
                    }
                    if bytes[index] == quote {
                        index += 1;
                        break;
                    }
                    index += 1;
                }
            }
            b'|' if depth == 0 && bytes.get(index + 1) == Some(&b'|') => {
                count += 1;
                index += 2;
            }
            b'&' if depth == 0 && bytes.get(index + 1) == Some(&b'&') => {
                count += 1;
                index += 2;
            }
            _ => index += 1,
        }
    }

    count
}

pub(crate) fn split_on_top_level_ops(s: &str) -> Vec<(String, String)> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut current_op = String::new();
    let mut index = 0usize;

    while index < bytes.len() {
        match bytes[index] {
            b'(' | b'[' | b'{' => {
                depth += 1;
                current.push(bytes[index] as char);
                index += 1;
            }
            b')' | b']' | b'}' => {
                depth = depth.saturating_sub(1);
                current.push(bytes[index] as char);
                index += 1;
            }
            b'"' | b'\'' | b'`' => {
                let quote = bytes[index];
                current.push(quote as char);
                index += 1;
                while index < bytes.len() {
                    current.push(bytes[index] as char);
                    if bytes[index] == b'\\' {
                        index += 1;
                        if index < bytes.len() {
                            current.push(bytes[index] as char);
                        }
                    } else if bytes[index] == quote {
                        index += 1;
                        break;
                    }
                    index += 1;
                }
            }
            b'|' if depth == 0 && bytes.get(index + 1) == Some(&b'|') => {
                parts.push((current_op.clone(), current.trim().to_string()));
                current_op = "||".to_string();
                current.clear();
                index += 2;
                while matches!(bytes.get(index), Some(b' ' | b'\t')) {
                    index += 1;
                }
            }
            b'&' if depth == 0 && bytes.get(index + 1) == Some(&b'&') => {
                parts.push((current_op.clone(), current.trim().to_string()));
                current_op = "&&".to_string();
                current.clear();
                index += 2;
                while matches!(bytes.get(index), Some(b' ' | b'\t')) {
                    index += 1;
                }
            }
            _ => {
                current.push(bytes[index] as char);
                index += 1;
            }
        }
    }

    let last = current.trim().to_string();
    if !last.is_empty() || !current_op.is_empty() {
        parts.push((current_op, last));
    }

    parts
}

pub(crate) fn split_trailing_punctuation(s: &str) -> (String, String) {
    let trimmed = s.trim_end();
    if let Some(last) = trimmed.chars().last() {
        if matches!(last, ',' | ';') {
            let body = trimmed[..trimmed.len() - last.len_utf8()]
                .trim_end()
                .to_string();
            return (body, last.to_string());
        }
    }
    (trimmed.to_string(), String::new())
}

pub(crate) fn unwrap_negated_group(s: &str) -> Option<&str> {
    let trimmed = s.trim();
    if !trimmed.starts_with("!(") || !trimmed.ends_with(')') {
        return None;
    }

    let inner = &trimmed[2..trimmed.len() - 1];
    if count_top_level_ops(inner) == 0 {
        return None;
    }

    Some(inner.trim())
}

pub(crate) fn is_wrappable_condition_expr(s: &str, min_ops: usize) -> bool {
    let trimmed = s.trim();
    count_top_level_ops(trimmed) >= min_ops || unwrap_negated_group(trimmed).is_some()
}

pub(crate) fn split_top_level_commas(s: &str) -> Vec<String> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut in_quote: Option<u8> = None;
    let mut current = String::new();
    let mut parts = Vec::new();
    let mut index = 0usize;

    while index < bytes.len() {
        match in_quote {
            Some(quote) => {
                current.push(bytes[index] as char);
                if bytes[index] == b'\\' {
                    index += 1;
                    if index < bytes.len() {
                        current.push(bytes[index] as char);
                    }
                } else if bytes[index] == quote {
                    in_quote = None;
                }
            }
            None => match bytes[index] {
                b'"' | b'\'' | b'`' => {
                    in_quote = Some(bytes[index]);
                    current.push(bytes[index] as char);
                }
                b'(' | b'[' | b'{' => {
                    depth += 1;
                    current.push(bytes[index] as char);
                }
                b')' | b']' | b'}' => {
                    depth = depth.saturating_sub(1);
                    current.push(bytes[index] as char);
                }
                b',' if depth == 0 => {
                    let part = current.trim();
                    if !part.is_empty() {
                        parts.push(part.to_string());
                    }
                    current.clear();
                }
                _ => current.push(bytes[index] as char),
            },
        }
        index += 1;
    }

    let last = current.trim();
    if !last.is_empty() {
        parts.push(last.to_string());
    }

    parts
}

```

# FILE: crates/fua-plugin-angular/src/hooks.rs
```rust
use crate::attributes::process_attribute_string;
use crate::context::{
    is_angular_binding, is_block_opener, is_class_attr, is_content_context, is_else_like,
};
use crate::response::{
    HookResponseExt, condition_whitespace_replacement, replacement, replacement_with_spacing,
};
use crate::state::{read_state, reset_state, with_state};
use fua_plugin_api::{
    HookRequest, HookResponse, LeadingSpacing, NodeHook, NodePhase, Replacement, TokenHook,
};
use serde_json::{Value, json};

pub fn dispatch_hook(request: HookRequest<'static>) -> HookResponse {
    match request {
        HookRequest::Node(node) => handle_node_hook(&node),
        HookRequest::Token(token) => handle_token_hook(&token),
    }
}

fn handle_node_hook(node: &NodeHook<'_>) -> HookResponse {
    if node.phase == NodePhase::Enter && node.kind == "ROOT" {
        reset_state();
    }

    HookResponse::Continue
}

fn handle_token_hook(token: &TokenHook<'_>) -> HookResponse {
    if should_process_attribute_string(token) {
        return handle_attribute_string(token);
    }

    if !is_content_context(token.context.parent_kind.as_ref()) {
        return HookResponse::Continue;
    }

    match token.kind.as_ref() {
        "IDENT" => handle_content_ident(token),
        "WHITESPACE" => handle_content_whitespace(token),
        "TEXT" => handle_content_text(token),
        _ => HookResponse::Continue,
    }
}

fn should_process_attribute_string(token: &TokenHook<'_>) -> bool {
    matches!(token.kind.as_ref(), "STRING_DOUBLE" | "STRING_SINGLE")
        && matches!(
            token.context.parent_kind.as_ref(),
            "OPEN_TAG" | "SELF_CLOSING_TAG"
        )
        && token
            .context
            .attribute_name
            .as_deref()
            .is_some_and(|name| is_angular_binding(name) || is_class_attr(name))
}

fn handle_attribute_string(token: &TokenHook<'_>) -> HookResponse {
    let options = plugin_options(token.plugin_options.as_deref());
    let attr_name = token.context.attribute_name.as_deref().unwrap_or_default();

    if let Some(output) = process_attribute_string(
        token.text.as_ref(),
        attr_name,
        &options,
        token.context.current_indent,
        token.context.indent_size,
        token.context.use_tabs,
    ) {
        replacement(output)
    } else {
        HookResponse::Continue
    }
}

fn handle_content_ident(token: &TokenHook<'_>) -> HookResponse {
    if is_block_opener(token.text.as_ref()) {
        let _ = with_state(|state| {
            state.awaiting_condition_start = true;
        });
        return replacement_with_spacing(token.text.as_ref(), LeadingSpacing::LineBreak);
    }

    if is_else_like(token.text.as_ref()) {
        return replacement_with_spacing(token.text.as_ref(), LeadingSpacing::Space);
    }

    clear_pending_condition_start();
    HookResponse::Continue
}

fn handle_content_whitespace(token: &TokenHook<'_>) -> HookResponse {
    let options = plugin_options(token.plugin_options.as_deref());
    let Some(target_indent) = read_state(|state| {
        if state.condition_depth == 0 || !token.text.contains('\n') {
            return None;
        }

        let next_is_close_paren = token.context.next_kind.as_deref() == Some("TEXT")
            && token.context.next_text.as_deref() == Some(")");
        let target_indent = if options
            .get("indent_condition_groups")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            state.condition_base_indent + state.condition_depth - usize::from(next_is_close_paren)
        } else if next_is_close_paren {
            state.condition_base_indent
        } else {
            state.condition_base_indent + 1
        };

        Some(target_indent)
    })
    .flatten() else {
        return HookResponse::Continue;
    };

    condition_whitespace_replacement(token.context.current_indent, target_indent)
}

fn handle_content_text(token: &TokenHook<'_>) -> HookResponse {
    match token.text.as_ref() {
        "(" => open_condition_group(token.context.current_indent),
        ")" => close_condition_group(),
        "{" => replacement_with_spacing("{", LeadingSpacing::Space).map_indent(0, 1),
        "}" => HookResponse::replace(
            Replacement::text("}")
                .with_indent(-1, 0)
                .with_leading(LeadingSpacing::LineBreak),
        ),
        _ => {
            clear_pending_condition_start();
            HookResponse::Continue
        }
    }
}

fn open_condition_group(current_indent: usize) -> HookResponse {
    let response = with_state(|state| {
        if state.awaiting_condition_start {
            state.awaiting_condition_start = false;
            state.condition_depth = 1;
            state.condition_base_indent = current_indent;
            return Some(replacement("("));
        }

        if state.condition_depth > 0 {
            state.condition_depth += 1;
            return Some(replacement("("));
        }

        None
    })
    .flatten();

    response.unwrap_or(HookResponse::Continue)
}

fn close_condition_group() -> HookResponse {
    let response = with_state(|state| {
        if state.condition_depth > 0 {
            state.condition_depth = state.condition_depth.saturating_sub(1);
            Some(replacement(")"))
        } else {
            None
        }
    })
    .flatten();

    response.unwrap_or(HookResponse::Continue)
}

fn clear_pending_condition_start() {
    let _ = with_state(|state| {
        if state.awaiting_condition_start {
            state.awaiting_condition_start = false;
        }
    });
}

fn plugin_options(plugin_options: Option<&str>) -> Value {
    plugin_options
        .and_then(|options| serde_json::from_str(options).ok())
        .unwrap_or_else(|| json!({}))
}

```

# FILE: crates/fua-plugin-angular/src/lib.rs
```rust
mod attributes;
mod context;
mod expressions;
mod hooks;
mod response;
mod state;

pub use hooks::dispatch_hook;

#[cfg(target_arch = "wasm32")]
use extism_pdk::*;
#[cfg(target_arch = "wasm32")]
use fua_plugin_api::{HookRequest, HookResponse};

#[cfg(target_arch = "wasm32")]
#[plugin_fn]
pub fn handle_hook(input: String) -> FnResult<String> {
    let request = match serde_json::from_str::<HookRequest<'static>>(&input) {
        Ok(request) => request,
        Err(_) => return Ok(String::new()),
    };

    let response = dispatch_hook(request);
    match response {
        HookResponse::Continue => Ok(String::new()),
        other => Ok(serde_json::to_string(&other).unwrap_or_default()),
    }
}

#[cfg(test)]
mod tests {
    use super::dispatch_hook;
    use fua_plugin_api::{
        HookContext, HookRequest, HookResponse, LeadingSpacing, NodeHook, NodePhase, Replacement,
        TokenHook,
    };
    use std::borrow::Cow;

    fn token_request(
        kind: &'static str,
        text: &'static str,
        parent_kind: &'static str,
    ) -> HookRequest<'static> {
        HookRequest::Token(TokenHook {
            kind: Cow::Borrowed(kind),
            text: Cow::Borrowed(text),
            context: HookContext::new(parent_kind, None, None, 2, 2, false),
            plugin_options: None,
        })
    }

    #[test]
    fn resets_state_on_root_enter() {
        let request = HookRequest::Node(NodeHook {
            phase: NodePhase::Enter,
            kind: Cow::Borrowed("ROOT"),
            text: Cow::Borrowed(""),
            tag_name: None,
            context: HookContext::new("NONE", None, None, 0, 2, false),
            plugin_options: None,
        });

        let response = dispatch_hook(request);
        assert_eq!(response, HookResponse::Continue);
    }

    #[test]
    fn block_openers_start_on_a_new_line() {
        let response = dispatch_hook(token_request("IDENT", "@if", "ROOT"));
        assert_eq!(
            response,
            HookResponse::replace(Replacement::text("@if").with_leading(LeadingSpacing::LineBreak),)
        );
    }
}

```

# FILE: crates/fua-plugin-angular/src/response.rs
```rust
use fua_plugin_api::{HookResponse, LeadingSpacing, Replacement};

pub(crate) fn replacement(output: impl Into<String>) -> HookResponse {
    HookResponse::replace(Replacement::text(output))
}

pub(crate) fn replacement_with_spacing(
    output: impl Into<String>,
    spacing: LeadingSpacing,
) -> HookResponse {
    HookResponse::replace(Replacement::text(output).with_leading(spacing))
}

pub(crate) fn condition_whitespace_replacement(
    current_indent: usize,
    target_indent: usize,
) -> HookResponse {
    let delta = target_indent as i32 - current_indent as i32;
    HookResponse::replace(
        Replacement::text("")
            .with_indent(delta, -delta)
            .with_leading(LeadingSpacing::LineBreak),
    )
}

pub(crate) trait HookResponseExt {
    fn map_indent(self, indent_before: i32, indent_after: i32) -> Self;
}

impl HookResponseExt for HookResponse {
    fn map_indent(self, indent_before: i32, indent_after: i32) -> Self {
        match self {
            HookResponse::Continue => HookResponse::Continue,
            HookResponse::Replace(replacement) => {
                HookResponse::Replace(replacement.with_indent(indent_before, indent_after))
            }
        }
    }
}

```

# FILE: crates/fua-plugin-angular/src/state.rs
```rust
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Default)]
pub(crate) struct AngularState {
    pub(crate) condition_depth: usize,
    pub(crate) condition_base_indent: usize,
    pub(crate) awaiting_condition_start: bool,
}

fn state() -> &'static Mutex<AngularState> {
    static STATE: OnceLock<Mutex<AngularState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(AngularState::default()))
}

pub(crate) fn reset_state() {
    if let Ok(mut state) = state().lock() {
        *state = AngularState::default();
    }
}

pub(crate) fn with_state<T>(f: impl FnOnce(&mut AngularState) -> T) -> Option<T> {
    state().lock().ok().map(|mut state| f(&mut state))
}

pub(crate) fn read_state<T>(f: impl FnOnce(&AngularState) -> T) -> Option<T> {
    state().lock().ok().map(|state| f(&state))
}

```

# FILE: crates/fua-plugin-api/Cargo.toml
```toml
[package]
name = "fua-plugin-api"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { workspace = true }

```

# FILE: crates/fua-plugin-api/src/lib.rs
```rust
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

pub const HANDLE_HOOK_EXPORT: &str = "handle_hook";

pub type Text<'a> = Cow<'a, str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodePhase {
    Enter,
    Exit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookContext<'a> {
    pub parent_kind: Text<'a>,
    pub tag_name: Option<Text<'a>>,
    pub attribute_name: Option<Text<'a>>,
    pub previous_kind: Option<Text<'a>>,
    pub previous_text: Option<Text<'a>>,
    pub next_kind: Option<Text<'a>>,
    pub next_text: Option<Text<'a>>,
    pub current_indent: usize,
    pub indent_size: usize,
    pub use_tabs: bool,
}

impl<'a> HookContext<'a> {
    pub fn new(
        parent_kind: &'a str,
        tag_name: Option<&'a str>,
        attribute_name: Option<&'a str>,
        current_indent: usize,
        indent_size: usize,
        use_tabs: bool,
    ) -> Self {
        Self {
            parent_kind: Cow::Borrowed(parent_kind),
            tag_name: tag_name.map(Cow::Borrowed),
            attribute_name: attribute_name.map(Cow::Borrowed),
            previous_kind: None,
            previous_text: None,
            next_kind: None,
            next_text: None,
            current_indent,
            indent_size,
            use_tabs,
        }
    }

    pub fn with_neighbors(
        mut self,
        previous_kind: Option<&'a str>,
        previous_text: Option<&'a str>,
        next_kind: Option<&'a str>,
        next_text: Option<&'a str>,
    ) -> Self {
        self.previous_kind = previous_kind.map(Cow::Borrowed);
        self.previous_text = previous_text.map(Cow::Borrowed);
        self.next_kind = next_kind.map(Cow::Borrowed);
        self.next_text = next_text.map(Cow::Borrowed);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeHook<'a> {
    pub phase: NodePhase,
    pub kind: Text<'a>,
    pub text: Text<'a>,
    pub tag_name: Option<Text<'a>>,
    pub context: HookContext<'a>,
    pub plugin_options: Option<Text<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenHook<'a> {
    pub kind: Text<'a>,
    pub text: Text<'a>,
    pub context: HookContext<'a>,
    pub plugin_options: Option<Text<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "hook", rename_all = "snake_case")]
pub enum HookRequest<'a> {
    Node(NodeHook<'a>),
    Token(TokenHook<'a>),
}

impl<'a> HookRequest<'a> {
    pub fn node(
        phase: NodePhase,
        kind: &'a str,
        text: &'a str,
        tag_name: Option<&'a str>,
        context: HookContext<'a>,
    ) -> Self {
        Self::Node(NodeHook {
            phase,
            kind: Cow::Borrowed(kind),
            text: Cow::Borrowed(text),
            tag_name: tag_name.map(Cow::Borrowed),
            context,
            plugin_options: None,
        })
    }

    pub fn token(kind: &'a str, text: &'a str, context: HookContext<'a>) -> Self {
        Self::Token(TokenHook {
            kind: Cow::Borrowed(kind),
            text: Cow::Borrowed(text),
            context,
            plugin_options: None,
        })
    }

    pub fn with_plugin_options(self, plugin_options: Option<Text<'a>>) -> Self {
        match self {
            Self::Node(mut hook) => {
                hook.plugin_options = plugin_options;
                Self::Node(hook)
            }
            Self::Token(mut hook) => {
                hook.plugin_options = plugin_options;
                Self::Token(hook)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeadingSpacing {
    None,
    Space,
    LineBreak,
    BlankLine,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Replacement {
    pub output: String,
    pub indent_before: i32,
    pub indent_after: i32,
    pub leading_spacing: LeadingSpacing,
}

impl Replacement {
    pub fn text(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            indent_before: 0,
            indent_after: 0,
            leading_spacing: LeadingSpacing::None,
        }
    }

    pub fn with_leading(mut self, leading_spacing: LeadingSpacing) -> Self {
        self.leading_spacing = leading_spacing;
        self
    }

    pub fn with_indent(mut self, indent_before: i32, indent_after: i32) -> Self {
        self.indent_before = indent_before;
        self.indent_after = indent_after;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum HookResponse {
    Continue,
    Replace(Replacement),
}

impl HookResponse {
    pub fn replace(replacement: Replacement) -> Self {
        Self::Replace(replacement)
    }
}

```

# FILE: examples/config.json
```json
{
  "indent_size": 4,
  "use_tabs": true,
  "print_width": 100,
  "bracket_same_line": false,
  "wrap_attributes": true,
  "single_quotes": false,
  "wrap_content": true,
  "inline_short_elements_max_len": 80,
  "plugins": [
    {
      "path": "../target/wasm32-wasip1/release/fua_plugin_angular.wasm",
      "options": {
        "wrap_conditions_min": 2,
        "wrap_conditions_in_parens": true,
        "wrap_ternary": true,
        "ngclass_wrap_entries_min": 2,
        "class_wrap_tokens_min": 6,
        "indent_condition_groups": true
      }
    }
  ]
}

```

# FILE: examples/formatted.html
```html

@if (this.audioCollectionEditService.editableCollection) {

	<!-- Main collection container -->
	<main
		class="flex rounded-xl bg-cover bg-center"
		[ngStyle]="this.imageUrl && !this.editingMode && this.audioCollectionEditService.editableCollection.imageId !== '00000000-0000-0000-0000-000000000000' ? {'background-image': this.getImageUrl()} : {}"
	>
		<div
			class="flex w-full flex-col justify-center rounded-xl p-4"
			[ngClass]="{'backdrop-brightness-50': this.imageUrl}"
		>

			<!-- Grid Layout Container -->
			<div
				class="grid h-fit w-full grid-cols-4 gap-8"
			>

				<!-- Collection Info Area (3/4 width - col-span-3) -->
				<div
					class="bg-tertiary-lighter-5% col-span-3 flex flex-col rounded-lg border-2 p-6"
					[ngClass]="{
						'border-accent': this.hasNameChanges || this.hasDescriptionChanges || this.hasImageChanges,
						'border-quaternary': !(this.hasNameChanges || this.hasDescriptionChanges || this.hasImageChanges)
					}"
				>
					<div
						class="absolute right-4 top-4 z-10"
					>
						<button
							class="btn-default btn-accent"
							(click)="this.saveAllChanges()"
						>

							{{ "submitChanges" | translate }}
						</button>
					</div>

					<!-- Success Message -->

					@if (showSaveSuccess) {
						<div
							class="absolute -top-3 right-4 z-10"
						>
							<div
								class="animate-fadeIn flex items-center gap-2 rounded-lg bg-green-500 px-4 py-1.5 text-sm text-white shadow-md"
							>
								<i class="fa-solid fa-check"></i>
								<span>Saved successfully</span>
							</div>
						</div>
					}
					<div
						class="flex w-full justify-between"
					>

						@if (this.audioCollection) {
							<div
								class="flex items-center space-x-2"
							>
								<i
									class="fa-regular fa-heart cursor-pointer"
									[ngClass]="{'text-secondary': this.isCollectionLiked, 'text-quinary-darker-5%': !this.isCollectionLiked}"
									(click)="this.likeAudioCollection()"
									(keyup.enter)="this.likeAudioCollection()"
									tabindex="0"
								>
								</i>
								<p class="text-md text-quinary-darker-5% cursor-pointer">{{ this.audioCollection.likes }}</p>
							</div>
							<div
								class="flex items-center space-x-2"
							>
								<i class="fa-solid fa-eye text-quinary-darker-5%"></i>
								<p class="text-md cursor-pointer text-quinary-darker-5%">{{ this.totalPlays }}</p>
							</div>
						}
					</div>

					<!-- Top Row: Name and Controls -->
					<div
						class="mb-4 flex items-start justify-between"
					>

						<!-- Name Field -->
						<div
							class="flex-1 text-white"
						>

							@if (this.isNameEditing) {
								<input
									#nameInput
									type="text"
									[ngModel]="this.audioCollectionEditService.editableCollection.name"
									(input)="handleNameChange($event)"
									class="w-full bg-transparent text-3xl font-semibold outline-none"
									placeholder="Enter collection name"
									(blur)="finishNameEdit()"
								/>
							} @else {
								<div
									class="relative flex items-center"
								>
									<h2
										class="cursor-pointer truncate text-3xl font-semibold"
										(click)="startNameEdit()"
										[title]="this.audioCollectionEditService.editableCollection.name || 'Enter collection name'"
										(keyup.enter)="startNameEdit()"
										tabindex="0"
									>

										{{ this.audioCollectionEditService.editableCollection.name || "Enter collection name" }}
									</h2>

									@if (this.hasNameChanges) {
										<span
											class="text-accent ml-2"
											title="Name has unsaved changes"
										>
											<i class="fa-solid fa-circle-exclamation"></i>
										</span>
									}
								</div>
							}
						</div>

						<!-- Control Buttons -->
						<div
							class="flex space-x-4"
						>

							<!-- Play/Stop Button -->
							<button
								class="group flex flex-col items-center gap-1"
								tabindex="0"
								[disabled]="!this.audioPlayerService.loaded"
								[class.opacity-50]="!this.audioPlayerService.loaded"
								(click)="this.toggleAmbiencePreview()"
							>

								@if (this.isAmbiencePreviewPlaying) {
									<i
										class="fa-regular fa-circle-stop text-3xl text-white transition-all group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Stop</span>
								} @else {
									<i
										class="fa-regular fa-circle-play text-3xl text-white transition-all group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Play</span>
								}
							</button>

							<!-- Generate Button -->
							<button
								(click)="this.generateAmbience()"
								class="group flex flex-col items-center gap-1"
								[disabled]="this.isLoadingAmbience"
								[class.opacity-50]="this.isLoadingAmbience"
							>
								<i
									class="fa-solid fa-wand-magic-sparkles text-3xl text-white transition-all group-hover:scale-110"
								>
								</i>
								<span class="text-xs text-gray-300">Generate</span>
							</button>

							<!-- Download Button - Only show when ambience is generated -->

							@if (this.generatedAmibienceUrl && !this.isLoadingAmbience) {
								<button
									(click)="this.export()"
									class="group flex flex-col items-center gap-1"
								>
									<i
										class="fa-solid fa-download text-3xl text-white transition-all group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Download</span>
								</button>
							}
						</div>
					</div>

					<!-- Mid Row: Description -->
					<div
						class="relative mb-4 flex-1"
					>

						@if (this.isDescriptionEditing) {
							<textarea
								#descriptionInput
								[ngModel]="this.audioCollectionEditService.editableCollection.description"
								(input)="handleDesciptionChange($event)"
								class="h-24 w-full resize-none overflow-y-auto bg-transparent text-lg text-gray-200 outline-none"
								placeholder="Enter collection description"
								(blur)="finishDescriptionEdit()"
							>
							</textarea>
						} @else {
							<div
								class="relative h-24 overflow-y-auto"
							>
								<p
									class="cursor-pointer break-words text-lg text-gray-200"
									(click)="startDescriptionEdit()"
									(keyup.enter)="startDescriptionEdit()"
									tabindex="0"
								>

									{{ this.audioCollectionEditService.editableCollection.description || "Enter collection description" }}
								</p>

								@if (this.hasDescriptionChanges) {
									<span
										class="text-accent absolute right-0 top-0 mr-1"
										title="Description has unsaved changes"
									>
										<i class="fa-solid fa-circle-exclamation"></i>
									</span>
								}
							</div>
						}
					</div>

					<!-- Bottom Row: Waveform or Loading Area -->
					<div
						class="flex h-24 flex-col gap-3"
					>

						<!-- Loading Progress or Waveform -->

						@if (this.isLoadingAmbience) {
							<div
								class="flex w-full flex-col gap-2"
							>
								<p class="text-lg text-white">{{ this.getLoadingStage() }}</p>
								<div
									class="h-2 w-full"
								>
									<app-progressbar
										[progress]="this.ambienceLoadingProgress / 5"
										[buffer]="this.ambienceLoadingProgress === 5 || this.ambienceLoadingProgress === 0 ? this.ambienceLoadingProgress / 5 : this.ambienceLoadingProgress / 5 + 0.1"
										[type]="'linear-buffer'"
									>
									</app-progressbar>
								</div>
							</div>
						} @else {
							<div
								class="relative"
							>

								<!-- Empty state when no ambience -->

								@if (!this.generatedAmibienceUrl) {
									<div
										class="bg-tertiary-lighter-10% flex h-24 w-full flex-col items-center justify-center rounded-lg border border-dashed border-gray-600"
									>
										<p
											class="text-gray-400"
										>
											<i class="fa-solid fa-wave-square mr-2"></i>
											Click "Generate" to create your ambience
										</p>
									</div>
								}
							</div>
						}

						<!-- Actual waveform when generated -->
						<div
							#waveform
							class="h-24 w-full"
							[class.hidden]="!this.generatedAmibienceUrl"
						>
						</div>
					</div>
				</div>

				<!-- Image Upload Area (1/4 width - col-span-1) -->
				<div
					dropZone
					(filesDropped)="this.onImageFileDropped($event)"
					[ngClass]="{
						'border-accent': this.hasImageChanges,
						'border-quaternary': !this.hasImageChanges
					}"
					class="bg-tertiary-lighter-5% relative col-span-1 flex flex-col items-center justify-center rounded-lg border-2"
				>

					<!-- Current Image Preview or Upload UI -->
					<div
						class="flex h-full w-full flex-col items-center justify-center"
					>

						<!-- Show image if exists -->

						@if (
							this.imageUrl && this.audioCollectionEditService.editableCollection.imageId !==
							"00000000-0000-0000-0000-000000000000"
						) {
							<div
								class="relative h-full w-full"
							>
								<img
									[src]="this.imageUrl"
									class="h-full w-full rounded-lg object-cover"
									alt="Collection image"
								/>
								<div
									class="absolute inset-0 flex flex-col items-center justify-center rounded-lg bg-black bg-opacity-50 opacity-0 transition-opacity hover:opacity-100"
								>
									<i class="fa-solid fa-cloud-arrow-up mb-2 text-xl text-white"></i>
									<p class="mb-2 text-sm text-white">Replace image</p>
									<label
										for="file"
										class="btn-default hover:bg-primary-lighter cursor-pointer rounded-full bg-primary px-3 py-1 text-xs text-white"
									>
										Choose
										file
									</label>
								</div>
							</div>
						} @else {

							<!-- Upload UI if no image -->
							<div
								class="flex h-full flex-col items-center justify-center p-4 text-white"
							>
								<i class="fa-solid fa-cloud-arrow-up text-danger mb-4 text-2xl"></i>
								<p class="mb-3 text-center text-sm">{{ "dragAndDropFiles" | translate }}</p>
								<div
									class="my-1 flex w-full items-center justify-center"
								>
									<hr class="border-quinary w-1/3" />
									<span class="text-quinary px-2 text-xs">or</span>
									<hr class="border-quinary w-1/3" />
								</div>
								<label
									for="file"
									class="btn-default btn-secondary mt-3 px-4 py-1.5 text-sm transition-colors"
								>
									Choose
									file
								</label>
								<p class="text-quinary mt-3 text-xs">Max size: 16MB</p>
							</div>
						}
					</div>
					<input
						(change)="this.handleImageFileUpload($event)"
						accept=".png,.jpg,.jpeg,.hvc"
						type="file"
						name="file"
						id="file"
						class="hidden"
					/>

					<!-- Mark as changed if needed -->

					@if (this.hasImageChanges) {
						<div
							class="text-accent absolute right-2 top-2"
						>
							<i class="fa-solid fa-circle-exclamation"></i>
						</div>
					}
				</div>

				<!-- Timeline Panel (3/4 width - col-span-3) -->
				<div
					class="bg-tertiary-lighter-5% relative col-span-3 flex flex-col gap-8 rounded-lg border-2 p-6"
					[ngClass]="{
						'border-quaternary': !this.audioCollectionEditService.changes['presets'] || !this.audioCollectionEditService.changes['audioList'] || !this.audioCollectionEditService.changes['timeline'],
						'border-accent': this.audioCollectionEditService.changes['presets'] && this.audioCollectionEditService.changes['audioList'] && this.audioCollectionEditService.changes['timeline']
					}"
				>

					<!-- Main Panel Save Button (absolute position) -->

					@if (
						this.audioCollectionEditService.changes["presets"] && this.audioCollectionEditService.changes["audioList"]
						&& this.audioCollectionEditService.changes["timeline"]
					) {
						<button
							(click)="saveAllChanges()"
							class="btn-default btn-accent absolute -top-3 right-3 z-10 flex items-center space-x-2 px-3 py-1.5 text-sm"
						>
							<i class="fa-solid fa-save mr-2"></i>
							<span>Save All Changes</span>
						</button>
					}

					<!-- Section Header -->
					<div
						class="border-tertiary-lighter-20% flex items-center justify-between border-b pb-4"
					>
						<div
							class="flex flex-col"
						>
							<h3 class="text-quinary text-xl font-semibold">Timeline & Audio Groups</h3>
							<p
								class="text-quinary-darker-10% text-sm"
							>
								Manage your video timeline and audio tracks
							</p>
						</div>
					</div>

					<!-- Audio Groups Panel -->
					<div
						class="bg-tertiary-lighter-10% relative flex flex-col gap-4 rounded-lg border p-4"
						[ngClass]="{
							'border-accent': this.audioCollectionEditService.changes['presets'] || this.audioCollectionEditService.changes['audioList'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['presets'] && !this.audioCollectionEditService.changes['audioList']
						}"
					>

						<!-- Presets Component -->
						<app-presets
							[presets]="this.audioCollectionEditService.editableCollection.presets"
							[selectedPresetId]="this.audioCollectionEditService.selectedPreset!.id"
							[audioList]="this.audioCollectionEditService.editableCollection.audioList"
							[originalAudioCollection]="this.audioCollection"
							[canSave]="!this.hasAudiosCountChanges()"
							(presetAdd)="this.addPreset()"
							(presetSave)="this.savePresetChanges()"
						>
						</app-presets>

						<!-- Presets Save Button (absolute position) -->

						@if (
							this.audioCollectionEditService.changes["presets"] &&
							!this.audioCollectionEditService.changes["audioList"]
						) {
							<button
								(click)="savePresetChanges()"
								class="btn-default btn-accent absolute right-4 top-4 flex items-center space-x-2 px-3 py-1.5 text-sm"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Preset Changes</span>
							</button>
						}

						<!-- Audio Groups Component -->
						<app-audio-groups
							[audioGroups]="this.audioCollectionEditService.editableCollection.audioList"
							[activeAudioGroupId]="activeAudioGroupId"
							(addAudioGroup)="addEmptyAudioGroup()"
							(saveAudioGroupChanges)="saveAudioGroupChanges()"
							(setActiveAudioGroup)="setActiveAudioGroup($event)"
							(audioVolumeChange)="handleAudioVolumeChange($event)"
							(audioMuteChange)="handleAudioMuteChange($event)"
							(audioPlaybackOffsetChange)="handleAudioPlaybackOffsetChange($event)"
							(audioProbabilityChange)="handleAudioProbabilityChange($event)"
							(deleteAudioGroup)="deleteAudioGroupFromCollection($event)"
							(deleteAudioFile)="deleteAudioFileFromCollection($event)"
							(nameChange)="handleGroupNameChange($event)"
							(audioGroupRefsChange)="handleAudioGroupRefsChange($event)"
						>
						</app-audio-groups>
					</div>
					<div
						class="bg-tertiary-lighter-10% relative flex flex-col gap-4 rounded-lg border p-4"
						[ngClass]="{
							'border-accent': this.audioCollectionEditService.changes['timeline'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['timeline']
						}"
					>

						<!-- Timeline Save Button (absolute position) -->

						@if (
							this.audioCollectionEditService.changes["timeline"] &&
							!this.audioCollectionEditService.changes["presets"] &&
							!this.audioCollectionEditService.changes["audioList"]
						) {
							<button
								(click)="saveTimelineChanges()"
								class="btn-default btn-accent absolute -top-3 right-3 z-10 flex items-center space-x-2 px-3 py-1.5 text-sm"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Timeline Changes</span>
							</button>
						}

						<!-- Video Preview and Timeline Container -->
						<app-timeline
							(timelineChange)="handleTimelineChange($event)"
							[timeline]="this.audioCollectionEditService.editableCollection!.timeline"
							[timelineLength]="120000"
						>
						</app-timeline>
					</div>
					<button
						class="btn-default btn-accent"
						(click)="this.logChanges()"
					>
						Log Changes
					</button>
				</div>

				<!-- Audio Library Panel (1/4 width - col-span-1) -->
				<div
					class="border-quaternary bg-tertiary-lighter-5% col-span-1 flex flex-col gap-4 rounded-lg border-2 p-6"
				>

					<!-- Library Header -->
					<div
						class="flex flex-col gap-2"
					>
						<h3 class="text-quinary text-xl font-semibold">Audio Library</h3>
						<p
							class="text-quinary-darker-10% text-sm"
						>

							@if (activeAudioGroupId) {
								Adding audio to selected group
							} @else {
								Search and add audio files to your collection
							}
						</p>
					</div>

					<!-- Search Input -->
					<div
						class="text-quinary group flex w-full"
					>
						<input
							type="text"
							placeholder="Search"
							class="input-default input-primary w-5/6 rounded-xl rounded-r-none border-r-0"
							[(ngModel)]="this.searchQuery"
							(ngModelChange)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
						/>
						<div
							class="bg-primary-lighter group-focus-within:bg-primary-lighter-10% flex w-1/6 cursor-pointer items-center justify-center rounded-r-lg"
							(click)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
							tabindex="0"
						>
							<i class="fa-solid fa-magnifying-glass"></i>
						</div>
					</div>

					<!-- Search Results Panel -->
					<div
						class="border-tertiary-lighter-20% bg-tertiary-lighter-10% flex h-full flex-col rounded-lg border p-4"
					>
						<div
							class="mb-4 flex items-center justify-between"
						>
							<div
								class="flex items-center gap-2"
							>
								<h4 class="text-quinary text-lg font-medium">Results</h4>
							</div>
						</div>

						<!-- Search Results List -->
						<pixli-scroll-view
							visibilityState="always"
							class="bg-tertiary-lighter-5% h-full w-full overflow-hidden rounded-lg py-2"
						>

							@if (isLoadingSearchedAudioCollections) {

								<!-- Loading Skeletons -->
								<div
									class="flex flex-col gap-3"
								>

									@for (skeleton of this.repeatSkeleton; track skeleton) {
										<app-audio-file-card-skeleton></app-audio-file-card-skeleton>
									}
								</div>
							} @else {

								@if (this.searchedAudioFiles && this.searchedAudioFiles.length > 0) {

									<!-- Audio Files List -->
									<div
										class="flex flex-col gap-2 px-2"
									>

										@for (audioFile of this.searchedAudioFiles; track audioFile) {
											<app-audio-file-card
												[audioFile]="audioFile"
											>

												@if (activeAudioGroupId) {
													<button
														toolbar-right
														tabindex="0"
														(click)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														(keyup.enter)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														class="btn-default btn-secondary flex aspect-square h-8 w-8 items-center justify-center rounded-full"
													>
														<i class="fa-solid fa-plus text-xs"></i>
													</button>
												}
											</app-audio-file-card>
										}
									</div>
								} @else {

									<!-- Empty State -->
									<div
										class="text-quinary-darker-20% flex h-full flex-col items-center justify-center gap-3 py-8"
									>
										<i class="fa-solid fa-music text-4xl opacity-50"></i>
										<p class="text-center">No audio files found</p>
										<p class="text-center text-sm">Try adjusting your search terms</p>
									</div>
								}
							}
						</pixli-scroll-view>
					</div>
				</div>
			</div>
		</div>
	</main>
}
```

# FILE: examples/formatted_angular.html
```html

@if (   this.audioCollectionEditService.editableCollection) {

	<!-- Main collection container -->
	<main
		class="flex rounded-xl bg-cover bg-center"
		[ngStyle]="this.imageUrl && !this.editingMode && this.audioCollectionEditService.editableCollection.imageId !== '00000000-0000-0000-0000-000000000000' ? {'background-image': this.getImageUrl()} : {}"
	>
		<div
			class="flex w-full flex-col justify-center rounded-xl p-4"
			[ngClass]="{'backdrop-brightness-50': this.imageUrl}"
		>

			<!-- Grid Layout Container -->
			<div
				class="grid h-fit w-full grid-cols-4 gap-8"
			>

				<!-- Collection Info Area (3/4 width - col-span-3) -->
				<div
					class="bg-tertiary-lighter-5% col-span-3 flex flex-col rounded-lg border-2 p-6"
					[ngClass]="{
						'border-accent':   this.hasNameChanges 
										|| this.hasDescriptionChanges 
										|| this.hasImageChanges,
						'border-quaternary': !(this.hasNameChanges || this.hasDescriptionChanges || this.hasImageChanges)
					}"
				>
					<div
						class="absolute right-4 top-4 z-10"
					>
						<button
							class="btn-default btn-accent"
							(click)="this.saveAllChanges()"
						>

							{{ "submitChanges" | translate }}
						</button>
					</div>

					<!-- Success Message -->

					@if (   showSaveSuccess) {
						<div
							class="absolute -top-3 right-4 z-10"
						>
							<div
								class="animate-fadeIn flex items-center gap-2 rounded-lg bg-green-500 px-4 py-1.5 text-sm text-white shadow-md"
							>
								<i class="fa-solid fa-check"></i>
								<span>Saved successfully</span>
							</div>
						</div>
					}
					<div
						class="flex w-full justify-between"
					>

						@if (   this.audioCollection) {
							<div
								class="flex items-center space-x-2"
							>
								<i
									class="fa-regular fa-heart cursor-pointer"
									[ngClass]="{'text-secondary': this.isCollectionLiked, 'text-quinary-darker-5%': !this.isCollectionLiked}"
									(click)="this.likeAudioCollection()"
									(keyup.enter)="this.likeAudioCollection()"
									tabindex="0"
								>
								</i>
								<p class="text-md text-quinary-darker-5% cursor-pointer">{{ this.audioCollection.likes }}</p>
							</div>
							<div
								class="flex items-center space-x-2"
							>
								<i class="fa-solid fa-eye text-quinary-darker-5%"></i>
								<p class="text-md cursor-pointer text-quinary-darker-5%">{{ this.totalPlays }}</p>
							</div>
						}
					</div>

					<!-- Top Row: Name and Controls -->
					<div
						class="mb-4 flex items-start justify-between"
					>

						<!-- Name Field -->
						<div
							class="flex-1 text-white"
						>

							@if (   this.isNameEditing) {
								<input
									#nameInput
									type="text"
									[ngModel]="this.audioCollectionEditService.editableCollection.name"
									(input)="handleNameChange($event)"
									class="w-full bg-transparent text-3xl font-semibold outline-none"
									placeholder="Enter collection name"
									(blur)="finishNameEdit()"
								/>
							} @else {
								<div
									class="relative flex items-center"
								>
									<h2
										class="cursor-pointer truncate text-3xl font-semibold"
										(click)="startNameEdit()"
										[title]="   this.audioCollectionEditService.editableCollection.name 
|| 'Enter collection name'"
										(keyup.enter)="startNameEdit()"
										tabindex="0"
									>

										{{ this.audioCollectionEditService.editableCollection.name || "Enter collection name" }}
									</h2>

									@if (   this.hasNameChanges) {
										<span
											class="text-accent ml-2"
											title="Name has unsaved changes"
										>
											<i class="fa-solid fa-circle-exclamation"></i>
										</span>
									}
								</div>
							}
						</div>

						<!-- Control Buttons -->
						<div
							class="flex space-x-4"
						>

							<!-- Play/Stop Button -->
							<button
								class="group flex flex-col items-center gap-1"
								tabindex="0"
								[disabled]="!this.audioPlayerService.loaded"
								[class.opacity-50]="!this.audioPlayerService.loaded"
								(click)="this.toggleAmbiencePreview()"
							>

								@if (   this.isAmbiencePreviewPlaying) {
									<i
										class="fa-regular fa-circle-stop text-3xl text-white transition-all group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Stop</span>
								} @else {
									<i
										class="fa-regular fa-circle-play text-3xl text-white transition-all group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Play</span>
								}
							</button>

							<!-- Generate Button -->
							<button
								(click)="this.generateAmbience()"
								class="group flex flex-col items-center gap-1"
								[disabled]="this.isLoadingAmbience"
								[class.opacity-50]="this.isLoadingAmbience"
							>
								<i
									class="fa-solid fa-wand-magic-sparkles text-3xl text-white transition-all group-hover:scale-110"
								>
								</i>
								<span class="text-xs text-gray-300">Generate</span>
							</button>

							<!-- Download Button - Only show when ambience is generated -->

							@if (   this.generatedAmibienceUrl && !this.isLoadingAmbience) {
								<button
									(click)="this.export()"
									class="group flex flex-col items-center gap-1"
								>
									<i
										class="fa-solid fa-download text-3xl text-white transition-all group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Download</span>
								</button>
							}
						</div>
					</div>

					<!-- Mid Row: Description -->
					<div
						class="relative mb-4 flex-1"
					>

						@if (   this.isDescriptionEditing) {
							<textarea
								#descriptionInput
								[ngModel]="this.audioCollectionEditService.editableCollection.description"
								(input)="handleDesciptionChange($event)"
								class="h-24 w-full resize-none overflow-y-auto bg-transparent text-lg text-gray-200 outline-none"
								placeholder="Enter collection description"
								(blur)="finishDescriptionEdit()"
							>
							</textarea>
						} @else {
							<div
								class="relative h-24 overflow-y-auto"
							>
								<p
									class="cursor-pointer break-words text-lg text-gray-200"
									(click)="startDescriptionEdit()"
									(keyup.enter)="startDescriptionEdit()"
									tabindex="0"
								>

									{{ this.audioCollectionEditService.editableCollection.description || "Enter collection description" }}
								</p>

								@if (   this.hasDescriptionChanges) {
									<span
										class="text-accent absolute right-0 top-0 mr-1"
										title="Description has unsaved changes"
									>
										<i class="fa-solid fa-circle-exclamation"></i>
									</span>
								}
							</div>
						}
					</div>

					<!-- Bottom Row: Waveform or Loading Area -->
					<div
						class="flex h-24 flex-col gap-3"
					>

						<!-- Loading Progress or Waveform -->

						@if (   this.isLoadingAmbience) {
							<div
								class="flex w-full flex-col gap-2"
							>
								<p class="text-lg text-white">{{ this.getLoadingStage() }}</p>
								<div
									class="h-2 w-full"
								>
									<app-progressbar
										[progress]="this.ambienceLoadingProgress / 5"
										[buffer]="this.ambienceLoadingProgress === 5 || this.ambienceLoadingProgress === 0 ? this.ambienceLoadingProgress / 5 : this.ambienceLoadingProgress / 5 + 0.1"
										[type]="'linear-buffer'"
									>
									</app-progressbar>
								</div>
							</div>
						} @else {
							<div
								class="relative"
							>

								<!-- Empty state when no ambience -->

								@if (   !this.generatedAmibienceUrl) {
									<div
										class="bg-tertiary-lighter-10% flex h-24 w-full flex-col items-center justify-center rounded-lg border border-dashed border-gray-600"
									>
										<p
											class="text-gray-400"
										>
											<i class="fa-solid fa-wave-square mr-2"></i>
											Click "Generate" to create your ambience
										</p>
									</div>
								}
							</div>
						}

						<!-- Actual waveform when generated -->
						<div
							#waveform
							class="h-24 w-full"
							[class.hidden]="!this.generatedAmibienceUrl"
						>
						</div>
					</div>
				</div>

				<!-- Image Upload Area (1/4 width - col-span-1) -->
				<div
					dropZone
					(filesDropped)="this.onImageFileDropped($event)"
					[ngClass]="{
						'border-accent': this.hasImageChanges,
						'border-quaternary': !this.hasImageChanges
					}"
					class="bg-tertiary-lighter-5% relative col-span-1 flex flex-col items-center justify-center rounded-lg border-2"
				>

					<!-- Current Image Preview or Upload UI -->
					<div
						class="flex h-full w-full flex-col items-center justify-center"
					>

						<!-- Show image if exists -->

						@if (
							   this.imageUrl && this.audioCollectionEditService.editableCollection.imageId !==
							"00000000-0000-0000-0000-000000000000"
						) {
							<div
								class="relative h-full w-full"
							>
								<img
									[src]="this.imageUrl"
									class="h-full w-full rounded-lg object-cover"
									alt="Collection image"
								/>
								<div
									class="absolute inset-0 flex flex-col items-center justify-center rounded-lg bg-black bg-opacity-50 opacity-0 transition-opacity hover:opacity-100"
								>
									<i class="fa-solid fa-cloud-arrow-up mb-2 text-xl text-white"></i>
									<p class="mb-2 text-sm text-white">Replace image</p>
									<label
										for="file"
										class="btn-default hover:bg-primary-lighter cursor-pointer rounded-full bg-primary px-3 py-1 text-xs text-white"
									>
										Choose
										file
									</label>
								</div>
							</div>
						} @else {

							<!-- Upload UI if no image -->
							<div
								class="flex h-full flex-col items-center justify-center p-4 text-white"
							>
								<i class="fa-solid fa-cloud-arrow-up text-danger mb-4 text-2xl"></i>
								<p class="mb-3 text-center text-sm">{{ "dragAndDropFiles" | translate }}</p>
								<div
									class="my-1 flex w-full items-center justify-center"
								>
									<hr class="border-quinary w-1/3" />
									<span class="text-quinary px-2 text-xs">or</span>
									<hr class="border-quinary w-1/3" />
								</div>
								<label
									for="file"
									class="btn-default btn-secondary mt-3 px-4 py-1.5 text-sm transition-colors"
								>
									Choose
									file
								</label>
								<p class="text-quinary mt-3 text-xs">Max size: 16MB</p>
							</div>
						}
					</div>
					<input
						(change)="this.handleImageFileUpload($event)"
						accept=".png,.jpg,.jpeg,.hvc"
						type="file"
						name="file"
						id="file"
						class="hidden"
					/>

					<!-- Mark as changed if needed -->

					@if (   this.hasImageChanges) {
						<div
							class="text-accent absolute right-2 top-2"
						>
							<i class="fa-solid fa-circle-exclamation"></i>
						</div>
					}
				</div>

				<!-- Timeline Panel (3/4 width - col-span-3) -->
				<div
					class="bg-tertiary-lighter-5% relative col-span-3 flex flex-col gap-8 rounded-lg border-2 p-6"
					[ngClass]="{
						'border-quaternary':   !this.audioCollectionEditService.changes['presets'] 
											|| !this.audioCollectionEditService.changes['audioList'] 
											|| !this.audioCollectionEditService.changes['timeline'],
						'border-accent':   this.audioCollectionEditService.changes['presets'] 
										&& this.audioCollectionEditService.changes['audioList'] 
										&& this.audioCollectionEditService.changes['timeline']
					}"
				>

					<!-- Main Panel Save Button (absolute position) -->

					@if (
						   this.audioCollectionEditService.changes["presets"] && this.audioCollectionEditService.changes["audioList"]
						&& this.audioCollectionEditService.changes["timeline"]
					) {
						<button
							(click)="saveAllChanges()"
							class="btn-default btn-accent absolute -top-3 right-3 z-10 flex items-center space-x-2 px-3 py-1.5 text-sm"
						>
							<i class="fa-solid fa-save mr-2"></i>
							<span>Save All Changes</span>
						</button>
					}

					<!-- Section Header -->
					<div
						class="border-tertiary-lighter-20% flex items-center justify-between border-b pb-4"
					>
						<div
							class="flex flex-col"
						>
							<h3 class="text-quinary text-xl font-semibold">Timeline & Audio Groups</h3>
							<p
								class="text-quinary-darker-10% text-sm"
							>
								Manage your video timeline and audio tracks
							</p>
						</div>
					</div>

					<!-- Audio Groups Panel -->
					<div
						class="bg-tertiary-lighter-10% relative flex flex-col gap-4 rounded-lg border p-4"
						[ngClass]="{
							'border-accent':   this.audioCollectionEditService.changes['presets'] 
											|| this.audioCollectionEditService.changes['audioList'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['presets'] 
														&& !this.audioCollectionEditService.changes['audioList']
						}"
					>

						<!-- Presets Component -->
						<app-presets
							[presets]="this.audioCollectionEditService.editableCollection.presets"
							[selectedPresetId]="this.audioCollectionEditService.selectedPreset!.id"
							[audioList]="this.audioCollectionEditService.editableCollection.audioList"
							[originalAudioCollection]="this.audioCollection"
							[canSave]="!this.hasAudiosCountChanges()"
							(presetAdd)="this.addPreset()"
							(presetSave)="this.savePresetChanges()"
						>
						</app-presets>

						<!-- Presets Save Button (absolute position) -->

						@if (
							   this.audioCollectionEditService.changes["presets"] &&
							!this.audioCollectionEditService.changes["audioList"]
						) {
							<button
								(click)="savePresetChanges()"
								class="btn-default btn-accent absolute right-4 top-4 flex items-center space-x-2 px-3 py-1.5 text-sm"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Preset Changes</span>
							</button>
						}

						<!-- Audio Groups Component -->
						<app-audio-groups
							[audioGroups]="this.audioCollectionEditService.editableCollection.audioList"
							[activeAudioGroupId]="activeAudioGroupId"
							(addAudioGroup)="addEmptyAudioGroup()"
							(saveAudioGroupChanges)="saveAudioGroupChanges()"
							(setActiveAudioGroup)="setActiveAudioGroup($event)"
							(audioVolumeChange)="handleAudioVolumeChange($event)"
							(audioMuteChange)="handleAudioMuteChange($event)"
							(audioPlaybackOffsetChange)="handleAudioPlaybackOffsetChange($event)"
							(audioProbabilityChange)="handleAudioProbabilityChange($event)"
							(deleteAudioGroup)="deleteAudioGroupFromCollection($event)"
							(deleteAudioFile)="deleteAudioFileFromCollection($event)"
							(nameChange)="handleGroupNameChange($event)"
							(audioGroupRefsChange)="handleAudioGroupRefsChange($event)"
						>
						</app-audio-groups>
					</div>
					<div
						class="bg-tertiary-lighter-10% relative flex flex-col gap-4 rounded-lg border p-4"
						[ngClass]="{
							'border-accent': this.audioCollectionEditService.changes['timeline'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['timeline']
						}"
					>

						<!-- Timeline Save Button (absolute position) -->

						@if (
							   this.audioCollectionEditService.changes["timeline"] &&
							!this.audioCollectionEditService.changes["presets"] &&
							!this.audioCollectionEditService.changes["audioList"]
						) {
							<button
								(click)="saveTimelineChanges()"
								class="btn-default btn-accent absolute -top-3 right-3 z-10 flex items-center space-x-2 px-3 py-1.5 text-sm"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Timeline Changes</span>
							</button>
						}

						<!-- Video Preview and Timeline Container -->
						<app-timeline
							(timelineChange)="handleTimelineChange($event)"
							[timeline]="this.audioCollectionEditService.editableCollection!.timeline"
							[timelineLength]="120000"
						>
						</app-timeline>
					</div>
					<button
						class="btn-default btn-accent"
						(click)="this.logChanges()"
					>
						Log Changes
					</button>
				</div>

				<!-- Audio Library Panel (1/4 width - col-span-1) -->
				<div
					class="border-quaternary bg-tertiary-lighter-5% col-span-1 flex flex-col gap-4 rounded-lg border-2 p-6"
				>

					<!-- Library Header -->
					<div
						class="flex flex-col gap-2"
					>
						<h3 class="text-quinary text-xl font-semibold">Audio Library</h3>
						<p
							class="text-quinary-darker-10% text-sm"
						>

							@if (   activeAudioGroupId) {
								Adding audio to selected group
							} @else {
								Search and add audio files to your collection
							}
						</p>
					</div>

					<!-- Search Input -->
					<div
						class="text-quinary group flex w-full"
					>
						<input
							type="text"
							placeholder="Search"
							class="input-default input-primary w-5/6 rounded-xl rounded-r-none border-r-0"
							[(ngModel)]="this.searchQuery"
							(ngModelChange)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
						/>
						<div
							class="bg-primary-lighter group-focus-within:bg-primary-lighter-10% flex w-1/6 cursor-pointer items-center justify-center rounded-r-lg"
							(click)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
							tabindex="0"
						>
							<i class="fa-solid fa-magnifying-glass"></i>
						</div>
					</div>

					<!-- Search Results Panel -->
					<div
						class="border-tertiary-lighter-20% bg-tertiary-lighter-10% flex h-full flex-col rounded-lg border p-4"
					>
						<div
							class="mb-4 flex items-center justify-between"
						>
							<div
								class="flex items-center gap-2"
							>
								<h4 class="text-quinary text-lg font-medium">Results</h4>
							</div>
						</div>

						<!-- Search Results List -->
						<pixli-scroll-view
							visibilityState="always"
							class="bg-tertiary-lighter-5% h-full w-full overflow-hidden rounded-lg py-2"
						>

							@if (   isLoadingSearchedAudioCollections) {

								<!-- Loading Skeletons -->
								<div
									class="flex flex-col gap-3"
								>

									@for (   skeleton of this.repeatSkeleton; track skeleton) {
										<app-audio-file-card-skeleton></app-audio-file-card-skeleton>
									}
								</div>
							} @else {

								@if (   this.searchedAudioFiles && this.searchedAudioFiles.length > 0) {

									<!-- Audio Files List -->
									<div
										class="flex flex-col gap-2 px-2"
									>

										@for (   audioFile of this.searchedAudioFiles; track audioFile) {
											<app-audio-file-card
												[audioFile]="audioFile"
											>

												@if (   activeAudioGroupId) {
													<button
														toolbar-right
														tabindex="0"
														(click)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														(keyup.enter)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														class="btn-default btn-secondary flex aspect-square h-8 w-8 items-center justify-center rounded-full"
													>
														<i class="fa-solid fa-plus text-xs"></i>
													</button>
												}
											</app-audio-file-card>
										}
									</div>
								} @else {

									<!-- Empty State -->
									<div
										class="text-quinary-darker-20% flex h-full flex-col items-center justify-center gap-3 py-8"
									>
										<i class="fa-solid fa-music text-4xl opacity-50"></i>
										<p class="text-center">No audio files found</p>
										<p class="text-center text-sm">Try adjusting your search terms</p>
									</div>
								}
							}
						</pixli-scroll-view>
					</div>
				</div>
			</div>
		</div>
	</main>
}
```

# FILE: examples/formatted_ideal.html
```html

@if (this.audioCollectionEditService.editableCollection) {
	<!-- Main collection container -->
	<main
		class="flex rounded-xl bg-cover bg-center"
		[ngStyle]="
			(
				   this.imageUrl
				&& !this.editingMode
				&& this.audioCollectionEditService.editableCollection.imageId !== '00000000-0000-0000-0000-000000000000'
			)
			? {'background-image': this.getImageUrl()}
			: {}"
	>
		<div
			class="
				flex 
				w-full 
				flex-col 
				justify-center 
				rounded-xl 
				p-4"
			[ngClass]="{'backdrop-brightness-50': this.imageUrl}"
		>
			<!-- Grid Layout Container -->
			<div
				class="grid h-fit w-full grid-cols-4 gap-8"
			>
				<!-- Collection Info Area (3/4 width - col-span-3) -->
				<div
					class="
						bg-tertiary-lighter-5% 
						col-span-3 
						flex 
						flex-col 
						rounded-lg 
						border-2 
						p-6"
					[ngClass]="{
						'border-accent':
							(
								   this.hasNameChanges
								|| this.hasDescriptionChanges
								|| this.hasImageChanges
							),
						'border-quaternary':
							(
								!(
									   this.hasNameChanges
									|| this.hasDescriptionChanges
									|| this.hasImageChanges
								)
							)
						}"
				>
					<div
						class="absolute right-4 top-4 z-10"
					>
						<button
							class="btn-default btn-accent"
							(click)="this.saveAllChanges()"
						>
							{{ 'submitChanges' | translate }}
						</button>
					</div>

					<!-- Success Message -->
					@if (showSaveSuccess) {
						<div
							class="absolute -top-3 right-4 z-10"
						>
							<div
								class="
									animate-fadeIn 
									flex 
									items-center 
									gap-2 
									rounded-lg 
									bg-green-500 
									px-4 
									py-1.5 
									text-sm 
									text-white 
									shadow-md"
							>
								<i class="fa-solid fa-check"></i>
								<span>Saved successfully</span>
							</div>
						</div>
					}

					<div
						class="flex w-full justify-between"
					>
						@if (this.audioCollection) {
							<div
								class="flex items-center space-x-2"
							>
								<i
									class="fa-regular fa-heart cursor-pointer"
									[ngClass]="{
										'text-secondary': this.isCollectionLiked,
										'text-quinary-darker-5%': !this.isCollectionLiked
										}"
									(click)="this.likeAudioCollection()"
									(keyup.enter)="this.likeAudioCollection()"
									tabindex="0"
								>
								</i>

								<p class="text-md text-quinary-darker-5% cursor-pointer">{{ this.audioCollection.likes }}</p>
							</div>

							<div
								class="flex items-center space-x-2"
							>
								<i class="fa-solid fa-eye text-quinary-darker-5%"></i>
								<p class="text-md cursor-pointer text-quinary-darker-5%">{{ this.totalPlays }}</p>
							</div>
						}
					</div>

					<!-- Top Row: Name and Controls -->
					<div
						class="mb-4 flex items-start justify-between"
					>
						<!-- Name Field -->
						<div
							class="flex-1 text-white"
						>
							@if (this.isNameEditing) {
								<input
									#nameInput
									type="text"
									[ngModel]="this.audioCollectionEditService.editableCollection.name"
									(input)="handleNameChange($event)"
									class="w-full bg-transparent text-3xl font-semibold outline-none"
									placeholder="Enter collection name"
									(blur)="finishNameEdit()"
								/>
							} @else {
								<div
									class="relative flex items-center"
								>
									<h2
										class="cursor-pointer truncate text-3xl font-semibold"
										(click)="startNameEdit()"
										[title]="   this.audioCollectionEditService.editableCollection.name 
|| 'Enter collection name'"
										(keyup.enter)="startNameEdit()"
										tabindex="0"
									>
										{{ this.audioCollectionEditService.editableCollection.name || 'Enter collection name' }}
									</h2>
									@if (this.hasNameChanges) {
										<span
											class="text-accent ml-2"
											title="Name has unsaved changes"
										>
											<i class="fa-solid fa-circle-exclamation"></i>
										</span>
									}
								</div>
							}
						</div>

						<!-- Control Buttons -->
						<div
							class="flex space-x-4"
						>
							<!-- Play/Stop Button -->
							<button
								class="group flex flex-col items-center gap-1"
								tabindex="0"
								[disabled]="!this.audioPlayerService.loaded"
								[class.opacity-50]="!this.audioPlayerService.loaded"
								(click)="this.toggleAmbiencePreview()"
							>
								@if (this.isAmbiencePreviewPlaying) {
									<i
										class="
											fa-regular 
											fa-circle-stop 
											text-3xl 
											text-white 
											transition-all 
											group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Stop</span>
								} @else {
									<i
										class="
											fa-regular 
											fa-circle-play 
											text-3xl 
											text-white 
											transition-all 
											group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Play</span>
								}
							</button>

							<!-- Generate Button -->
							<button
								(click)="this.generateAmbience()"
								class="group flex flex-col items-center gap-1"
								[disabled]="this.isLoadingAmbience"
								[class.opacity-50]="this.isLoadingAmbience"
							>
								<i
									class="
										fa-solid 
										fa-wand-magic-sparkles 
										text-3xl 
										text-white 
										transition-all 
										group-hover:scale-110"
								>
								</i>
								<span class="text-xs text-gray-300">Generate</span>
							</button>

							<!-- Download Button - Only show when ambience is generated -->
							@if (this.generatedAmibienceUrl && !this.isLoadingAmbience) {
								<button
									(click)="this.export()"
									class="group flex flex-col items-center gap-1"
								>
									<i
										class="
											fa-solid 
											fa-download 
											text-3xl 
											text-white 
											transition-all 
											group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Download</span>
								</button>
							}
						</div>
					</div>

					<!-- Mid Row: Description -->
					<div
						class="relative mb-4 flex-1"
					>
						@if (this.isDescriptionEditing) {
							<textarea
								#descriptionInput
								[ngModel]="this.audioCollectionEditService.editableCollection.description"
								(input)="handleDesciptionChange($event)"
								class="
									h-24 
									w-full 
									resize-none 
									overflow-y-auto 
									bg-transparent 
									text-lg 
									text-gray-200 
									outline-none"
								placeholder="Enter collection description"
								(blur)="finishDescriptionEdit()"
							>
							</textarea>
						} @else {
							<div
								class="relative h-24 overflow-y-auto"
							>
								<p
									class="cursor-pointer break-words text-lg text-gray-200"
									(click)="startDescriptionEdit()"
									(keyup.enter)="startDescriptionEdit()"
									tabindex="0"
								>
									{{ this.audioCollectionEditService.editableCollection.description || 'Enter collection description' }}
								</p>
								@if (this.hasDescriptionChanges) {
									<span
										class="text-accent absolute right-0 top-0 mr-1"
										title="Description has unsaved changes"
									>
										<i class="fa-solid fa-circle-exclamation"></i>
									</span>
								}
							</div>
						}
					</div>

					<!-- Bottom Row: Waveform or Loading Area -->
					<div
						class="flex h-24 flex-col gap-3"
					>
						<!-- Loading Progress or Waveform -->
						@if (this.isLoadingAmbience) {
							<div
								class="flex w-full flex-col gap-2"
							>
								<p class="text-lg text-white">{{ this.getLoadingStage() }}</p>
								<div
									class="h-2 w-full"
								>
									<app-progressbar
										[progress]="this.ambienceLoadingProgress / 5"
										[buffer]="
											(
												   this.ambienceLoadingProgress === 5
												|| this.ambienceLoadingProgress === 0
											)
											? this.ambienceLoadingProgress / 5
											: this.ambienceLoadingProgress / 5 + 0.1"
										[type]="'linear-buffer'"
									>
									</app-progressbar>
								</div>
							</div>
						} @else {
							<div
								class="relative"
							>
								<!-- Empty state when no ambience -->
								@if (!this.generatedAmibienceUrl) {
									<div
										class="
											bg-tertiary-lighter-10% 
											flex 
											h-24 
											w-full 
											flex-col 
											items-center 
											justify-center 
											rounded-lg 
											border 
											border-dashed 
											border-gray-600"
									>
										<p
											class="text-gray-400"
										>
											<i class="fa-solid fa-wave-square mr-2"></i>
											Click "Generate" to create your ambience
										</p>
									</div>
								}
							</div>
						}
						<!-- Actual waveform when generated -->
						<div
							#waveform
							class="h-24 w-full"
							[class.hidden]="!this.generatedAmibienceUrl"
						>
						</div>
					</div>
				</div>

				<!-- Image Upload Area (1/4 width - col-span-1) -->
				<div
					dropZone
					(filesDropped)="this.onImageFileDropped($event)"
					[ngClass]="{
						'border-accent': this.hasImageChanges,
						'border-quaternary': !this.hasImageChanges
						}"
					class="
						bg-tertiary-lighter-5% 
						relative 
						col-span-1 
						flex 
						flex-col 
						items-center 
						justify-center 
						rounded-lg 
						border-2"
				>
					<!-- Current Image Preview or Upload UI -->
					<div
						class="
							flex 
							h-full 
							w-full 
							flex-col 
							items-center 
							justify-center"
					>
						<!-- Show image if exists -->
						@if (this.imageUrl && this.audioCollectionEditService.editableCollection.imageId !==
							"00000000-0000-0000-0000-000000000000") {
							<div
								class="relative h-full w-full"
							>
								<img
									[src]="this.imageUrl"
									class="h-full w-full rounded-lg object-cover"
									alt="Collection image"
								/>
								<div
									class="
										absolute 
										inset-0 
										flex 
										flex-col 
										items-center 
										justify-center 
										rounded-lg 
										bg-black 
										bg-opacity-50 
										opacity-0 
										transition-opacity 
										hover:opacity-100"
								>
									<i class="fa-solid fa-cloud-arrow-up mb-2 text-xl text-white"></i>
									<p class="mb-2 text-sm text-white">Replace image</p>
									<label
										for="file"
										class="
											btn-default 
											hover:bg-primary-lighter 
											cursor-pointer 
											rounded-full 
											bg-primary 
											px-3 
											py-1 
											text-xs 
											text-white"
									>
										Choose
										file
									</label>
								</div>
							</div>
						} @else {
							<!-- Upload UI if no image -->
							<div
								class="
									flex 
									h-full 
									flex-col 
									items-center 
									justify-center 
									p-4 
									text-white"
							>
								<i class="fa-solid fa-cloud-arrow-up text-danger mb-4 text-2xl"></i>
								<p class="mb-3 text-center text-sm">{{ 'dragAndDropFiles' | translate }}</p>

								<div
									class="my-1 flex w-full items-center justify-center"
								>
									<hr class="border-quinary w-1/3" />
									<span class="text-quinary px-2 text-xs">or</span>
									<hr class="border-quinary w-1/3" />
								</div>

								<label
									for="file"
									class="
										btn-default 
										btn-secondary 
										mt-3 
										px-4 
										py-1.5 
										text-sm 
										transition-colors"
								>
									Choose
									file
								</label>
								<p class="text-quinary mt-3 text-xs">Max size: 16MB</p>
							</div>
						}
					</div>

					<input
						(change)="this.handleImageFileUpload($event)"
						accept=".png,.jpg,.jpeg,.hvc"
						type="file"
						name="file"
						id="file"
						class="hidden"
					/>

					<!-- Mark as changed if needed -->
					@if (this.hasImageChanges) {
						<div
							class="text-accent absolute right-2 top-2"
						>
							<i class="fa-solid fa-circle-exclamation"></i>
						</div>
					}
				</div>

				<!-- Timeline Panel (3/4 width - col-span-3) -->
				<div
					class="
						bg-tertiary-lighter-5% 
						relative 
						col-span-3 
						flex 
						flex-col 
						gap-8 
						rounded-lg 
						border-2 
						p-6"
					[ngClass]="{
						'border-quaternary':
							(
								   !this.audioCollectionEditService.changes['presets']
								|| !this.audioCollectionEditService.changes['audioList']
								|| !this.audioCollectionEditService.changes['timeline']
							),
						'border-accent':
							(
								   this.audioCollectionEditService.changes['presets']
								&& this.audioCollectionEditService.changes['audioList']
								&& this.audioCollectionEditService.changes['timeline']
							)
						}"
				>
					<!-- Main Panel Save Button (absolute position) -->
					@if (this.audioCollectionEditService.changes["presets"] && this.audioCollectionEditService.changes["audioList"]
						&& this.audioCollectionEditService.changes["timeline"]) {
						<button
							(click)="saveAllChanges()"
							class="
								btn-default 
								btn-accent 
								absolute 
								-top-3 
								right-3 
								z-10 
								flex 
								items-center 
								space-x-2 
								px-3 
								py-1.5 
								text-sm"
						>
							<i class="fa-solid fa-save mr-2"></i>
							<span>Save All Changes</span>
						</button>
					}

					<!-- Section Header -->
					<div
						class="
							border-tertiary-lighter-20% 
							flex 
							items-center 
							justify-between 
							border-b 
							pb-4"
					>
						<div
							class="flex flex-col"
						>
							<h3 class="text-quinary text-xl font-semibold">Timeline & Audio Groups</h3>
							<p
								class="text-quinary-darker-10% text-sm"
							>
								Manage your video timeline and audio tracks
							</p>
						</div>
					</div>

					<!-- Audio Groups Panel -->
					<div
						class="
							bg-tertiary-lighter-10% 
							relative 
							flex 
							flex-col 
							gap-4 
							rounded-lg 
							border 
							p-4"
						[ngClass]="{
							'border-accent':
								(
									   this.audioCollectionEditService.changes['presets']
									|| this.audioCollectionEditService.changes['audioList']
								),
							'border-tertiary-lighter-20%':
								(
									   !this.audioCollectionEditService.changes['presets']
									&& !this.audioCollectionEditService.changes['audioList']
								)
							}"
					>
						<!-- Presets Component -->
						<app-presets
							[presets]="this.audioCollectionEditService.editableCollection.presets"
							[selectedPresetId]="this.audioCollectionEditService.selectedPreset!.id"
							[audioList]="this.audioCollectionEditService.editableCollection.audioList"
							[originalAudioCollection]="this.audioCollection"
							[canSave]="!this.hasAudiosCountChanges()"
							(presetAdd)="this.addPreset()"
							(presetSave)="this.savePresetChanges()"
						>
						</app-presets>

						<!-- Presets Save Button (absolute position) -->
						@if (this.audioCollectionEditService.changes["presets"] &&
							!this.audioCollectionEditService.changes["audioList"]) {
							<button
								(click)="savePresetChanges()"
								class="
									btn-default 
									btn-accent 
									absolute 
									right-4 
									top-4 
									flex 
									items-center 
									space-x-2 
									px-3 
									py-1.5 
									text-sm"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Preset Changes</span>
							</button>
						}

						<!-- Audio Groups Component -->
						<app-audio-groups
							[audioGroups]="this.audioCollectionEditService.editableCollection.audioList"
							[activeAudioGroupId]="activeAudioGroupId"
							(addAudioGroup)="addEmptyAudioGroup()"
							(saveAudioGroupChanges)="saveAudioGroupChanges()"
							(setActiveAudioGroup)="setActiveAudioGroup($event)"
							(audioVolumeChange)="handleAudioVolumeChange($event)"
							(audioMuteChange)="handleAudioMuteChange($event)"
							(audioPlaybackOffsetChange)="handleAudioPlaybackOffsetChange($event)"
							(audioProbabilityChange)="handleAudioProbabilityChange($event)"
							(deleteAudioGroup)="deleteAudioGroupFromCollection($event)"
							(deleteAudioFile)="deleteAudioFileFromCollection($event)"
							(nameChange)="handleGroupNameChange($event)"
							(audioGroupRefsChange)="handleAudioGroupRefsChange($event)"
						>
						</app-audio-groups>
					</div>

					<div
						class="
							bg-tertiary-lighter-10% 
							relative 
							flex 
							flex-col 
							gap-4 
							rounded-lg 
							border 
							p-4"
						[ngClass]="{
							'border-accent': this.audioCollectionEditService.changes['timeline'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['timeline']
							}"
					>
						<!-- Timeline Save Button (absolute position) -->
						@if (this.audioCollectionEditService.changes["timeline"] &&
							!this.audioCollectionEditService.changes["presets"] && !this.audioCollectionEditService.changes["audioList"])
						{
							<button
								(click)="saveTimelineChanges()"
								class="
									btn-default 
									btn-accent 
									absolute 
									-top-3 
									right-3 
									z-10 
									flex 
									items-center 
									space-x-2 
									px-3 
									py-1.5 
									text-sm"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Timeline Changes</span>
							</button>
						}

						<!-- Video Preview and Timeline Container -->
						<app-timeline
							(timelineChange)="handleTimelineChange($event)"
							[timeline]="this.audioCollectionEditService.editableCollection!.timeline"
							[timelineLength]="120000"
						>
						</app-timeline>
					</div>

					<button
						class="btn-default btn-accent"
						(click)="this.logChanges()"
					>
						Log Changes
					</button>
				</div>

				<!-- Audio Library Panel (1/4 width - col-span-1) -->
				<div
					class="
						border-quaternary 
						bg-tertiary-lighter-5% 
						col-span-1 
						flex 
						flex-col 
						gap-4 
						rounded-lg 
						border-2 
						p-6"
				>
					<!-- Library Header -->
					<div
						class="flex flex-col gap-2"
					>
						<h3 class="text-quinary text-xl font-semibold">Audio Library</h3>
						<p
							class="text-quinary-darker-10% text-sm"
						>
							@if (activeAudioGroupId) {
            Adding audio to selected group
            } @else {
            Search and add audio files to your collection
            }
						</p>
					</div>

					<!-- Search Input -->
					<div
						class="text-quinary group flex w-full"
					>
						<input
							type="text"
							placeholder="Search"
							class="
								input-default 
								input-primary 
								w-5/6 
								rounded-xl 
								rounded-r-none 
								border-r-0"
							[(ngModel)]="this.searchQuery"
							(ngModelChange)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
						/>
						<div
							class="
								bg-primary-lighter 
								group-focus-within:bg-primary-lighter-10% 
								flex 
								w-1/6 
								cursor-pointer 
								items-center 
								justify-center 
								rounded-r-lg"
							(click)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
							tabindex="0"
						>
							<i class="fa-solid fa-magnifying-glass"></i>
						</div>
					</div>

					<!-- Search Results Panel -->
					<div
						class="
							border-tertiary-lighter-20% 
							bg-tertiary-lighter-10% 
							flex 
							h-full 
							flex-col 
							rounded-lg 
							border 
							p-4"
					>
						<div
							class="mb-4 flex items-center justify-between"
						>
							<div
								class="flex items-center gap-2"
							>
								<h4 class="text-quinary text-lg font-medium">Results</h4>
							</div>
						</div>

						<!-- Search Results List -->
						<pixli-scroll-view
							visibilityState="always"
							class="
								bg-tertiary-lighter-5% 
								h-full 
								w-full 
								overflow-hidden 
								rounded-lg 
								py-2"
						>
							@if (isLoadingSearchedAudioCollections) {
								<!-- Loading Skeletons -->
								<div
									class="flex flex-col gap-3"
								>
									@for (skeleton of this.repeatSkeleton; track skeleton) {
										<app-audio-file-card-skeleton></app-audio-file-card-skeleton>
									}
								</div>
							} @else {
								@if (this.searchedAudioFiles && this.searchedAudioFiles.length > 0) {
									<!-- Audio Files List -->
									<div
										class="flex flex-col gap-2 px-2"
									>
										@for (audioFile of this.searchedAudioFiles; track audioFile) {
											<app-audio-file-card
												[audioFile]="audioFile"
											>
												@if (activeAudioGroupId) {
													<button
														toolbar-right
														tabindex="0"
														(click)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														(keyup.enter)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														class="
															btn-default 
															btn-secondary 
															flex 
															aspect-square 
															h-8 
															w-8 
															items-center 
															justify-center 
															rounded-full"
													>
														<i class="fa-solid fa-plus text-xs"></i>
													</button>
												}
											</app-audio-file-card>
										}
									</div>
								} @else {
									<!-- Empty State -->
									<div
										class="
											text-quinary-darker-20% 
											flex 
											h-full 
											flex-col 
											items-center 
											justify-center 
											gap-3 
											py-8"
									>
										<i class="fa-solid fa-music text-4xl opacity-50"></i>
										<p class="text-center">No audio files found</p>
										<p class="text-center text-sm">Try adjusting your search terms</p>
									</div>
								}
							}
						</pixli-scroll-view>
					</div>
				</div>
			</div>
		</div>
	</main>
}
```

# FILE: examples/formatted_sample.html
```html

@if (this.audioCollectionEditService.editableCollection) {
	<!-- Main collection container -->
	<main
		class="flex rounded-xl bg-cover bg-center"
		[ngStyle]="
			(
				   this.imageUrl
				&& !this.editingMode
				&& this.audioCollectionEditService.editableCollection.imageId !== '00000000-0000-0000-0000-000000000000'
			)
			? {'background-image': this.getImageUrl()}
			: {}"
	>
		<div
			class="
				aboba
				
				w-full
				
				flex flex-col 
				justify-center
				
				rounded-xl p-4
			"
			[ngClass]="{'backdrop-brightness-50': this.imageUrl}"
		>
			<!-- Grid Layout Container -->
			<div
				class="grid h-fit w-full grid-cols-4 gap-8"
			>
				<!-- Collection Info Area (3/4 width - col-span-3) -->
				<div
					class="
						bg-tertiary-lighter-5%
						col-span-3
						flex
						flex-col
						rounded-lg
						border-2
						p-6
						"
					[ngClass]="{
						'border-accent':
							(
								   this.hasNameChanges
								|| this.hasDescriptionChanges
								|| this.hasImageChanges
							),
						'border-quaternary':
							(
								!(
									   this.hasNameChanges
									|| this.hasDescriptionChanges
									|| this.hasImageChanges
								)
							)
						}"
				>
					<div
						class="absolute right-4 top-4 z-10"
					>
						<button
							class="btn-default btn-accent"
							(click)="this.saveAllChanges()"
						>
							{{ 'submitChanges' | translate }}
						</button>
					</div>

					<!-- Success Message -->
					@if (showSaveSuccess) {
						<div
							class="absolute -top-3 right-4 z-10"
						>
							<div
								class="
									animate-fadeIn
									flex
									items-center
									gap-2
									rounded-lg
									bg-green-500
									px-4
									py-1.5
									text-sm
									text-white
									shadow-md
									"
							>
								<i class="fa-solid fa-check"></i>
								<span>Saved successfully</span>
							</div>
						</div>
					}

					<div
						class="flex w-full justify-between"
					>
						@if (this.audioCollection) {
							<div
								class="flex items-center space-x-2"
							>
								<i
									class="fa-regular fa-heart cursor-pointer"
									[ngClass]="{
										'text-secondary': this.isCollectionLiked,
										'text-quinary-darker-5%': !this.isCollectionLiked
										}"
									(click)="this.likeAudioCollection()"
									(keyup.enter)="this.likeAudioCollection()"
									tabindex="0"
								>
								</i>

								<p class="text-md text-quinary-darker-5% cursor-pointer">{{ this.audioCollection.likes }}</p>
							</div>

							<div
								class="flex items-center space-x-2"
							>
								<i class="fa-solid fa-eye text-quinary-darker-5%"></i>
								<p class="text-md cursor-pointer text-quinary-darker-5%">{{ this.totalPlays }}</p>
							</div>
						}
					</div>

					<!-- Top Row: Name and Controls -->
					<div
						class="mb-4 flex items-start justify-between"
					>
						<!-- Name Field -->
						<div
							class="flex-1 text-white"
						>
							@if (this.isNameEditing) {
								<input
									#nameInput
									type="text"
									[ngModel]="this.audioCollectionEditService.editableCollection.name"
									(input)="handleNameChange($event)"
									class="w-full bg-transparent text-3xl font-semibold outline-none"
									placeholder="Enter collection name"
									(blur)="finishNameEdit()"
								/>
							} @else {
								<div
									class="relative flex items-center"
								>
									<h2
										class="cursor-pointer truncate text-3xl font-semibold"
										(click)="startNameEdit()"
										[title]="this.audioCollectionEditService.editableCollection.name || 'Enter collection name'"
										(keyup.enter)="startNameEdit()"
										tabindex="0"
									>
										{{ this.audioCollectionEditService.editableCollection.name || 'Enter collection name' }}
									</h2>
									@if (this.hasNameChanges) {
										<span
											class="text-accent ml-2"
											title="Name has unsaved changes"
										>
											<i class="fa-solid fa-circle-exclamation"></i>
										</span>
									}
								</div>
							}
						</div>

						<!-- Control Buttons -->
						<div
							class="flex space-x-4"
						>
							<!-- Play/Stop Button -->
							<button
								class="group flex flex-col items-center gap-1"
								tabindex="0"
								[disabled]="!this.audioPlayerService.loaded"
								[class.opacity-50]="!this.audioPlayerService.loaded"
								(click)="this.toggleAmbiencePreview()"
							>
								@if (this.isAmbiencePreviewPlaying) {
									<i
										class="
											fa-regular
											fa-circle-stop
											text-3xl
											text-white
											transition-all
											group-hover:scale-110
											"
									>
									</i>
									<span class="text-xs text-gray-300">Stop</span>
								} @else {
									<i
										class="
											fa-regular
											fa-circle-play
											text-3xl
											text-white
											transition-all
											group-hover:scale-110
											"
									>
									</i>
									<span class="text-xs text-gray-300">Play</span>
								}
							</button>

							<!-- Generate Button -->
							<button
								(click)="this.generateAmbience()"
								class="group flex flex-col items-center gap-1"
								[disabled]="this.isLoadingAmbience"
								[class.opacity-50]="this.isLoadingAmbience"
							>
								<i
									class="
										fa-solid
										fa-wand-magic-sparkles
										text-3xl
										text-white
										transition-all
										group-hover:scale-110
										"
								>
								</i>
								<span class="text-xs text-gray-300">Generate</span>
							</button>

							<!-- Download Button - Only show when ambience is generated -->
							@if (this.generatedAmibienceUrl && !this.isLoadingAmbience) {
								<button
									(click)="this.export()"
									class="group flex flex-col items-center gap-1"
								>
									<i
										class="
											fa-solid
											fa-download
											text-3xl
											text-white
											transition-all
											group-hover:scale-110
											"
									>
									</i>
									<span class="text-xs text-gray-300">Download</span>
								</button>
							}
						</div>
					</div>

					<!-- Mid Row: Description -->
					<div
						class="relative mb-4 flex-1"
					>
						@if (this.isDescriptionEditing) {
							<textarea
								#descriptionInput
								[ngModel]="this.audioCollectionEditService.editableCollection.description"
								(input)="handleDesciptionChange($event)"
								class="
									h-24
									w-full
									resize-none
									overflow-y-auto
									bg-transparent
									text-lg
									text-gray-200
									outline-none
									"
								placeholder="Enter collection description"
								(blur)="finishDescriptionEdit()"
							>
							</textarea>
						} @else {
							<div
								class="relative h-24 overflow-y-auto"
							>
								<p
									class="cursor-pointer break-words text-lg text-gray-200"
									(click)="startDescriptionEdit()"
									(keyup.enter)="startDescriptionEdit()"
									tabindex="0"
								>
									{{ this.audioCollectionEditService.editableCollection.description || 'Enter collection description' }}
								</p>
								@if (this.hasDescriptionChanges) {
									<span
										class="text-accent absolute right-0 top-0 mr-1"
										title="Description has unsaved changes"
									>
										<i class="fa-solid fa-circle-exclamation"></i>
									</span>
								}
							</div>
						}
					</div>

					<!-- Bottom Row: Waveform or Loading Area -->
					<div
						class="flex h-24 flex-col gap-3"
					>
						<!-- Loading Progress or Waveform -->
						@if (this.isLoadingAmbience) {
							<div
								class="flex w-full flex-col gap-2"
							>
								<p class="text-lg text-white">{{ this.getLoadingStage() }}</p>
								<div
									class="h-2 w-full"
								>
									<app-progressbar
										[progress]="this.ambienceLoadingProgress / 5"
										[buffer]="
											(
												   this.ambienceLoadingProgress === 5
												|| this.ambienceLoadingProgress === 0
											)
											? this.ambienceLoadingProgress / 5
											: this.ambienceLoadingProgress / 5 + 0.1"
										[type]="'linear-buffer'"
									>
									</app-progressbar>
								</div>
							</div>
						} @else {
							<div
								class="relative"
							>
								<!-- Empty state when no ambience -->
								@if (!this.generatedAmibienceUrl) {
									<div
										class="
											bg-tertiary-lighter-10%
											flex
											h-24
											w-full
											flex-col
											items-center
											justify-center
											rounded-lg
											border
											border-dashed
											border-gray-600
											"
									>
										<p
											class="text-gray-400"
										>
											<i class="fa-solid fa-wave-square mr-2"></i>
											Click "Generate" to create your ambience
										</p>
									</div>
								}
							</div>
						}
						<!-- Actual waveform when generated -->
						<div
							#waveform
							class="h-24 w-full"
							[class.hidden]="!this.generatedAmibienceUrl"
						>
						</div>
					</div>
				</div>

				<!-- Image Upload Area (1/4 width - col-span-1) -->
				<div
					dropZone
					(filesDropped)="this.onImageFileDropped($event)"
					[ngClass]="{
						'border-accent': this.hasImageChanges,
						'border-quaternary': !this.hasImageChanges
						}"
					class="
						bg-tertiary-lighter-5%
						relative
						col-span-1
						flex
						flex-col
						items-center
						justify-center
						rounded-lg
						border-2
						"
				>
					<!-- Current Image Preview or Upload UI -->
					<div
						class="
							flex
							h-full
							w-full
							flex-col
							items-center
							justify-center
							"
					>
						<!-- Show image if exists -->
						@if (this.imageUrl && this.audioCollectionEditService.editableCollection.imageId !==
							"00000000-0000-0000-0000-000000000000") {
							<div
								class="relative h-full w-full"
							>
								<img
									[src]="this.imageUrl"
									class="h-full w-full rounded-lg object-cover"
									alt="Collection image"
								/>
								<div
									class="
										absolute
										inset-0
										flex
										flex-col
										items-center
										justify-center
										rounded-lg
										bg-black
										bg-opacity-50
										opacity-0
										transition-opacity
										hover:opacity-100
										"
								>
									<i class="fa-solid fa-cloud-arrow-up mb-2 text-xl text-white"></i>
									<p class="mb-2 text-sm text-white">Replace image</p>
									<label
										for="file"
										class="
											btn-default
											hover:bg-primary-lighter
											cursor-pointer
											rounded-full
											bg-primary
											px-3
											py-1
											text-xs
											text-white
											"
									>
										Choose
										file
									</label>
								</div>
							</div>
						} @else {
							<!-- Upload UI if no image -->
							<div
								class="
									flex
									h-full
									flex-col
									items-center
									justify-center
									p-4
									text-white
									"
							>
								<i class="fa-solid fa-cloud-arrow-up text-danger mb-4 text-2xl"></i>
								<p class="mb-3 text-center text-sm">{{ 'dragAndDropFiles' | translate }}</p>

								<div
									class="my-1 flex w-full items-center justify-center"
								>
									<hr class="border-quinary w-1/3" />
									<span class="text-quinary px-2 text-xs">or</span>
									<hr class="border-quinary w-1/3" />
								</div>

								<label
									for="file"
									class="
										btn-default
										btn-secondary
										mt-3
										px-4
										py-1.5
										text-sm
										transition-colors
										"
								>
									Choose
									file
								</label>
								<p class="text-quinary mt-3 text-xs">Max size: 16MB</p>
							</div>
						}
					</div>

					<input
						(change)="this.handleImageFileUpload($event)"
						accept=".png,.jpg,.jpeg,.hvc"
						type="file"
						name="file"
						id="file"
						class="hidden"
					/>

					<!-- Mark as changed if needed -->
					@if (this.hasImageChanges) {
						<div
							class="text-accent absolute right-2 top-2"
						>
							<i class="fa-solid fa-circle-exclamation"></i>
						</div>
					}
				</div>

				<!-- Timeline Panel (3/4 width - col-span-3) -->
				<div
					class="
						bg-tertiary-lighter-5%
						relative
						col-span-3
						flex
						flex-col
						gap-8
						rounded-lg
						border-2
						p-6
						"
					[ngClass]="{
						'border-quaternary':
							(
								   !this.audioCollectionEditService.changes['presets']
								|| !this.audioCollectionEditService.changes['audioList']
								|| !this.audioCollectionEditService.changes['timeline']
							),
						'border-accent':
							(
								   this.audioCollectionEditService.changes['presets']
								&& this.audioCollectionEditService.changes['audioList']
								&& this.audioCollectionEditService.changes['timeline']
							)
						}"
				>
					<!-- Main Panel Save Button (absolute position) -->
					@if (this.audioCollectionEditService.changes["presets"] && this.audioCollectionEditService.changes["audioList"]
						&& this.audioCollectionEditService.changes["timeline"]) {
						<button
							(click)="saveAllChanges()"
							class="
								btn-default
								btn-accent
								absolute
								-top-3
								right-3
								z-10
								flex
								items-center
								space-x-2
								px-3
								py-1.5
								text-sm
								"
						>
							<i class="fa-solid fa-save mr-2"></i>
							<span>Save All Changes</span>
						</button>
					}

					<!-- Section Header -->
					<div
						class="
							border-tertiary-lighter-20%
							flex
							items-center
							justify-between
							border-b
							pb-4
							"
					>
						<div
							class="flex flex-col"
						>
							<h3 class="text-quinary text-xl font-semibold">Timeline & Audio Groups</h3>
							<p
								class="text-quinary-darker-10% text-sm"
							>
								Manage your video timeline and audio tracks
							</p>
						</div>
					</div>

					<!-- Audio Groups Panel -->
					<div
						class="
							bg-tertiary-lighter-10%
							relative
							flex
							flex-col
							gap-4
							rounded-lg
							border
							p-4
							"
						[ngClass]="{
							'border-accent':
								(
									   this.audioCollectionEditService.changes['presets']
									|| this.audioCollectionEditService.changes['audioList']
								),
							'border-tertiary-lighter-20%':
								(
									   !this.audioCollectionEditService.changes['presets']
									&& !this.audioCollectionEditService.changes['audioList']
								)
							}"
					>
						<!-- Presets Component -->
						<app-presets
							[presets]="this.audioCollectionEditService.editableCollection.presets"
							[selectedPresetId]="this.audioCollectionEditService.selectedPreset!.id"
							[audioList]="this.audioCollectionEditService.editableCollection.audioList"
							[originalAudioCollection]="this.audioCollection"
							[canSave]="!this.hasAudiosCountChanges()"
							(presetAdd)="this.addPreset()"
							(presetSave)="this.savePresetChanges()"
						>
						</app-presets>

						<!-- Presets Save Button (absolute position) -->
						@if (this.audioCollectionEditService.changes["presets"] &&
							!this.audioCollectionEditService.changes["audioList"]) {
							<button
								(click)="savePresetChanges()"
								class="
									btn-default
									btn-accent
									absolute
									right-4
									top-4
									flex
									items-center
									space-x-2
									px-3
									py-1.5
									text-sm
									"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Preset Changes</span>
							</button>
						}

						<!-- Audio Groups Component -->
						<app-audio-groups
							[audioGroups]="this.audioCollectionEditService.editableCollection.audioList"
							[activeAudioGroupId]="activeAudioGroupId"
							(addAudioGroup)="addEmptyAudioGroup()"
							(saveAudioGroupChanges)="saveAudioGroupChanges()"
							(setActiveAudioGroup)="setActiveAudioGroup($event)"
							(audioVolumeChange)="handleAudioVolumeChange($event)"
							(audioMuteChange)="handleAudioMuteChange($event)"
							(audioPlaybackOffsetChange)="handleAudioPlaybackOffsetChange($event)"
							(audioProbabilityChange)="handleAudioProbabilityChange($event)"
							(deleteAudioGroup)="deleteAudioGroupFromCollection($event)"
							(deleteAudioFile)="deleteAudioFileFromCollection($event)"
							(nameChange)="handleGroupNameChange($event)"
							(audioGroupRefsChange)="handleAudioGroupRefsChange($event)"
						>
						</app-audio-groups>
					</div>

					<div
						class="
							bg-tertiary-lighter-10%
							relative
							flex
							flex-col
							gap-4
							rounded-lg
							border
							p-4
							"
						[ngClass]="{
							'border-accent': this.audioCollectionEditService.changes['timeline'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['timeline']
							}"
					>
						<!-- Timeline Save Button (absolute position) -->
						@if (this.audioCollectionEditService.changes["timeline"] &&
							!this.audioCollectionEditService.changes["presets"] && !this.audioCollectionEditService.changes["audioList"])
						{
							<button
								(click)="saveTimelineChanges()"
								class="
									btn-default
									btn-accent
									absolute
									-top-3
									right-3
									z-10
									flex
									items-center
									space-x-2
									px-3
									py-1.5
									text-sm
									"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Timeline Changes</span>
							</button>
						}

						<!-- Video Preview and Timeline Container -->
						<app-timeline
							(timelineChange)="handleTimelineChange($event)"
							[timeline]="this.audioCollectionEditService.editableCollection!.timeline"
							[timelineLength]="120000"
						>
						</app-timeline>
					</div>

					<button
						class="btn-default btn-accent"
						(click)="this.logChanges()"
					>
						Log Changes
					</button>
				</div>

				<!-- Audio Library Panel (1/4 width - col-span-1) -->
				<div
					class="
						border-quaternary
						bg-tertiary-lighter-5%
						col-span-1
						flex
						flex-col
						gap-4
						rounded-lg
						border-2
						p-6
						"
				>
					<!-- Library Header -->
					<div
						class="flex flex-col gap-2"
					>
						<h3 class="text-quinary text-xl font-semibold">Audio Library</h3>
						<p
							class="text-quinary-darker-10% text-sm"
						>
							@if (activeAudioGroupId) {
            Adding audio to selected group
            } @else {
            Search and add audio files to your collection
            }
						</p>
					</div>

					<!-- Search Input -->
					<div
						class="text-quinary group flex w-full"
					>
						<input
							type="text"
							placeholder="Search"
							class="
								input-default
								input-primary
								w-5/6
								rounded-xl
								rounded-r-none
								border-r-0
								"
							[(ngModel)]="this.searchQuery"
							(ngModelChange)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
						/>
						<div
							class="
								bg-primary-lighter
								group-focus-within:bg-primary-lighter-10%
								flex
								w-1/6
								cursor-pointer
								items-center
								justify-center
								rounded-r-lg
								"
							(click)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
							tabindex="0"
						>
							<i class="fa-solid fa-magnifying-glass"></i>
						</div>
					</div>

					<!-- Search Results Panel -->
					<div
						class="
							border-tertiary-lighter-20%
							bg-tertiary-lighter-10%
							flex
							h-full
							flex-col
							rounded-lg
							border
							p-4
							"
					>
						<div
							class="mb-4 flex items-center justify-between"
						>
							<div
								class="flex items-center gap-2"
							>
								<h4 class="text-quinary text-lg font-medium">Results</h4>
							</div>
						</div>

						<!-- Search Results List -->
						<pixli-scroll-view
							visibilityState="always"
							class="
								bg-tertiary-lighter-5%
								h-full
								w-full
								overflow-hidden
								rounded-lg
								py-2
								"
						>
							@if (isLoadingSearchedAudioCollections) {
								<!-- Loading Skeletons -->
								<div
									class="flex flex-col gap-3"
								>
									@for (skeleton of this.repeatSkeleton; track skeleton) {
										<app-audio-file-card-skeleton></app-audio-file-card-skeleton>
									}
								</div>
							} @else {
								@if (this.searchedAudioFiles && this.searchedAudioFiles.length > 0) {
									<!-- Audio Files List -->
									<div
										class="flex flex-col gap-2 px-2"
									>
										@for (audioFile of this.searchedAudioFiles; track audioFile) {
											<app-audio-file-card
												[audioFile]="audioFile"
											>
												@if (activeAudioGroupId) {
													<button
														toolbar-right
														tabindex="0"
														(click)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														(keyup.enter)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														class="
															btn-default
															btn-secondary
															flex
															aspect-square
															h-8
															w-8
															items-center
															justify-center
															rounded-full
															"
													>
														<i class="fa-solid fa-plus text-xs"></i>
													</button>
												}
											</app-audio-file-card>
										}
									</div>
								} @else {
									<!-- Empty State -->
									<div
										class="
											text-quinary-darker-20%
											flex
											h-full
											flex-col
											items-center
											justify-center
											gap-3
											py-8
											"
									>
										<i class="fa-solid fa-music text-4xl opacity-50"></i>
										<p class="text-center">No audio files found</p>
										<p class="text-center text-sm">Try adjusting your search terms</p>
									</div>
								}
							}
						</pixli-scroll-view>
					</div>
				</div>
			</div>
		</div>
	</main>
}
```

# FILE: examples/ideal.html
```html

@if (this.audioCollectionEditService.editableCollection) {

	<!-- Main collection container -->
	<main
		class="flex rounded-xl bg-cover bg-center"
		[ngStyle]="this.imageUrl && !this.editingMode && this.audioCollectionEditService.editableCollection.imageId !== '00000000-0000-0000-0000-000000000000' ? {'background-image': this.getImageUrl()} : {}"
	>
		<div
			class="flex w-full flex-col justify-center rounded-xl p-4"
			[ngClass]="{'backdrop-brightness-50': this.imageUrl}"
		>

			<!-- Grid Layout Container -->
			<div
				class="grid h-fit w-full grid-cols-4 gap-8"
			>

				<!-- Collection Info Area (3/4 width - col-span-3) -->
				<div
					class="bg-tertiary-lighter-5% col-span-3 flex flex-col rounded-lg border-2 p-6"
					[ngClass]="{
						'border-accent': 
							(
								   this.hasNameChanges
								|| this.hasDescriptionChanges
								|| this.hasImageChanges
							),
						'border-quaternary':
							(
								!(
										this.hasNameChanges
									||	this.hasDescriptionChanges
									||	this.hasImageChanges
								)
								|| this.hasImageChanges
							)
					}"
				>
					<div
						class="absolute right-4 top-4 z-10"
					>
						<button
							class="btn-default btn-accent"
							(click)="this.saveAllChanges()"
						>

							{{ "submitChanges" | translate }}
						</button>
					</div>

					<!-- Success Message -->

					@if (showSaveSuccess) {
						<div
							class="absolute -top-3 right-4 z-10"
						>
							<div
								class="animate-fadeIn flex items-center gap-2 rounded-lg bg-green-500 px-4 py-1.5 text-sm text-white shadow-md"
							>
								<i class="fa-solid fa-check"></i>
								<span>Saved successfully</span>
							</div>
						</div>
					}
					<div
						class="flex w-full justify-between"
					>

						@if (this.audioCollection) {
							<div
								class="flex items-center space-x-2"
							>
								<i
									class="fa-regular fa-heart cursor-pointer"
									[ngClass]="{'text-secondary': this.isCollectionLiked, 'text-quinary-darker-5%': !this.isCollectionLiked}"
									(click)="this.likeAudioCollection()"
									(keyup.enter)="this.likeAudioCollection()"
									tabindex="0"
								>
								</i>
								<p class="text-md text-quinary-darker-5% cursor-pointer">{{ this.audioCollection.likes }}</p>
							</div>
							<div
								class="flex items-center space-x-2"
							>
								<i class="fa-solid fa-eye text-quinary-darker-5%"></i>
								<p class="text-md cursor-pointer text-quinary-darker-5%">{{ this.totalPlays }}</p>
							</div>
						}
					</div>

					<!-- Top Row: Name and Controls -->
					<div
						class="mb-4 flex items-start justify-between"
					>

						<!-- Name Field -->
						<div
							class="flex-1 text-white"
						>

							@if (this.isNameEditing) {
								<input
									#nameInput
									type="text"
									[ngModel]="this.audioCollectionEditService.editableCollection.name"
									(input)="handleNameChange($event)"
									class="w-full bg-transparent text-3xl font-semibold outline-none"
									placeholder="Enter collection name"
									(blur)="finishNameEdit()"
								/>
							} @else {
								<div
									class="relative flex items-center"
								>
									<h2
										class="cursor-pointer truncate text-3xl font-semibold"
										(click)="startNameEdit()"
										[title]="this.audioCollectionEditService.editableCollection.name || 'Enter collection name'"
										(keyup.enter)="startNameEdit()"
										tabindex="0"
									>

										{{ this.audioCollectionEditService.editableCollection.name || "Enter collection name" }}
									</h2>

									@if (this.hasNameChanges) {
										<span
											class="text-accent ml-2"
											title="Name has unsaved changes"
										>
											<i class="fa-solid fa-circle-exclamation"></i>
										</span>
									}
								</div>
							}
						</div>

						<!-- Control Buttons -->
						<div
							class="flex space-x-4"
						>

							<!-- Play/Stop Button -->
							<button
								class="group flex flex-col items-center gap-1"
								tabindex="0"
								[disabled]="!this.audioPlayerService.loaded"
								[class.opacity-50]="!this.audioPlayerService.loaded"
								(click)="this.toggleAmbiencePreview()"
							>

								@if (this.isAmbiencePreviewPlaying) {
									<i
										class="fa-regular fa-circle-stop text-3xl text-white transition-all group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Stop</span>
								} @else {
									<i
										class="fa-regular fa-circle-play text-3xl text-white transition-all group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Play</span>
								}
							</button>

							<!-- Generate Button -->
							<button
								(click)="this.generateAmbience()"
								class="group flex flex-col items-center gap-1"
								[disabled]="this.isLoadingAmbience"
								[class.opacity-50]="this.isLoadingAmbience"
							>
								<i
									class="fa-solid fa-wand-magic-sparkles text-3xl text-white transition-all group-hover:scale-110"
								>
								</i>
								<span class="text-xs text-gray-300">Generate</span>
							</button>

							<!-- Download Button - Only show when ambience is generated -->

							@if (this.generatedAmibienceUrl && !this.isLoadingAmbience) {
								<button
									(click)="this.export()"
									class="group flex flex-col items-center gap-1"
								>
									<i
										class="fa-solid fa-download text-3xl text-white transition-all group-hover:scale-110"
									>
									</i>
									<span class="text-xs text-gray-300">Download</span>
								</button>
							}
						</div>
					</div>

					<!-- Mid Row: Description -->
					<div
						class="relative mb-4 flex-1"
					>

						@if (this.isDescriptionEditing) {
							<textarea
								#descriptionInput
								[ngModel]="this.audioCollectionEditService.editableCollection.description"
								(input)="handleDesciptionChange($event)"
								class="h-24 w-full resize-none overflow-y-auto bg-transparent text-lg text-gray-200 outline-none"
								placeholder="Enter collection description"
								(blur)="finishDescriptionEdit()"
							>
							</textarea>
						} @else {
							<div
								class="relative h-24 overflow-y-auto"
							>
								<p
									class="cursor-pointer break-words text-lg text-gray-200"
									(click)="startDescriptionEdit()"
									(keyup.enter)="startDescriptionEdit()"
									tabindex="0"
								>

									{{ this.audioCollectionEditService.editableCollection.description || "Enter collection description" }}
								</p>

								@if (this.hasDescriptionChanges) {
									<span
										class="text-accent absolute right-0 top-0 mr-1"
										title="Description has unsaved changes"
									>
										<i class="fa-solid fa-circle-exclamation"></i>
									</span>
								}
							</div>
						}
					</div>

					<!-- Bottom Row: Waveform or Loading Area -->
					<div
						class="flex h-24 flex-col gap-3"
					>

						<!-- Loading Progress or Waveform -->

						@if (this.isLoadingAmbience) {
							<div
								class="flex w-full flex-col gap-2"
							>
								<p class="text-lg text-white">{{ this.getLoadingStage() }}</p>
								<div
									class="h-2 w-full"
								>
									<app-progressbar
										[progress]="this.ambienceLoadingProgress / 5"
										[buffer]="this.ambienceLoadingProgress === 5 || this.ambienceLoadingProgress === 0 ? this.ambienceLoadingProgress / 5 : this.ambienceLoadingProgress / 5 + 0.1"
										[type]="'linear-buffer'"
									>
									</app-progressbar>
								</div>
							</div>
						} @else {
							<div
								class="relative"
							>

								<!-- Empty state when no ambience -->

								@if (!this.generatedAmibienceUrl) {
									<div
										class="bg-tertiary-lighter-10% flex h-24 w-full flex-col items-center justify-center rounded-lg border border-dashed border-gray-600"
									>
										<p
											class="text-gray-400"
										>
											<i class="fa-solid fa-wave-square mr-2"></i>
											Click "Generate" to create your ambience
										</p>
									</div>
								}
							</div>
						}

						<!-- Actual waveform when generated -->
						<div
							#waveform
							class="h-24 w-full"
							[class.hidden]="!this.generatedAmibienceUrl"
						>
						</div>
					</div>
				</div>

				<!-- Image Upload Area (1/4 width - col-span-1) -->
				<div
					dropZone
					(filesDropped)="this.onImageFileDropped($event)"
					[ngClass]="{
						'border-accent': this.hasImageChanges,
						'border-quaternary': !this.hasImageChanges
					}"
					class="bg-tertiary-lighter-5% relative col-span-1 flex flex-col items-center justify-center rounded-lg border-2"
				>

					<!-- Current Image Preview or Upload UI -->
					<div
						class="flex h-full w-full flex-col items-center justify-center"
					>

						<!-- Show image if exists -->
!!!!
						@if (
							   this.imageUrl 
							&& this.audioCollectionEditService.editableCollection.imageId !== "00000000-0000-0000-0000-000000000000"
						) {
							<div
								class="relative h-full w-full"
							>
								<img
									[src]="this.imageUrl"
									class="h-full w-full rounded-lg object-cover"
									alt="Collection image"
								/>
								<div
									class="absolute inset-0 flex flex-col items-center justify-center rounded-lg bg-black bg-opacity-50 opacity-0 transition-opacity hover:opacity-100"
								>
									<i class="fa-solid fa-cloud-arrow-up mb-2 text-xl text-white"></i>
									<p class="mb-2 text-sm text-white">Replace image</p>
									<label
										for="file"
										class="btn-default hover:bg-primary-lighter cursor-pointer rounded-full bg-primary px-3 py-1 text-xs text-white"
									>
										Choose
										file
									</label>
								</div>
							</div>
						} @else {

							<!-- Upload UI if no image -->
							<div
								class="flex h-full flex-col items-center justify-center p-4 text-white"
							>
								<i class="fa-solid fa-cloud-arrow-up text-danger mb-4 text-2xl"></i>
								<p class="mb-3 text-center text-sm">{{ "dragAndDropFiles" | translate }}</p>
								<div
									class="my-1 flex w-full items-center justify-center"
								>
									<hr class="border-quinary w-1/3" />
									<span class="text-quinary px-2 text-xs">or</span>
									<hr class="border-quinary w-1/3" />
								</div>
								<label
									for="file"
									class="btn-default btn-secondary mt-3 px-4 py-1.5 text-sm transition-colors"
								>
									Choose
									file
								</label>
								<p class="text-quinary mt-3 text-xs">Max size: 16MB</p>
							</div>
						}
					</div>
					<input
						(change)="this.handleImageFileUpload($event)"
						accept=".png,.jpg,.jpeg,.hvc"
						type="file"
						name="file"
						id="file"
						class="hidden"
					/>

					<!-- Mark as changed if needed -->

					@if (this.hasImageChanges) {
						<div
							class="text-accent absolute right-2 top-2"
						>
							<i class="fa-solid fa-circle-exclamation"></i>
						</div>
					}
				</div>

				<!-- Timeline Panel (3/4 width - col-span-3) -->
				<div
					class="
						bg-tertiary-lighter-5% 
						relative col-span-3 
						flex flex-col 
						gap-8 
						rounded-lg 
						border-2 
						p-6
					"
					[ngClass]="{
						'border-quaternary':   
							(
								   !this.audioCollectionEditService.changes['presets']
								|| !this.audioCollectionEditService.changes['audioList']
								|| !this.audioCollectionEditService.changes['timeline']
							),
						'border-accent': 
							( 
								   this.audioCollectionEditService.changes['presets']
								&& this.audioCollectionEditService.changes['audioList']
								&& this.audioCollectionEditService.changes['timeline']
							)
					}"
				>

					<!-- Main Panel Save Button (absolute position) -->

					@if (
						(
							   this.audioCollectionEditService.changes["presets"] 
							&& this.audioCollectionEditService.changes["audioList"]
							&& this.audioCollectionEditService.changes["timeline"]
						)
						|| this.aboba === "siuuu"
						|| (
							   this.maga === "2024"
							&& this.maga2 === "2025"
							&& this.maga3 === "2026?"
						)
					) {
						<button
							(click)="saveAllChanges()"
							class="btn-default btn-accent absolute -top-3 right-3 z-10 flex items-center space-x-2 px-3 py-1.5 text-sm"
						>
							<i class="fa-solid fa-save mr-2"></i>
							<span>Save All Changes</span>
						</button>
					}

					<!-- Section Header -->
					<div
						class="border-tertiary-lighter-20% flex items-center justify-between border-b pb-4"
					>
						<div
							class="flex flex-col"
						>
							<h3 class="text-quinary text-xl font-semibold">Timeline & Audio Groups</h3>
							<p
								class="text-quinary-darker-10% text-sm"
							>
								Manage your video timeline and audio tracks
							</p>
						</div>
					</div>

					<!-- Audio Groups Panel -->
					<div
						class="bg-tertiary-lighter-10% relative flex flex-col gap-4 rounded-lg border p-4"
						[ngClass]="{
							'border-accent': this.audioCollectionEditService.changes['presets'] || this.audioCollectionEditService.changes['audioList'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['presets'] && !this.audioCollectionEditService.changes['audioList']
						}"
					>

						<!-- Presets Component -->
						<app-presets
							[presets]="this.audioCollectionEditService.editableCollection.presets"
							[selectedPresetId]="this.audioCollectionEditService.selectedPreset!.id"
							[audioList]="this.audioCollectionEditService.editableCollection.audioList"
							[originalAudioCollection]="this.audioCollection"
							[canSave]="!this.hasAudiosCountChanges()"
							(presetAdd)="this.addPreset()"
							(presetSave)="this.savePresetChanges()"
						>
						</app-presets>

						<!-- Presets Save Button (absolute position) -->

						@if (
							this.audioCollectionEditService.changes["presets"] &&
							!this.audioCollectionEditService.changes["audioList"]
						) {
							<button
								(click)="savePresetChanges()"
								class="btn-default btn-accent absolute right-4 top-4 flex items-center space-x-2 px-3 py-1.5 text-sm"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Preset Changes</span>
							</button>
						}

						<!-- Audio Groups Component -->
						<app-audio-groups
							[audioGroups]="this.audioCollectionEditService.editableCollection.audioList"
							[activeAudioGroupId]="activeAudioGroupId"
							(addAudioGroup)="addEmptyAudioGroup()"
							(saveAudioGroupChanges)="saveAudioGroupChanges()"
							(setActiveAudioGroup)="setActiveAudioGroup($event)"
							(audioVolumeChange)="handleAudioVolumeChange($event)"
							(audioMuteChange)="handleAudioMuteChange($event)"
							(audioPlaybackOffsetChange)="handleAudioPlaybackOffsetChange($event)"
							(audioProbabilityChange)="handleAudioProbabilityChange($event)"
							(deleteAudioGroup)="deleteAudioGroupFromCollection($event)"
							(deleteAudioFile)="deleteAudioFileFromCollection($event)"
							(nameChange)="handleGroupNameChange($event)"
							(audioGroupRefsChange)="handleAudioGroupRefsChange($event)"
						>
						</app-audio-groups>
					</div>
					<div
						class="bg-tertiary-lighter-10% relative flex flex-col gap-4 rounded-lg border p-4"
						[ngClass]="{
							'border-accent': this.audioCollectionEditService.changes['timeline'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['timeline']
						}"
					>

						<!-- Timeline Save Button (absolute position) -->

						@if (
							this.audioCollectionEditService.changes["timeline"] &&
							!this.audioCollectionEditService.changes["presets"] &&
							!this.audioCollectionEditService.changes["audioList"]
						) {
							<button
								(click)="saveTimelineChanges()"
								class="btn-default btn-accent absolute -top-3 right-3 z-10 flex items-center space-x-2 px-3 py-1.5 text-sm"
							>
								<i class="fa-solid fa-save mr-1"></i>
								<span>Save Timeline Changes</span>
							</button>
						}

						<!-- Video Preview and Timeline Container -->
						<app-timeline
							(timelineChange)="handleTimelineChange($event)"
							[timeline]="this.audioCollectionEditService.editableCollection!.timeline"
							[timelineLength]="120000"
						>
						</app-timeline>
					</div>
					<button
						class="btn-default btn-accent"
						(click)="this.logChanges()"
					>
						Log Changes
					</button>
				</div>

				<!-- Audio Library Panel (1/4 width - col-span-1) -->
				<div
					class="border-quaternary bg-tertiary-lighter-5% col-span-1 flex flex-col gap-4 rounded-lg border-2 p-6"
				>

					<!-- Library Header -->
					<div
						class="flex flex-col gap-2"
					>
						<h3 class="text-quinary text-xl font-semibold">Audio Library</h3>
						<p
							class="text-quinary-darker-10% text-sm"
						>

							@if (activeAudioGroupId) {
								Adding audio to selected group
							} @else {
								Search and add audio files to your collection
							}
						</p>
					</div>

					<!-- Search Input -->
					<div
						class="text-quinary group flex w-full"
					>
						<input
							type="text"
							placeholder="Search"
							class="input-default input-primary w-5/6 rounded-xl rounded-r-none border-r-0"
							[(ngModel)]="this.searchQuery"
							(ngModelChange)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
						/>
						<div
							class="bg-primary-lighter group-focus-within:bg-primary-lighter-10% flex w-1/6 cursor-pointer items-center justify-center rounded-r-lg"
							(click)="this.initiateSearch()"
							(keyup.enter)="this.initiateSearch()"
							tabindex="0"
						>
							<i class="fa-solid fa-magnifying-glass"></i>
						</div>
					</div>

					<!-- Search Results Panel -->
					<div
						class="border-tertiary-lighter-20% bg-tertiary-lighter-10% flex h-full flex-col rounded-lg border p-4"
					>
						<div
							class="mb-4 flex items-center justify-between"
						>
							<div
								class="flex items-center gap-2"
							>
								<h4 class="text-quinary text-lg font-medium">Results</h4>
							</div>
						</div>

						<!-- Search Results List -->
						<pixli-scroll-view
							visibilityState="always"
							class="bg-tertiary-lighter-5% h-full w-full overflow-hidden rounded-lg py-2"
						>

							@if (isLoadingSearchedAudioCollections) {

								<!-- Loading Skeletons -->
								<div
									class="flex flex-col gap-3"
								>

									@for (skeleton of this.repeatSkeleton; track skeleton) {
										<app-audio-file-card-skeleton></app-audio-file-card-skeleton>
									}
								</div>
							} @else {

								@if (this.searchedAudioFiles && this.searchedAudioFiles.length > 0) {

									<!-- Audio Files List -->
									<div
										class="flex flex-col gap-2 px-2"
									>

										@for (audioFile of this.searchedAudioFiles; track audioFile) {
											<app-audio-file-card
												[audioFile]="audioFile"
											>

												@if (activeAudioGroupId) {
													<button
														toolbar-right
														tabindex="0"
														(click)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														(keyup.enter)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
														class="btn-default btn-secondary flex aspect-square h-8 w-8 items-center justify-center rounded-full"
													>
														<i class="fa-solid fa-plus text-xs"></i>
													</button>
												}
											</app-audio-file-card>
										}
									</div>
								} @else {

									<!-- Empty State -->
									<div
										class="text-quinary-darker-20% flex h-full flex-col items-center justify-center gap-3 py-8"
									>
										<i class="fa-solid fa-music text-4xl opacity-50"></i>
										<p class="text-center">No audio files found</p>
										<p class="text-center text-sm">Try adjusting your search terms</p>
									</div>
								}
							}
						</pixli-scroll-view>
					</div>
				</div>
			</div>
		</div>
	</main>
}
```

# FILE: examples/react_example.jsx
```jsx
import React, { useState } from 'react';

const ReactComponent = ({ title, items }) => {
  const [active, setActive] = useState(false);

  return (
<div 
     className="react-container"
              data-active={active}
  >
               <header     >
        <h1 
             className="title-header"
   >
      {title}
               </h1>
            <button 
                         onClick={() => setActive(!active)}
                        className={`toggle-btn ${active ? 'active' : ''}`}
                 >
   Toggle State
                 </button>
                    </header>

          <main>
    {items.length > 0 ? (
      <ul className="item-list">
        {items.map((item, index) => (
          <li key={item.id} className="list-item">
            <span>{item.name}</span>
            <button
              onClick={() => console.log('clicked', item.id)}
              disabled={!active}
            >
              Action
            </button>
          </li>
        ))}
      </ul>
    ) : (
      <div className="empty-state">
        <p>No items found.</p>
      </div>
    )}
          </main>
</div>
  );
};

export default ReactComponent;


```

# FILE: examples/react_formatted.jsx
```jsx
import React, { useState } from 'react';

const ReactComponent = ({ title, items }) => {
  const [active, setActive] = useState(false);

  return (

<div
	className="react-container"
	data-active={active}
>
	<header>
		<h1 className="title-header">{title}</h1>
		<button 
      onClick={() =>setActive(!active)} 
      className={`toggle-btn ${active ? 'active' : ''}`} 
    >Toggle State</button>
	</header>

	<main>{items.length > 0 ? (
      <ul className="item-list">
        {items.map((item, index) => (
          <li key={item.id} className="list-item">
            <span>{item.name}</span>
            <button
              onClick={() => console.log('clicked', item.id)}
              disabled={!active}
            >
              Action
            </button>
          </li>
        ))}
      </ul>
    ) : (
      <div className="empty-state">
        <p>No items found.</p>
      </div>
    )}</main>
</div>
);
};

export default ReactComponent;


```

# FILE: examples/sample.html
```html
@if (this.audioCollectionEditService.editableCollection) {
<!-- Main collection container -->
<main class="flex rounded-xl bg-cover bg-center"
  [ngStyle]="this.imageUrl && !this.editingMode && this.audioCollectionEditService.editableCollection.imageId !== '00000000-0000-0000-0000-000000000000' ? {'background-image': this.getImageUrl()} : {}">
  <div class="flex w-full flex-col justify-center rounded-xl p-4" [ngClass]="{'backdrop-brightness-50': this.imageUrl}">
    <!-- Grid Layout Container -->
    <div class="grid h-fit w-full grid-cols-4 gap-8">
      <!-- Collection Info Area (3/4 width - col-span-3) -->
      <div class="bg-tertiary-lighter-5% col-span-3 flex flex-col rounded-lg border-2 p-6" [ngClass]="{
						'border-accent': this.hasNameChanges || this.hasDescriptionChanges || this.hasImageChanges,
						'border-quaternary': !(this.hasNameChanges || this.hasDescriptionChanges || this.hasImageChanges)
					}">
        <div class="absolute right-4 top-4 z-10">
          <button class="btn-default btn-accent" (click)="this.saveAllChanges()">
            {{ 'submitChanges' | translate }}
          </button>
        </div>

        <!-- Success Message -->
        @if (showSaveSuccess) {
        <div class="absolute -top-3 right-4 z-10">
          <div
            class="animate-fadeIn flex items-center gap-2 rounded-lg bg-green-500 px-4 py-1.5 text-sm text-white shadow-md">
            <i class="fa-solid fa-check"></i>
            <span>Saved successfully</span>
          </div>
        </div>
        }

        <div class="flex w-full justify-between">
          @if (this.audioCollection) {
          <div class="flex items-center space-x-2">
            <i class="fa-regular fa-heart cursor-pointer"
              [ngClass]="{'text-secondary': this.isCollectionLiked, 'text-quinary-darker-5%': !this.isCollectionLiked}"
              (click)="this.likeAudioCollection()" (keyup.enter)="this.likeAudioCollection()" tabindex="0">
            </i>

            <p class="text-md text-quinary-darker-5% cursor-pointer">
              {{ this.audioCollection.likes }}
            </p>
          </div>

          <div class="flex items-center space-x-2">
            <i class="fa-solid fa-eye text-quinary-darker-5%"></i>
            <p class="text-md cursor-pointer text-quinary-darker-5%">
              {{ this.totalPlays }}
            </p>
          </div>
          }
        </div>

        <!-- Top Row: Name and Controls -->
        <div class="mb-4 flex items-start justify-between">
          <!-- Name Field -->
          <div class="flex-1 text-white">
            @if (this.isNameEditing) {
            <input #nameInput type="text" [ngModel]="this.audioCollectionEditService.editableCollection.name"
              (input)="handleNameChange($event)" class="w-full bg-transparent text-3xl font-semibold outline-none"
              placeholder="Enter collection name" (blur)="finishNameEdit()" />
            } @else {
            <div class="relative flex items-center">
              <h2 class="cursor-pointer truncate text-3xl font-semibold" (click)="startNameEdit()"
                [title]="this.audioCollectionEditService.editableCollection.name || 'Enter collection name'"
                (keyup.enter)="startNameEdit()" tabindex="0">
                {{ this.audioCollectionEditService.editableCollection.name || 'Enter collection name' }}
              </h2>
              @if (this.hasNameChanges) {
              <span class="text-accent ml-2" title="Name has unsaved changes">
                <i class="fa-solid fa-circle-exclamation"></i>
              </span>
              }
            </div>
            }
          </div>

          <!-- Control Buttons -->
          <div class="flex space-x-4">
            <!-- Play/Stop Button -->
            <button class="group flex flex-col items-center gap-1" tabindex="0"
              [disabled]="!this.audioPlayerService.loaded" [class.opacity-50]="!this.audioPlayerService.loaded"
              (click)="this.toggleAmbiencePreview()">
              @if (this.isAmbiencePreviewPlaying) {
              <i class="fa-regular fa-circle-stop text-3xl text-white transition-all group-hover:scale-110"></i>
              <span class="text-xs text-gray-300">Stop</span>
              } @else {
              <i class="fa-regular fa-circle-play text-3xl text-white transition-all group-hover:scale-110"></i>
              <span class="text-xs text-gray-300">Play</span>
              }
            </button>

            <!-- Generate Button -->
            <button (click)="this.generateAmbience()" class="group flex flex-col items-center gap-1"
              [disabled]="this.isLoadingAmbience" [class.opacity-50]="this.isLoadingAmbience">
              <i class="fa-solid fa-wand-magic-sparkles text-3xl text-white transition-all group-hover:scale-110"></i>
              <span class="text-xs text-gray-300">Generate</span>
            </button>

            <!-- Download Button - Only show when ambience is generated -->
            @if (this.generatedAmibienceUrl && !this.isLoadingAmbience) {
            <button (click)="this.export()" class="group flex flex-col items-center gap-1">
              <i class="fa-solid fa-download text-3xl text-white transition-all group-hover:scale-110"></i>
              <span class="text-xs text-gray-300">Download</span>
            </button>
            }
          </div>
        </div>

        <!-- Mid Row: Description -->
        <div class="relative mb-4 flex-1">
          @if (this.isDescriptionEditing) {
          <textarea #descriptionInput [ngModel]="this.audioCollectionEditService.editableCollection.description"
            (input)="handleDesciptionChange($event)"
            class="h-24 w-full resize-none overflow-y-auto bg-transparent text-lg text-gray-200 outline-none"
            placeholder="Enter collection description" (blur)="finishDescriptionEdit()"></textarea>
          } @else {
          <div class="relative h-24 overflow-y-auto">
            <p class="cursor-pointer break-words text-lg text-gray-200" (click)="startDescriptionEdit()"
              (keyup.enter)="startDescriptionEdit()" tabindex="0">
              {{ this.audioCollectionEditService.editableCollection.description || 'Enter collection description' }}
            </p>
            @if (this.hasDescriptionChanges) {
            <span class="text-accent absolute right-0 top-0 mr-1" title="Description has unsaved changes">
              <i class="fa-solid fa-circle-exclamation"></i>
            </span>
            }
          </div>
          }
        </div>

        <!-- Bottom Row: Waveform or Loading Area -->
        <div class="flex h-24 flex-col gap-3">
          <!-- Loading Progress or Waveform -->
          @if (this.isLoadingAmbience) {
          <div class="flex w-full flex-col gap-2">
            <p class="text-lg text-white">{{ this.getLoadingStage() }}</p>
            <div class="h-2 w-full">
              <app-progressbar [progress]="this.ambienceLoadingProgress / 5"
                [buffer]="this.ambienceLoadingProgress === 5 || this.ambienceLoadingProgress === 0 ? this.ambienceLoadingProgress / 5 : this.ambienceLoadingProgress / 5 + 0.1"
                [type]="'linear-buffer'"></app-progressbar>
            </div>
          </div>
          } @else {
          <div class="relative">
            <!-- Empty state when no ambience -->
            @if (!this.generatedAmibienceUrl) {
            <div
              class="bg-tertiary-lighter-10% flex h-24 w-full flex-col items-center justify-center rounded-lg border border-dashed border-gray-600">
              <p class="text-gray-400">
                <i class="fa-solid fa-wave-square mr-2"></i>
                Click "Generate" to create your ambience
              </p>
            </div>
            }
          </div>
          }
          <!-- Actual waveform when generated -->
          <div #waveform class="h-24 w-full" [class.hidden]="!this.generatedAmibienceUrl"></div>
        </div>
      </div>

      <!-- Image Upload Area (1/4 width - col-span-1) -->
      <div dropZone (filesDropped)="this.onImageFileDropped($event)" [ngClass]="{
						'border-accent': this.hasImageChanges,
						'border-quaternary': !this.hasImageChanges
					}"
        class="bg-tertiary-lighter-5% relative col-span-1 flex flex-col items-center justify-center rounded-lg border-2">
        <!-- Current Image Preview or Upload UI -->
        <div class="flex h-full w-full flex-col items-center justify-center">
          <!-- Show image if exists -->
          @if (this.imageUrl && this.audioCollectionEditService.editableCollection.imageId !==
          '00000000-0000-0000-0000-000000000000') {
          <div class="relative h-full w-full">
            <img [src]="this.imageUrl" class="h-full w-full rounded-lg object-cover" alt="Collection image" />
            <div
              class="absolute inset-0 flex flex-col items-center justify-center rounded-lg bg-black bg-opacity-50 opacity-0 transition-opacity hover:opacity-100">
              <i class="fa-solid fa-cloud-arrow-up mb-2 text-xl text-white"></i>
              <p class="mb-2 text-sm text-white">Replace image</p>
              <label for="file"
                class="btn-default hover:bg-primary-lighter cursor-pointer rounded-full bg-primary px-3 py-1 text-xs text-white">Choose
                file</label>
            </div>
          </div>
          } @else {
          <!-- Upload UI if no image -->
          <div class="flex h-full flex-col items-center justify-center p-4 text-white">
            <i class="fa-solid fa-cloud-arrow-up text-danger mb-4 text-2xl"></i>
            <p class="mb-3 text-center text-sm">{{ 'dragAndDropFiles' | translate }}</p>

            <div class="my-1 flex w-full items-center justify-center">
              <hr class="border-quinary w-1/3" />
              <span class="text-quinary px-2 text-xs">or</span>
              <hr class="border-quinary w-1/3" />
            </div>

            <label for="file" class="btn-default btn-secondary mt-3 px-4 py-1.5 text-sm transition-colors">Choose
              file</label>
            <p class="text-quinary mt-3 text-xs">Max size: 16MB</p>
          </div>
          }
        </div>

        <input (change)="this.handleImageFileUpload($event)" accept=".png,.jpg,.jpeg,.hvc" type="file" name="file"
          id="file" class="hidden" />

        <!-- Mark as changed if needed -->
        @if (this.hasImageChanges) {
        <div class="text-accent absolute right-2 top-2">
          <i class="fa-solid fa-circle-exclamation"></i>
        </div>
        }
      </div>

      <!-- Timeline Panel (3/4 width - col-span-3) -->
      <div class="bg-tertiary-lighter-5% relative col-span-3 flex flex-col gap-8 rounded-lg border-2 p-6" [ngClass]="{
						'border-quaternary': !this.audioCollectionEditService.changes['presets'] || !this.audioCollectionEditService.changes['audioList'] || !this.audioCollectionEditService.changes['timeline'],
						'border-accent': this.audioCollectionEditService.changes['presets'] && this.audioCollectionEditService.changes['audioList'] && this.audioCollectionEditService.changes['timeline']
					}">
        <!-- Main Panel Save Button (absolute position) -->
        @if (this.audioCollectionEditService.changes['presets'] && this.audioCollectionEditService.changes['audioList']
        && this.audioCollectionEditService.changes['timeline']) {
        <button (click)="saveAllChanges()"
          class="btn-default btn-accent absolute -top-3 right-3 z-10 flex items-center space-x-2 px-3 py-1.5 text-sm">
          <i class="fa-solid fa-save mr-2"></i>
          <span>Save All Changes</span>
        </button>
        }

        <!-- Section Header -->
        <div class="border-tertiary-lighter-20% flex items-center justify-between border-b pb-4">
          <div class="flex flex-col">
            <h3 class="text-quinary text-xl font-semibold">Timeline & Audio Groups</h3>
            <p class="text-quinary-darker-10% text-sm">Manage your video timeline and audio tracks</p>
          </div>
        </div>

        <!-- Audio Groups Panel -->
        <div class="bg-tertiary-lighter-10% relative flex flex-col gap-4 rounded-lg border p-4" [ngClass]="{
							'border-accent': this.audioCollectionEditService.changes['presets'] || this.audioCollectionEditService.changes['audioList'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['presets'] && !this.audioCollectionEditService.changes['audioList']
						}">
          <!-- Presets Component -->
          <app-presets [presets]="this.audioCollectionEditService.editableCollection.presets"
            [selectedPresetId]="this.audioCollectionEditService.selectedPreset!.id"
            [audioList]="this.audioCollectionEditService.editableCollection.audioList"
            [originalAudioCollection]="this.audioCollection" [canSave]="!this.hasAudiosCountChanges()"
            (presetAdd)="this.addPreset()" (presetSave)="this.savePresetChanges()"></app-presets>

          <!-- Presets Save Button (absolute position) -->
          @if (this.audioCollectionEditService.changes['presets'] &&
          !this.audioCollectionEditService.changes['audioList']) {
          <button (click)="savePresetChanges()"
            class="btn-default btn-accent absolute right-4 top-4 flex items-center space-x-2 px-3 py-1.5 text-sm">
            <i class="fa-solid fa-save mr-1"></i>
            <span>Save Preset Changes</span>
          </button>
          }

          <!-- Audio Groups Component -->
          <app-audio-groups [audioGroups]="this.audioCollectionEditService.editableCollection.audioList"
            [activeAudioGroupId]="activeAudioGroupId" (addAudioGroup)="addEmptyAudioGroup()"
            (saveAudioGroupChanges)="saveAudioGroupChanges()" (setActiveAudioGroup)="setActiveAudioGroup($event)"
            (audioVolumeChange)="handleAudioVolumeChange($event)" (audioMuteChange)="handleAudioMuteChange($event)"
            (audioPlaybackOffsetChange)="handleAudioPlaybackOffsetChange($event)"
            (audioProbabilityChange)="handleAudioProbabilityChange($event)"
            (deleteAudioGroup)="deleteAudioGroupFromCollection($event)"
            (deleteAudioFile)="deleteAudioFileFromCollection($event)" (nameChange)="handleGroupNameChange($event)"
            (audioGroupRefsChange)="handleAudioGroupRefsChange($event)"></app-audio-groups>
        </div>

        <div class="bg-tertiary-lighter-10% relative flex flex-col gap-4 rounded-lg border p-4" [ngClass]="{
							'border-accent': this.audioCollectionEditService.changes['timeline'],
							'border-tertiary-lighter-20%': !this.audioCollectionEditService.changes['timeline']
						}">
          <!-- Timeline Save Button (absolute position) -->
          @if (this.audioCollectionEditService.changes['timeline'] &&
          !this.audioCollectionEditService.changes['presets'] && !this.audioCollectionEditService.changes['audioList'])
          {
          <button (click)="saveTimelineChanges()"
            class="btn-default btn-accent absolute -top-3 right-3 z-10 flex items-center space-x-2 px-3 py-1.5 text-sm">
            <i class="fa-solid fa-save mr-1"></i>
            <span>Save Timeline Changes</span>
          </button>
          }

          <!-- Video Preview and Timeline Container -->
          <app-timeline (timelineChange)="handleTimelineChange($event)"
            [timeline]="this.audioCollectionEditService.editableCollection!.timeline" [timelineLength]="120000">
          </app-timeline>
        </div>

        <button class="btn-default btn-accent" (click)="this.logChanges()">
          Log Changes
        </button>
      </div>

      <!-- Audio Library Panel (1/4 width - col-span-1) -->
      <div class="border-quaternary bg-tertiary-lighter-5% col-span-1 flex flex-col gap-4 rounded-lg border-2 p-6">
        <!-- Library Header -->
        <div class="flex flex-col gap-2">
          <h3 class="text-quinary text-xl font-semibold">Audio Library</h3>
          <p class="text-quinary-darker-10% text-sm">
            @if (activeAudioGroupId) {
            Adding audio to selected group
            } @else {
            Search and add audio files to your collection
            }
          </p>
        </div>

        <!-- Search Input -->
        <div class="text-quinary group flex w-full">
          <input type="text" placeholder="Search"
            class="input-default input-primary w-5/6 rounded-xl rounded-r-none border-r-0"
            [(ngModel)]="this.searchQuery" (ngModelChange)="this.initiateSearch()"
            (keyup.enter)="this.initiateSearch()" />
          <div
            class="bg-primary-lighter group-focus-within:bg-primary-lighter-10% flex w-1/6 cursor-pointer items-center justify-center rounded-r-lg"
            (click)="this.initiateSearch()" (keyup.enter)="this.initiateSearch()" tabindex="0">
            <i class="fa-solid fa-magnifying-glass"></i>
          </div>
        </div>

        <!-- Search Results Panel -->
        <div class="border-tertiary-lighter-20% bg-tertiary-lighter-10% flex h-full flex-col rounded-lg border p-4">
          <div class="mb-4 flex items-center justify-between">
            <div class="flex items-center gap-2">
              <h4 class="text-quinary text-lg font-medium">Results</h4>
            </div>
          </div>

          <!-- Search Results List -->
          <pixli-scroll-view visibilityState="always"
            class="bg-tertiary-lighter-5% h-full w-full overflow-hidden rounded-lg py-2">
            @if (isLoadingSearchedAudioCollections) {
            <!-- Loading Skeletons -->
            <div class="flex flex-col gap-3">
              @for (skeleton of this.repeatSkeleton; track skeleton) {
              <app-audio-file-card-skeleton></app-audio-file-card-skeleton>
              }
            </div>
            } @else {
            @if (this.searchedAudioFiles && this.searchedAudioFiles.length > 0) {
            <!-- Audio Files List -->
            <div class="flex flex-col gap-2 px-2">
              @for (audioFile of this.searchedAudioFiles; track audioFile) {
              <app-audio-file-card [audioFile]="audioFile">
                @if (activeAudioGroupId) {
                <button toolbar-right tabindex="0"
                  (click)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
                  (keyup.enter)="this.addAudioToCollection(audioFile.audioMetadata.audioId, activeAudioGroupId)"
                  class="btn-default btn-secondary flex aspect-square h-8 w-8 items-center justify-center rounded-full">
                  <i class="fa-solid fa-plus text-xs"></i>
                </button>
                }
              </app-audio-file-card>
              }
            </div>
            } @else {
            <!-- Empty State -->
            <div class="text-quinary-darker-20% flex h-full flex-col items-center justify-center gap-3 py-8">
              <i class="fa-solid fa-music text-4xl opacity-50"></i>
              <p class="text-center">No audio files found</p>
              <p class="text-center text-sm">Try adjusting your search terms</p>
            </div>
            }
            }
          </pixli-scroll-view>
        </div>
      </div>
    </div>
  </div>
</main>
}
```

# FILE: examples/test_plain_formatted.html
```html

<!DOCTYPE
	html
>
	<html
		lang="en"
	>
		<head
		>
			<meta charset="UTF-8">
			<meta name="viewport" content="width=device-width, initial-scale=1.0">
			<title>Plain HTML Test</title>
			<link rel="stylesheet" href="styles.css">
		</head>
		<body
		>
			<header
				class="site-header"
			>
				<nav
					class="nav"
				>
					<a href="/" class="nav-logo">Home</a>
					<ul
						class="nav-links"
					>
						<li
						><a href="/about">About</a>
						</li>
						<li
						><a href="/blog">Blog</a>
						</li>
						<li
						><a href="/contact">Contact</a>
						</li>
					</ul>
				</nav>
			</header>

			<main
				class="container"
			>
				<section
					class="hero"
				>
					<h1 class="hero-title">Welcome to the site</h1>
					<p class="hero-subtitle">A simple, fast, clean website.</p>
					<a href="/get-started" class="btn btn-primary">Get Started</a>
				</section>

				<section
					class="features"
				>
					<article
						class="card"
					>
						<img src="icon1.svg" alt="Feature one" width="48" height="48">
						<h2>Fast</h2>
						<p>Loads in under a second on any connection.</p>
					</article>
					<article
						class="card"
					>
						<img src="icon2.svg" alt="Feature two" width="48" height="48">
						<h2>Accessible</h2>
						<p>Built with semantic HTML and ARIA labels.</p>
					</article>
					<article
						class="card"
					>
						<img src="icon3.svg" alt="Feature three" width="48" height="48">
						<h2>Open Source</h2>
						<p>Every line of code is public on GitHub.</p>
					</article>
				</section>
			</main>

			<footer
				class="site-footer"
			>
				<p>&copy; 2024 My Site. All rights reserved.</p>
			</footer>
		</body>
	</html>
	
```

# FILE: examples/vue_example.html
```html
<template
>
      <div 
      class="vue-container"
           :class="{ 'is-active': active }"    @click="toggleActive"
  >
          <header>
                      <h1      >{{ title }}</h1>
  </header>

<main
>
        <transition 
           name="fade"
                         mode="out-in"
>
                                                  <div
                                  v-if="items.length > 0"
         class="items-list"
            >
      <div 
        v-for="(item, index) in items"
                   :key="item.id"
                              class="item-card"
        @mouseover="hoverItem(index)"
>
            <span
   class="item-name"
               >{{ item.name }}</span>
            
            <button
   @click.prevent.stop="deleteItem(item.id)"
   class="btn delete-btn"
            >
                            <i 
      class="icon-trash"
                            ></i>
                            <span
  class="btn-text"
                            >Delete</span>
    </button>
                                              </div>
                                                  </div>
                                 <div 
        v-else
                                                  class="empty-state"
  >
               <p
>    No items available. </p>
          </div>
              </transition>
  </main>
      </div>
  </template>

<script setup>
import { ref } from 'vue';

const title = ref('Vue Feature Component');
const active = ref(false);
const items = ref([{ id: 1, name: 'Apple' }, { id: 2, name: 'Banana' }]);

const toggleActive = () => active.value = !active.value;
const hoverItem = (index) => console.log('Hovering:', index);
const deleteItem = (id) => items.value = items.value.filter(i => i.id !== id);
</script>

<style scoped>
.vue-container { padding: 1rem; }
.is-active { border: 2px solid green; }
.fade-enter-active, .fade-leave-active { transition: opacity 0.5s; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>

```

# FILE: examples/vue_formatted.html
```html

<template
>
	<div
		class="vue-container"
		:class="{ 'is-active': active }"
		@click="toggleActive"
	>
		<header
		>
			<h1>{{ title }}</h1>
		</header>

		<main
		>
			<transition
				name="fade"
				mode="out-in"
			>
				<div
					v-if="items.length > 0"
					class="items-list"
				>
					<div
						v-for="(item, index) in items"
						:key="item.id"
						class="item-card"
						@mouseover="hoverItem(index)"
					>
						<span class="item-name">{{ item.name }}</span>

						<button
							@click.prevent.stop="deleteItem(item.id)"
							class="btn delete-btn"
						>
							<i class="icon-trash"></i>
							<span class="btn-text">Delete</span>
						</button>
					</div>
				</div>
				<div
					v-else
					class="empty-state"
				>
					<p>No items available.</p>
				</div>
			</transition>
		</main>
	</div>
</template>

<script setup>
import { ref } from 'vue';

const title = ref('Vue Feature Component');
const active = ref(false);
const items = ref([{ id: 1, name: 'Apple' }, { id: 2, name: 'Banana' }]);

const toggleActive = () => active.value = !active.value;
const hoverItem = (index) => console.log('Hovering:', index);
const deleteItem = (id) => items.value = items.value.filter(i => i.id !== id);
</script>

<style scoped>
.vue-container { padding: 1rem; }
.is-active { border: 2px solid green; }
.fade-enter-active, .fade-leave-active { transition: opacity 0.5s; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>

```
