# PROJECT CONTEXT REPORT
Generated: 2026-04-17 23:07:10

## 1. PROJECT STRUCTURE
```text
fua-fua-format/
|-- crates/
|   |-- cli/
|   |   |-- npm/
|   |   |   |-- package.json
|   |   |   `-- run.js
|   |   |-- src/
|   |   |   `-- main.rs
|   |   `-- Cargo.toml
|   |-- core/
|   |   |-- src/
|   |   |   |-- config.rs
|   |   |   |-- formatter.rs
|   |   |   |-- lexer.rs
|   |   |   |-- lib.rs
|   |   |   |-- parser.rs
|   |   |   |-- plugins.rs
|   |   |   `-- syntax.rs
|   |   `-- Cargo.toml
|   `-- fua-plugin-angular/
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
|   |-- test_plain.html
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
**Total Files Scanned:** 32

| Extension | Count |
|---|---|
| .html | 10 |
| .rs | 9 |
| .toml | 4 |
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

A lightning-fast, highly-permissive, framework-agnostic HTML formatter written in Rust.

Fua Fua Format is designed to handle modern web development structures losslessly. Built on top of [Logos](https://github.com/maciejhirsz/logos) (for fast lexical analysis) and [Rowan](https://github.com/rust-analyzer/rowan) (for a lossless Red-Green syntax tree), it natively understands framework-specific bindings like Angular's `*ngIf` and `[(ngModel)]` or Vue's `@click` and `:disabled`, keeping your formatting completely structurally intact.

## Features
- **Framework-Agnostic**: Formats Angular, Vue, and vanilla HTML without choking on structural syntaxes.
- **Lossless Syntax Tree**: Guarantees zero data loss or layout corruption during formatting. 
- **Highly Configurable**: Control behavior with extensive config files or CLI arguments.
- **Microsecond Performance**: Built on top of ultra-fast Rust lexers utilized by `rust-analyzer`.

## Usage

You can use the formatter directly through the CLI:

```bash
# Format a file and output to stdout
cargo run --bin cli -- --input my_file.html

# Format a file explicitly overriding tab behavior and indent size
cargo run --bin cli -- --input my_file.html --output formatted.html --use-tabs true

# Format via a configuration file
cargo run --bin cli -- --input examples/sample.html --config examples/config.json --output examples/formatted.html
```

### CLI Arguments
* `-i, --input <file>`: Input file path. Reads from `stdin` if not provided.
* `-o, --output <file>`: Output file path. Writes to `stdout` by default.
* `-c, --config <json file>`: Path to your formatting configuration definitions.
* `--indent-size <number>`: Override the indent size explicitly.
* `--use-tabs <bool>`: Override the whitespace strategy explicitly.

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
  "indent_condition_groups": false,
  "plugin": {
    "options": {
      "wrap_conditions_in_parens": false
    }
  }
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
* `indent_condition_groups` *(Boolean, Default: false)*
  When Angular-style control-flow conditions already span multiple lines, indent nested parenthesized groups one extra level so layouts like `(` ... `)` blocks stay visually grouped.
* `plugin.options.wrap_conditions_in_parens` *(Boolean, Default: false)*
  For Angular binding values that are already multiline and get condition wrapping, emit an extra parenthesized group around the wrapped expression so object entries can format like `'key':` then `(` ... `)`.

## Architecture

Fua Fua Format consists of two primary workspace crates:
- `core`: Houses the Logos tokenizer (`lexer.rs`), the string tree parser (`parser.rs`), the configuration definitions (`config.rs`), and the top-down indent tree walker formatting engine (`formatter.rs`).
- `cli`: Houses the fast Clap CLI command interface bridging parameters linearly into the `core` parser.

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
fua-core = { package = "core", path = "../core" }
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

# FILE: crates/cli/src/main.rs
```rust
use clap::Parser as ClapParser;
use fua_core::config::FormatterConfig;
use fua_core::formatter::Formatter;
use fua_core::parser::Parser;
use fua_core::syntax::SyntaxNode;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path (or stdin if not provided)
    #[arg(short, long)]
    input: Option<String>,

    /// Output file path (or stdout if not provided)
    #[arg(short, long)]
    output: Option<String>,

    /// Path to a JSON configuration file
    #[arg(short, long)]
    config: Option<String>,

    /// Override indent size
    #[arg(long)]
    indent_size: Option<usize>,

    /// Use tabs instead of spaces
    #[arg(long)]
    use_tabs: Option<bool>,

    /// Path to a compiled .wasm formatter plugin (e.g. fua_plugin_angular.wasm)
    #[arg(long)]
    plugin: Option<String>,
}

/// For JS/JSX/TS/TSX files split the text at the first `<` tag boundary so
/// that the formatter only sees the HTML/JSX portion.  The leading script code
/// is returned as-is and re-prepended after formatting.
fn split_script_preamble(input: &str) -> (&str, &str) {
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            let next = bytes.get(i + 1).copied().unwrap_or(b' ');
            if next.is_ascii_alphabetic() || next == b'/' || next == b'!' {
                return (&input[..i], &input[i..]);
            }
        }
        i += 1;
    }
    (input, "")
}

/// Extract the **inner content** of `<script>` and `<style>` blocks as opaque
/// placeholders so the formatter never tries to reformat JS or CSS.
///
/// The opening/closing tags are kept in the output so the formatter can still
/// handle their attributes (e.g. `setup`, `scoped`, `lang="ts"`).
///
/// Returns the modified source plus a list of the extracted inner strings
/// (indexed by their placeholder number, e.g. `__RAW0__`).
fn extract_raw_blocks(input: &str) -> (String, Vec<String>) {
    const RAW_TAGS: &[&str] = &["script", "style"];

    let mut out = String::with_capacity(input.len());
    let mut blocks: Vec<String> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Look for an opening `<` followed by a raw-block tag name.
        if chars[i] == '<' {
            // Peek ahead (skip optional `/`) to get the tag name
            let start = i + 1;
            let tag_match = RAW_TAGS.iter().find(|&&tag| {
                let end = start + tag.len();
                if end > len { return false; }
                let candidate: String = chars[start..end].iter().collect();
                let candidate_lc = candidate.to_lowercase();
                if candidate_lc != tag { return false; }
                // Must be followed by whitespace, `>`, or end-of-input
                let after = chars.get(end).copied().unwrap_or('>');
                after == '>' || after == ' ' || after == '\t' || after == '\n' || after == '\r' || after == '/'
            });

            if let Some(&tag) = tag_match {
                // Consume the full opening tag (up to and including `>`).
                let open_start = i;
                while i < len && chars[i] != '>' { i += 1; }
                i += 1; // consume `>`
                let open_tag: String = chars[open_start..i].iter().collect();
                out.push_str(&open_tag);

                // Find the matching closing tag `</tagname>` (case-insensitive).
                let close_needle = format!("</{}", tag);
                let mut inner = String::new();
                loop {
                    if i >= len { break; }
                    // Check for closing tag start
                    let remaining: String = chars[i..].iter().collect();
                    let remaining_lc = remaining.to_lowercase();
                    if remaining_lc.starts_with(&close_needle) {
                        break;
                    }
                    inner.push(chars[i]);
                    i += 1;
                }

                let placeholder = format!("__RAW{}__", blocks.len());
                blocks.push(inner);
                out.push_str(&placeholder);
                // Don't advance i — the closing tag will be consumed normally.
                continue;
            }
        }

        out.push(chars[i]);
        i += 1;
    }

    (out, blocks)
}

/// Restore `__RAW0__` placeholders back to their original inner content.
fn restore_raw_blocks(formatted: &str, blocks: &[String]) -> String {
    let mut result = formatted.to_string();
    for (idx, inner) in blocks.iter().enumerate() {
        let placeholder = format!("__RAW{}__", idx);
        result = result.replace(&placeholder, inner);
    }
    result
}

/// Replace every `{...}` JSX expression that lives **outside** of a HTML tag's
/// attribute list with a unique placeholder token like `__FUA0__`.
/// Returns the modified source and the ordered list of extracted expressions.
///
/// Rules:
///  - Inside `<tag ...>` attribute regions we leave `{...}` untouched
///    (they are already passed through verbatim as attribute values).
///  - Outside tags (i.e. in element content) we extract balanced `{...}`
///    blocks, respecting nesting, single/double quotes, and template literals.
fn extract_jsx_expressions(input: &str) -> (String, Vec<String>) {
    let mut out = String::with_capacity(input.len());
    let mut exprs: Vec<String> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;
    // Track whether we are inside a `<tag …>` opening (i.e. attribute region).
    let mut in_tag = false;

    while i < len {
        let c = chars[i];

        if c == '<' {
            // Entering a tag – but `</` is a close tag (no attributes).
            if chars.get(i + 1).copied() != Some('/') {
                in_tag = true;
            }
            out.push(c);
            i += 1;
            continue;
        }

        if c == '>' && in_tag {
            in_tag = false;
            out.push(c);
            i += 1;
            continue;
        }

        // Inside a tag: copy verbatim (attribute `{…}` expressions are fine).
        if in_tag {
            out.push(c);
            i += 1;
            continue;
        }

        // Outside a tag: extract balanced `{…}` blocks.
        if c == '{' {
            let start = i;
            let mut depth = 0usize;
            let mut expr = String::new();

            while i < len {
                let ec = chars[i];
                // Handle string literals inside the expression opaquely.
                if ec == '"' || ec == '\'' || ec == '`' {
                    let q = ec;
                    expr.push(ec);
                    i += 1;
                    while i < len {
                        let sc = chars[i];
                        expr.push(sc);
                        i += 1;
                        if sc == '\\' {
                            // Escape – consume next char raw.
                            if i < len { expr.push(chars[i]); i += 1; }
                            continue;
                        }
                        if sc == q { break; }
                    }
                    continue;
                }
                if ec == '{' { depth += 1; }
                if ec == '}' {
                    depth -= 1;
                    expr.push(ec);
                    i += 1;
                    if depth == 0 { break; }
                    continue;
                }
                expr.push(ec);
                i += 1;
            }

            let placeholder = format!("__FUA{}__", exprs.len());
            exprs.push(expr);
            out.push_str(&placeholder);
            continue;
        }

        out.push(c);
        i += 1;
    }

    (out, exprs)
}

/// Extract `{...}` blocks in **element content** position (outside tags) that
/// contain NO HTML child elements (i.e. no `<letter` pattern inside).
///
/// This handles:
///  - Handlebars / Vue mustaches: `{{ value }}`
///  - Any other framework that uses `{expr}` syntax in template content
///
/// Angular's `@if (...) { ... }` blocks are **NOT** extracted because their
/// inner content always contains HTML child tags (`<element>`).
fn extract_template_expressions(input: &str) -> (String, Vec<String>) {
    let mut out = String::with_capacity(input.len());
    let mut exprs: Vec<String> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_tag = false;

    while i < len {
        let c = chars[i];

        if c == '<' {
            if chars.get(i + 1).copied() != Some('/') {
                in_tag = true;
            }
            out.push(c);
            i += 1;
            continue;
        }
        if c == '>' && in_tag {
            in_tag = false;
            out.push(c);
            i += 1;
            continue;
        }
        if in_tag {
            out.push(c);
            i += 1;
            continue;
        }

        // Outside a tag — check for a `{` that opens an expression block.
        if c == '{' {
            // Collect the balanced `{...}` block.
            let mut depth = 0usize;
            let mut expr = String::new();
            let mut j = i;

            while j < len {
                let ec = chars[j];
                // Skip string literals so their braces don't confuse depth.
                if ec == '"' || ec == '\'' || ec == '`' {
                    let q = ec;
                    expr.push(ec);
                    j += 1;
                    while j < len {
                        let sc = chars[j];
                        expr.push(sc);
                        j += 1;
                        if sc == '\\' {
                            if j < len { expr.push(chars[j]); j += 1; }
                            continue;
                        }
                        if sc == q { break; }
                    }
                    continue;
                }
                if ec == '{' { depth += 1; }
                if ec == '}' {
                    depth = depth.saturating_sub(1);
                    expr.push(ec);
                    j += 1;
                    if depth == 0 { break; }
                    continue;
                }
                expr.push(ec);
                j += 1;
            }

            // Only extract if the block contains NO opening HTML tags.
            // A block with `<letter` inside it is an Angular-style structural
            // block that the formatter needs to see and indent properly.
            let inner = &expr[1..expr.len().saturating_sub(1)]; // strip outer { }
            let has_html = inner.chars().enumerate().any(|(k, ch)| {
                if ch == '<' {
                    let next = inner.chars().nth(k + 1).unwrap_or(' ');
                    next.is_ascii_alphabetic()
                } else {
                    false
                }
            });

            if !has_html {
                let placeholder = format!("__TMPL{}__", exprs.len());
                exprs.push(expr);
                out.push_str(&placeholder);
                i = j;
                continue;
            }
            // Fall through: emit the `{` and let the formatter handle it.
        }

        out.push(chars[i]);
        i += 1;
    }

    (out, exprs)
}

/// Restore placeholders produced by `extract_template_expressions`.
fn restore_template_expressions(formatted: &str, exprs: &[String]) -> String {
    let mut result = formatted.to_string();
    for (idx, expr) in exprs.iter().enumerate() {
        let placeholder = format!("__TMPL{}__", idx);
        result = result.replace(&placeholder, expr);
    }
    result
}

/// Restore placeholders produced by `extract_jsx_expressions` back to their
/// original expression text.
fn restore_jsx_expressions(formatted: &str, exprs: &[String]) -> String {
    let mut result = formatted.to_string();
    for (idx, expr) in exprs.iter().enumerate() {
        let placeholder = format!("__FUA{}__", idx);
        result = result.replace(&placeholder, expr);
    }
    result
}

fn main() {
    let args = Args::parse();

    // Read input
    let mut input_text = String::new();
    if let Some(path) = &args.input {
        input_text = fs::read_to_string(path).expect("Failed to read input file");
    } else {
        io::stdin().read_to_string(&mut input_text).expect("Failed to read stdin");
    }

    if input_text.trim().is_empty() {
        eprintln!("Error: No HTML provided.");
        std::process::exit(1);
    }

    // Detect whether this is a JS/JSX/TS/TSX file that needs preamble splitting
    let is_script_file = args.input.as_deref()
        .and_then(|p| Path::new(p).extension())
        .map_or(false, |ext| {
            let e = ext.to_string_lossy().to_lowercase();
            e == "jsx" || e == "tsx" || e == "js" || e == "ts"
        });

    let (preamble, html_part) = if is_script_file {
        split_script_preamble(&input_text)
    } else {
        ("", input_text.as_str())
    };

    if html_part.trim().is_empty() {
        // Nothing to format – write file as-is
        if let Some(path) = &args.output {
            fs::write(path, &input_text).expect("Failed to write output file");
            println!("Successfully formatted into: {}", path);
        } else {
            print!("{}", input_text);
        }
        return;
    }

    // Load configuration
    let mut config = FormatterConfig::default();
    if let Some(config_path) = &args.config {
        let config_str = fs::read_to_string(config_path).expect("Failed to read config file");
        config = serde_json::from_str(&config_str).expect("Failed to parse config file");
    }

    // Apply CLI overrides over JSON config
    if let Some(size) = args.indent_size {
        config.indent_size = size;
    }
    if let Some(tabs) = args.use_tabs {
        config.use_tabs = tabs;
    }

    // Always extract <script> / <style> inner content as opaque pass-through.
    let (html_no_raw, raw_blocks) = extract_raw_blocks(html_part);

    // For HTML files: extract template `{...}` expressions (Vue mustaches, etc.)
    // that contain no HTML child elements — they're opaque pass-through.
    // For JSX files: use the broader `extract_jsx_expressions` instead.
    let (html_no_tmpl, tmpl_exprs) = if !is_script_file {
        extract_template_expressions(&html_no_raw)
    } else {
        (html_no_raw.clone(), Vec::new())
    };

    // For JSX/TSX files, also extract curly-brace expressions so the formatter
    // only sees the clean HTML skeleton (they are restored verbatim afterwards).
    let (html_to_parse, jsx_exprs) = if is_script_file {
        extract_jsx_expressions(&html_no_raw)
    } else {
        (html_no_tmpl.clone(), Vec::new())
    };

    // Parse and format only the HTML/JSX portion
    let parser = Parser::new(&html_to_parse);
    let green_node = parser.parse();
    let syntax_node = SyntaxNode::new_root(green_node);

    // Determine plugin path: CLI flag takes precedence, then config.plugin.path.
    // Paths from the config file are resolved relative to the config file's
    // directory so that `"./fua_plugin_angular.wasm"` just works.
    let plugin_path: Option<String> = args.plugin.clone().or_else(|| {
        let raw = config.plugin.path.as_deref()?;
        if let Some(cfg_path) = &args.config {
            let cfg_dir = Path::new(cfg_path).parent().unwrap_or(Path::new("."));
            Some(cfg_dir.join(raw).to_string_lossy().into_owned())
        } else {
            Some(raw.to_string())
        }
    });

    let mut formatter = Formatter::new(config);
    if let Some(ref path) = plugin_path {
        if let Err(e) = formatter.load_plugin(path) {
            eprintln!("Error: failed to load plugin '{}': {}", path, e);
            std::process::exit(1);
        }
    }
    let formatted_html = formatter.format(&syntax_node);

    // Restore in reverse order: JSX exprs → template exprs → raw blocks.
    let after_jsx = if is_script_file {
        restore_jsx_expressions(&formatted_html, &jsx_exprs)
    } else {
        formatted_html
    };
    let after_tmpl = if !is_script_file {
        restore_template_expressions(&after_jsx, &tmpl_exprs)
    } else {
        after_jsx
    };
    let restored = restore_raw_blocks(&after_tmpl, &raw_blocks);

    let output_text = format!("{}{}", preamble, restored);

    // Write output
    if let Some(path) = &args.output {
        fs::write(path, output_text).expect("Failed to write output file");
        println!("Successfully formatted into: {}", path);
    } else {
        print!("{}", output_text);
    }
}

```

# FILE: crates/core/Cargo.toml
```toml
[package]
name = "core"
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
    /// as a JSON string via `NodeData.plugin_options`.
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
    pub indent_condition_groups: bool,
    pub inline_short_elements_max_len: usize,
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
            indent_condition_groups: false,
            inline_short_elements_max_len: 80,
            plugin: PluginConfig::default(),
        }
    }
}

```

# FILE: crates/core/src/formatter.rs
```rust
use crate::config::FormatterConfig;
use crate::plugins::{NodeData, PluginManager, PluginResult};
use crate::syntax::{SyntaxKind, SyntaxNode};
use rowan::NodeOrToken;

fn is_block_element(_tag_name: &str) -> bool {
    true
}

fn get_tag_name(node: &SyntaxNode) -> Option<String> {
    for element in node.children_with_tokens() {
        if let NodeOrToken::Node(n) = element {
            if n.kind() == SyntaxKind::OPEN_TAG.into() {
                for tag_element in n.children_with_tokens() {
                    if let NodeOrToken::Token(t) = tag_element {
                        if t.kind() == SyntaxKind::IDENT.into() {
                            return Some(t.text().to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn kind_label(kind: SyntaxKind) -> &'static str {
    match kind {
        SyntaxKind::ELEMENT          => "ELEMENT",
        SyntaxKind::ROOT             => "ROOT",
        SyntaxKind::OPEN_TAG         => "OPEN_TAG",
        SyntaxKind::CLOSE_TAG        => "CLOSE_TAG",
        SyntaxKind::SELF_CLOSING_TAG => "SELF_CLOSING_TAG",
        _                            => "OTHER",
    }
}

pub struct Formatter {
    config: FormatterConfig,
    output: String,
    current_indent: usize,
    plugin_manager: PluginManager,
    /// Most-recently-seen IDENT inside an opening tag (for attribute-name hint).
    last_attr_name: String,
    /// Cached JSON of `config.plugin.options`.
    plugin_options_json: Option<String>,

    // ── Angular @if / @for condition state ────────────────────────────────────
    /// Depth of `(` chars seen after an Angular block-opener IDENT.
    /// 0 = not inside a condition; 1 = directly inside `@if (`; etc.
    angular_cond_depth: usize,
    /// The `current_indent` value at the moment the block-opener was emitted.
    /// Condition lines are indented relative to this value.
    angular_cond_base_indent: usize,
    /// True immediately after emitting an Angular block-opener (`@if`, `@for`…)
    /// so the next TEXT `(` token triggers condition-mode.
    just_saw_block_opener: bool,
}

impl Formatter {
    pub fn new(config: FormatterConfig) -> Self {
        let plugin_options_json = config.plugin.options.as_ref().map(|v| v.to_string());
        Self {
            config,
            output: String::new(),
            current_indent: 0,
            plugin_manager: PluginManager::new(),
            last_attr_name: String::new(),
            plugin_options_json,
            angular_cond_depth: 0,
            angular_cond_base_indent: 0,
            just_saw_block_opener: false,
        }
    }

    pub fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.plugin_manager.load_wasm(path)
    }

    pub fn get_indent(&self) -> String {
        if self.config.use_tabs {
            "\t".repeat(self.current_indent)
        } else {
            " ".repeat(self.current_indent * self.config.indent_size)
        }
    }

    fn indent_str(n: usize, use_tabs: bool, indent_size: usize) -> String {
        if use_tabs { "\t".repeat(n) } else { " ".repeat(n * indent_size) }
    }

    fn push_newlines_with_indent(&mut self, count: usize) {
        let mut trim_len = self.output.len();
        for c in self.output.chars().rev() {
            if c == ' ' || c == '\t' {
                trim_len -= c.len_utf8();
            } else {
                break;
            }
        }
        self.output.truncate(trim_len);

        let mut trailing_newlines = 0;
        for c in self.output.chars().rev() {
            if c == '\n' {
                trailing_newlines += 1;
            } else {
                break;
            }
        }

        while trailing_newlines < count {
            self.output.push('\n');
            trailing_newlines += 1;
        }
        self.output.push_str(&self.get_indent());
    }

    fn emit_plugin_result(&mut self, result: PluginResult) {
        if result.indent_delta < 0 {
            self.current_indent = self
                .current_indent
                .saturating_sub((-result.indent_delta) as usize);
        }
        if result.prepend_newline {
            self.push_newlines_with_indent(1);
        } else if result.prepend_space {
            let last = self.output.chars().next_back().unwrap_or('\0');
            if last != ' ' && last != '\t' && last != '\n' {
                self.output.push(' ');
            }
        }
        self.output.push_str(&result.output);
        if result.indent_delta > 0 {
            self.current_indent += result.indent_delta as usize;
        }
    }

    fn make_node_data<'a>(
        kind: &'a str,
        text: &'a str,
        parent_kind: &'a str,
        attr_name: &'a str,
        plugin_options: Option<&'a str>,
        current_indent: usize,
        indent_size: usize,
        use_tabs: bool,
    ) -> NodeData<'a> {
        NodeData {
            kind,
            text,
            parent_kind,
            attribute_name: attr_name,
            current_indent,
            indent_size,
            use_tabs,
            plugin_options,
        }
    }

    pub fn format(mut self, root: &SyntaxNode) -> String {
        self.format_node_internal(root, false);
        self.output
    }

    fn format_node_internal(&mut self, node: &SyntaxNode, force_inline: bool) {
        let prev_indent = self.current_indent;
        let is_block_elem = node.kind() == SyntaxKind::ELEMENT.into()
            && get_tag_name(node)
                .as_deref()
                .map_or(false, is_block_element);

        for element in node.children_with_tokens() {
            match element {
                NodeOrToken::Node(n) => {
                    if n.kind() == SyntaxKind::ELEMENT.into() {
                        let child_is_block =
                            get_tag_name(&n).as_deref().map_or(false, is_block_element);
                        let force_inline_child = is_element_simple_and_short(
                            &n,
                            self.config.inline_short_elements_max_len,
                        );

                        if child_is_block && !force_inline && !force_inline_child {
                            self.push_newlines_with_indent(1);
                        }

                        self.format_node_internal(&n, force_inline || force_inline_child);
                    } else if n.kind() == SyntaxKind::OPEN_TAG.into() {
                        self.format_node_internal(&n, force_inline);
                        if is_block_elem {
                            self.current_indent += 1;
                        }
                    } else if n.kind() == SyntaxKind::CLOSE_TAG.into() {
                        if is_block_elem {
                            self.current_indent = self.current_indent.saturating_sub(1);
                            if !force_inline {
                                self.push_newlines_with_indent(1);
                            }
                        }
                        self.format_node_internal(&n, force_inline);
                    } else {
                        self.format_node_internal(&n, force_inline);
                    }
                }

                NodeOrToken::Token(ref t) => {
                    let text = t.text();
                    let kind = t.kind();
                    let is_content = node.kind() == SyntaxKind::ELEMENT.into()
                        || node.kind() == SyntaxKind::ROOT.into();

                    // ── WHITESPACE ───────────────────────────────────────────
                    if kind == SyntaxKind::WHITESPACE.into() {
                        if node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into()
                        {
                            let next = element.next_sibling_or_token();
                            let next_is_bracket = next.as_ref().map_or(false, |e| {
                                e.kind() == SyntaxKind::CLOSE_ANGLE.into()
                                    || e.kind() == SyntaxKind::SLASH_CLOSE_ANGLE.into()
                            });

                            if next_is_bracket {
                                if self.config.bracket_same_line || force_inline {
                                    if next.as_ref().unwrap().kind()
                                        == SyntaxKind::SLASH_CLOSE_ANGLE.into()
                                    {
                                        self.output.push(' ');
                                    }
                                    continue;
                                } else if self.config.wrap_attributes {
                                    self.push_newlines_with_indent(1);
                                    continue;
                                }
                            }

                            if text.contains('\n') || self.config.wrap_attributes {
                                if force_inline {
                                    self.output.push(' ');
                                } else {
                                    self.push_newlines_with_indent(1);
                                    if self.config.use_tabs {
                                        self.output.push('\t');
                                    } else {
                                        self.output.push_str(
                                            &" ".repeat(self.config.indent_size),
                                        );
                                    }
                                }
                            } else {
                                self.output.push(' ');
                            }
                        } else if node.kind() == SyntaxKind::CLOSE_TAG.into() {
                            // drop whitespace inside `</div >`
                        } else {
                            // ── Content whitespace ───────────────────────────
                            let has_nl = text.contains('\n');

                            // Inside an Angular @if / @for condition paren:
                            // optionally indent based on the current paren depth.
                            if self.angular_cond_depth > 0 && has_nl {
                                let next = element.next_sibling_or_token();
                                let next_is_close_paren =
                                    next.as_ref().map_or(false, |e| match e {
                                        NodeOrToken::Token(t) => {
                                            t.kind() == SyntaxKind::TEXT.into()
                                                && t.text() == ")"
                                        }
                                        _ => false,
                                    });

                                let target = if self.config.indent_condition_groups {
                                    self.angular_cond_base_indent
                                        + self.angular_cond_depth
                                        - usize::from(next_is_close_paren)
                                } else if next_is_close_paren {
                                    self.angular_cond_base_indent
                                } else {
                                    self.angular_cond_base_indent + 1
                                };

                                let saved = self.current_indent;
                                self.current_indent = target;
                                self.push_newlines_with_indent(1);
                                self.current_indent = saved;
                                continue;
                            }

                            let next = element.next_sibling_or_token();
                            let next_text = next.as_ref().and_then(|e| {
                                if let NodeOrToken::Token(nt) = e {
                                    Some(nt.text())
                                } else {
                                    None
                                }
                            });

                            let newlines = text.matches('\n').count();

                            if self.plugin_manager.has_plugin()
                                && next_text.map(|t| t.starts_with('@')).unwrap_or(false)
                            {
                                if newlines > 1 {
                                    self.push_newlines_with_indent(2);
                                }
                                continue;
                            }

                            let next_is_block = next.as_ref().map_or(false, |e| {
                                if e.kind() == SyntaxKind::ELEMENT.into() {
                                    let n = e.as_node().unwrap();
                                    !is_element_simple_and_short(
                                        n,
                                        self.config.inline_short_elements_max_len,
                                    )
                                } else {
                                    false
                                }
                            });

                            let is_next_close_tag_of_block = next
                                .as_ref()
                                .map_or(false, |e| e.kind() == SyntaxKind::CLOSE_TAG.into());

                            if next_is_block || is_next_close_tag_of_block {
                                if newlines > 1 {
                                    self.push_newlines_with_indent(2);
                                }
                                continue;
                            }

                            let mut prev_non_ws = element.prev_sibling_or_token();
                            while let Some(p) = &prev_non_ws {
                                if p.kind() == SyntaxKind::WHITESPACE.into() {
                                    prev_non_ws = p.prev_sibling_or_token();
                                } else {
                                    break;
                                }
                            }
                            let is_after_open_tag = prev_non_ws
                                .as_ref()
                                .map_or(false, |e| e.kind() == SyntaxKind::OPEN_TAG.into());

                            if (has_nl
                                || (is_after_open_tag && self.config.wrap_content))
                                && !force_inline
                            {
                                if newlines > 1 {
                                    self.push_newlines_with_indent(2);
                                } else {
                                    self.push_newlines_with_indent(1);
                                }
                            } else if force_inline {
                                let is_before_close_tag = next.as_ref().map_or(false, |e| {
                                    e.kind() == SyntaxKind::CLOSE_TAG.into()
                                });
                                if !is_after_open_tag && !is_before_close_tag {
                                    self.output.push(' ');
                                }
                            } else {
                                self.output.push(' ');
                            }
                        }
                        continue;

                    // ── STRING DOUBLE ─────────────────────────────────────────
                    } else if kind == SyntaxKind::STRING_DOUBLE.into() {
                        let in_tag = node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into();

                        if in_tag && self.plugin_manager.has_plugin() {
                            let attr = self.last_attr_name.clone();
                            let opts = self.plugin_options_json.clone();
                            let node_data = Self::make_node_data(
                                "STRING_DOUBLE",
                                text,
                                kind_label(node.kind()),
                                &attr,
                                opts.as_deref(),
                                self.current_indent,
                                self.config.indent_size,
                                self.config.use_tabs,
                            );
                            if let Some(result) =
                                self.plugin_manager.call_format_hook(&node_data)
                            {
                                self.emit_plugin_result(result);
                                continue;
                            }
                        }

                        if self.config.single_quotes && text.len() >= 2 {
                            let inner = &text[1..text.len() - 1];
                            let escaped = inner.replace("'", "&apos;");
                            self.output.push('\'');
                            self.output.push_str(&escaped);
                            self.output.push('\'');
                        } else {
                            self.output.push_str(text);
                        }

                    // ── STRING SINGLE ─────────────────────────────────────────
                    } else if kind == SyntaxKind::STRING_SINGLE.into() {
                        let in_tag = node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into();

                        if in_tag && self.plugin_manager.has_plugin() {
                            let attr = self.last_attr_name.clone();
                            let opts = self.plugin_options_json.clone();
                            let node_data = Self::make_node_data(
                                "STRING_SINGLE",
                                text,
                                kind_label(node.kind()),
                                &attr,
                                opts.as_deref(),
                                self.current_indent,
                                self.config.indent_size,
                                self.config.use_tabs,
                            );
                            if let Some(result) =
                                self.plugin_manager.call_format_hook(&node_data)
                            {
                                self.emit_plugin_result(result);
                                continue;
                            }
                        }

                        if !self.config.single_quotes && text.len() >= 2 {
                            let inner = &text[1..text.len() - 1];
                            let escaped = inner.replace("\"", "&quot;");
                            self.output.push('"');
                            self.output.push_str(&escaped);
                            self.output.push('"');
                        } else {
                            self.output.push_str(text);
                        }

                    // ── CLOSE ANGLE / SELF-CLOSE ──────────────────────────────
                    } else if kind == SyntaxKind::CLOSE_ANGLE.into()
                        || kind == SyntaxKind::SLASH_CLOSE_ANGLE.into()
                    {
                        let prev = element.prev_sibling_or_token();
                        let prev_is_whitespace = prev
                            .as_ref()
                            .map_or(false, |e| e.kind() == SyntaxKind::WHITESPACE.into());

                        let in_opening = node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into();
                        if in_opening
                            && !self.config.bracket_same_line
                            && self.config.wrap_attributes
                        {
                            if !prev_is_whitespace && !force_inline {
                                self.push_newlines_with_indent(1);
                            }
                        }
                        self.output.push_str(text);

                    // ── COMMENT ───────────────────────────────────────────────
                    } else if kind == SyntaxKind::COMMENT.into() {
                        self.push_newlines_with_indent(1);
                        self.output.push_str(text);

                    // ── IDENT ─────────────────────────────────────────────────
                    } else if kind == SyntaxKind::IDENT.into() {
                        let in_tag = node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into();

                        if in_tag {
                            self.last_attr_name = text.to_string();
                        }

                        let is_elem_or_root = node.kind() == SyntaxKind::ELEMENT.into()
                            || node.kind() == SyntaxKind::ROOT.into();

                        let mut prev_non_ws = element.prev_sibling_or_token();
                        while let Some(p) = &prev_non_ws {
                            if p.kind() == SyntaxKind::WHITESPACE.into() {
                                prev_non_ws = p.prev_sibling_or_token();
                            } else {
                                break;
                            }
                        }
                        let is_after_open_tag = prev_non_ws
                            .as_ref()
                            .map_or(false, |e| e.kind() == SyntaxKind::OPEN_TAG.into());

                        if is_after_open_tag && self.config.wrap_content && !force_inline {
                            self.push_newlines_with_indent(1);
                        }

                        if is_elem_or_root {
                            let opts = self.plugin_options_json.clone();
                            let node_data = Self::make_node_data(
                                "IDENT",
                                text,
                                kind_label(node.kind()),
                                "",
                                opts.as_deref(),
                                self.current_indent,
                                self.config.indent_size,
                                self.config.use_tabs,
                            );
                            if let Some(result) =
                                self.plugin_manager.call_format_hook(&node_data)
                            {
                                // If the plugin pushed to a new line (block opener like @if),
                                // the next `(` starts the condition state machine.
                                if result.prepend_newline {
                                    self.just_saw_block_opener = true;
                                }
                                self.emit_plugin_result(result);
                                continue;
                            }
                        }

                        self.output.push_str(text);

                    // ── TEXT (content nodes) ──────────────────────────────────
                    } else if kind == SyntaxKind::TEXT.into() {
                        let is_elem_or_root = node.kind() == SyntaxKind::ELEMENT.into()
                            || node.kind() == SyntaxKind::ROOT.into();

                        if is_elem_or_root {
                            // ── Angular condition paren tracking ──────────────
                            if text == "(" {
                                if self.just_saw_block_opener {
                                    // `@if (` — enter condition mode
                                    self.just_saw_block_opener = false;
                                    self.angular_cond_depth = 1;
                                    self.angular_cond_base_indent = self.current_indent;
                                } else if self.angular_cond_depth > 0 {
                                    self.angular_cond_depth += 1;
                                }
                                self.output.push('(');
                                continue;
                            }

                            if text == ")" && self.angular_cond_depth > 0 {
                                self.angular_cond_depth =
                                    self.angular_cond_depth.saturating_sub(1);
                                self.output.push(')');
                                continue;
                            }

                            // ── Plugin hook for other TEXT tokens ─────────────
                            let opts = self.plugin_options_json.clone();
                            let node_data = Self::make_node_data(
                                "TEXT",
                                text,
                                kind_label(node.kind()),
                                "",
                                opts.as_deref(),
                                self.current_indent,
                                self.config.indent_size,
                                self.config.use_tabs,
                            );
                            if let Some(result) =
                                self.plugin_manager.call_format_hook(&node_data)
                            {
                                self.emit_plugin_result(result);
                                continue;
                            }
                        }

                        // ── Default TEXT handling (collapse whitespace) ────────
                        let mut prev_non_ws = element.prev_sibling_or_token();
                        while let Some(p) = &prev_non_ws {
                            if p.kind() == SyntaxKind::WHITESPACE.into() {
                                prev_non_ws = p.prev_sibling_or_token();
                            } else {
                                break;
                            }
                        }
                        let is_after_open_tag = prev_non_ws
                            .as_ref()
                            .map_or(false, |e| e.kind() == SyntaxKind::OPEN_TAG.into());

                        let mut next_non_ws = element.next_sibling_or_token();
                        while let Some(n) = &next_non_ws {
                            if n.kind() == SyntaxKind::WHITESPACE.into() {
                                next_non_ws = n.next_sibling_or_token();
                            } else {
                                break;
                            }
                        }
                        let is_before_close_tag = next_non_ws
                            .as_ref()
                            .map_or(false, |e| e.kind() == SyntaxKind::CLOSE_TAG.into());

                        let mut collapsed = if force_inline {
                            let mut s = text
                                .replace('\r', "")
                                .replace('\n', " ")
                                .replace('\t', " ");
                            while s.contains("  ") {
                                s = s.replace("  ", " ");
                            }
                            if is_after_open_tag {
                                s = s.trim_start().to_string();
                            }
                            if is_before_close_tag {
                                s = s.trim_end().to_string();
                            }
                            s
                        } else {
                            text.replace("  ", " ")
                        };

                        if is_after_open_tag
                            && self.config.wrap_content
                            && !collapsed.trim().is_empty()
                            && !force_inline
                        {
                            self.output.push('\n');
                            self.output.push_str(&self.get_indent());
                            collapsed = collapsed.trim_start().to_string();
                        }

                        self.output.push_str(&collapsed);

                    // ── FALLTHROUGH ───────────────────────────────────────────
                    } else {
                        // Reset block-opener flag on any unrecognised token
                        self.just_saw_block_opener = false;
                        self.output.push_str(text);
                    }
                }
            }
        }

        self.current_indent = prev_indent;
    }
}

fn is_element_simple_and_short(node: &SyntaxNode, max_len: usize) -> bool {
    let mut has_complex = false;
    for c in node.children_with_tokens() {
        if c.kind() == SyntaxKind::ELEMENT.into() || c.kind() == SyntaxKind::COMMENT.into() {
            has_complex = true;
            break;
        }
    }
    if has_complex {
        return false;
    }

    let mut total_len = 0;
    for t in node.descendants_with_tokens() {
        if let NodeOrToken::Token(tok) = t {
            if tok.kind() != SyntaxKind::WHITESPACE.into() {
                total_len += tok.text().len();
            }
        }
    }

    total_len <= max_len
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_smart_whitespace_formatting() {
        let input = "<div id=\"app\">   <p>Hello <span>world</span></p></div>";

        let parser = Parser::new(input);
        let green_node = parser.parse();
        let syntax_node = SyntaxNode::new_root(green_node);

        let formatter = Formatter::new(FormatterConfig::default());
        let output = formatter.format(&syntax_node);

        let expected = "\n<div id=\"app\">\n  <p>Hello <span>world</span>\n  </p>\n</div>";

        assert_eq!(output, expected);
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

    /// Permissive identifiers: tags and attributes.
    /// Captures standard names as well as Angular (*ngIf, [(ngModel)]) and Vue (@click, :disabled).
    #[regex(r"[a-zA-Z0-9_\-\*\[\]\@\:\$]+")]
    Ident,

    #[regex(r#""[^"]*""#)]
    StringDouble,

    #[regex(r"'[^']*'")]
    StringSingle,

    /// Permissive text nodes: capture any sequence of characters not handled by other tokens.
    /// This includes punctuation, non-ascii characters, etc.
    #[regex(r#"[^a-zA-Z0-9_\-\*\[\]\(\)\@\:\$<>= \t\n\r\f"']+"#)]
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
    fn test_framework_syntax() {
        let input = r#"<button @click="doIt" *ngIf="show" [(ngModel)]="val" :disabled="true" />"#;
        let tokens = lex(input);

        assert_eq!(
            tokens,
            vec![
                (Token::OpenAngle, "<"),
                (Token::Ident, "button"),
                (Token::Whitespace, " "),
                (Token::Ident, "@click"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"doIt\""),
                (Token::Whitespace, " "),
                (Token::Ident, "*ngIf"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"show\""),
                (Token::Whitespace, " "),
                (Token::Ident, "[(ngModel)]"),
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
pub mod lexer;
pub mod syntax;
pub mod parser;
pub mod config;
pub mod formatter;
pub mod plugins;

// Workaround for num-derive generating `core::option::Option` when the crate itself is named `core`
pub use std::option;
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

    fn parse_node(&mut self) {
        if let Some(&(ref res, _)) = self.lexer.peek() {
            let token = res.as_ref().unwrap_or(&Token::Text);
            match token {
                Token::OpenAngle => self.parse_element(),
                Token::OpenAngleSlash => { self.parse_tag(SyntaxKind::CLOSE_TAG); },
                _ => self.bump(),
            }
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
                "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta" | "source" | "track" | "wbr"
            );
        }

        // Parse open tag
        let is_self_closing = self.parse_tag(SyntaxKind::OPEN_TAG);

        if !is_self_closing && !is_void {
            // Parse children recursively
            loop {
                if let Some(&(ref res, _)) = self.lexer.peek() {
                    let token = res.as_ref().unwrap_or(&Token::Text);
                    if *token == Token::OpenAngleSlash {
                        break;
                    }
                    self.parse_node();
                } else {
                    break; // Graceful EOF fallback
                }
            }

            // Parse close tag if it exists
            if let Some(&(ref res, _)) = self.lexer.peek() {
                let token = res.as_ref().unwrap_or(&Token::Text);
                if *token == Token::OpenAngleSlash {
                    self.parse_tag(SyntaxKind::CLOSE_TAG);
                }
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
        while let Some(&(ref res, _)) = self.lexer.peek() {
            let token = res.as_ref().unwrap_or(&Token::Text);
            let is_close = *token == Token::CloseAngle || *token == Token::SlashCloseAngle;
            
            if *token == Token::SlashCloseAngle {
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
            <div id="app" *ngIf="show" @click="handle">
                hello world! 
                <button [(ngModel)]="value" :disabled="false" />
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
use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};

// ── Data contract shared between host and all guest plugins ──────────────────

/// Serialized and sent to the WASM plugin for every token the host wants
/// to offer for pre-emption.  The plugin returns [`PluginResult`] if it
/// claims the token, or an empty string / error to pass through.
#[derive(Debug, Serialize)]
pub struct NodeData<'a> {
    /// SyntaxKind as a human-readable string: `"IDENT"`, `"TEXT"`, `"STRING_DOUBLE"`, …
    pub kind: &'a str,
    /// Raw source text of the token (e.g. `"@if"`, `"{"`, `"\"value\""`).
    pub text: &'a str,
    /// SyntaxKind of the **parent** node: `"ELEMENT"`, `"ROOT"`, `"OPEN_TAG"`, …
    pub parent_kind: &'a str,
    /// For tokens inside a tag, the name of the attribute this token belongs to
    /// (e.g. `"[ngClass]"`).  Empty string when not inside an attribute.
    pub attribute_name: &'a str,
    /// Current indentation depth at the point this token is about to be emitted.
    pub current_indent: usize,
    /// Spaces per indent level (only meaningful when `use_tabs` is false).
    pub indent_size: usize,
    /// Whether the formatter is configured to use tab characters.
    pub use_tabs: bool,
    /// JSON string of plugin-specific options from `config.plugin.options`.
    /// `None` when no plugin section is present in the config.
    pub plugin_options: Option<&'a str>,
}

/// What the WASM plugin tells the formatter to do for a given token.
///
/// If the plugin returns an empty string the formatter falls back to its
/// built-in logic.  Otherwise it deserialises this struct and hands control
/// to [`Formatter::emit_plugin_result`].
#[derive(Debug, Deserialize)]
pub struct PluginResult {
    /// The literal text to push into the formatter output for this token.
    pub output: String,
    /// Signed indent adjustment applied **around** `output`:
    ///   - negative → decrement `current_indent` *before* emitting (e.g. closing `}`)
    ///   - positive → increment `current_indent` *after* emitting (e.g. opening `{`)
    pub indent_delta: i32,
    /// When `true` the formatter calls `push_newlines_with_indent(1)` before
    /// pushing `output`, ensuring a clean newline + correct indentation prefix.
    pub prepend_newline: bool,
    /// When `true` the formatter ensures at least one space immediately before
    /// `output` (idempotent — will not double-up an existing trailing space).
    pub prepend_space: bool,
}

// ── Plugin host ───────────────────────────────────────────────────────────────

pub struct PluginManager {
    plugin: Option<Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugin: None }
    }

    /// Returns `true` if a WASM plugin is currently loaded.
    pub fn has_plugin(&self) -> bool {
        self.plugin.is_some()
    }

    /// Load a compiled `.wasm` plugin from `path`.
    /// The plugin must export a function named `format_hook`.
    pub fn load_wasm(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let manifest = Manifest::new([Wasm::file(path)]);
        let plugin = Plugin::new(&manifest, [], true)?;
        self.plugin = Some(plugin);
        Ok(())
    }

    /// Offer `node_data` to the loaded plugin.
    ///
    /// Returns `Some(PluginResult)` when the plugin claims the token and
    /// provides formatting instructions.  Returns `None` (pass-through) when:
    ///   - no plugin is loaded
    ///   - the plugin function returns an empty string
    ///   - any serialisation / call error occurs
    pub fn call_format_hook(&mut self, node_data: &NodeData<'_>) -> Option<PluginResult> {
        let plugin = self.plugin.as_mut()?;
        let input = serde_json::to_string(node_data).ok()?;
        let response: String = plugin.call("format_hook", input).ok()?;
        if response.is_empty() {
            return None;
        }
        serde_json::from_str(&response).ok()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
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

# FILE: crates/fua-plugin-angular/Cargo.toml
```toml
[package]
name = "fua-plugin-angular"
version = "0.1.0"
edition = "2024"

# cdylib produces the .wasm binary when compiled with --target wasm32-wasip1
[lib]
crate-type = ["cdylib"]

[dependencies]
extism-pdk = "1"
serde = { workspace = true }
serde_json = { workspace = true }

```

# FILE: crates/fua-plugin-angular/src/lib.rs
```rust
/// fua-plugin-angular — Extism guest plugin
///
/// Handles Angular 17+ template specifics:
///   • Control-flow syntax: @if / @else if / @else / @for / @switch / …
///   • Condition wrapping: long `||` / `&&` chains in Angular bindings are
///     split onto individual lines.  The `||` / `&&` operator is aligned to
///     the same visual column as the start of the first condition on the
///     key line.  Extra spaces after the key `:` are added so the continuation
///     tab stops align cleanly.
///
/// Example output (indent_size=4, use_tabs=true):
///   'border-accent':   this.hasNameChanges·
///                   || this.hasDescriptionChanges·
///                   || this.hasImageChanges,
///
/// Build to WASM:
///   cargo build -p fua-plugin-angular --target wasm32-wasip1 --release
use extism_pdk::*;
use serde::{Deserialize, Serialize};

// ── Shared data contract (mirrors fua-core/src/plugins.rs) ───────────────────

#[derive(Deserialize)]
struct NodeData {
    kind: String,
    text: String,
    parent_kind: String,
    attribute_name: String,
    #[allow(dead_code)]
    current_indent: usize,
    indent_size: usize,
    use_tabs: bool,
    plugin_options: Option<String>,
}

#[derive(Serialize)]
struct PluginResult {
    output: String,
    indent_delta: i32,
    prepend_newline: bool,
    prepend_space: bool,
}

impl PluginResult {
    fn new(
        output: impl Into<String>,
        indent_delta: i32,
        prepend_newline: bool,
        prepend_space: bool,
    ) -> Self {
        Self {
            output: output.into(),
            indent_delta,
            prepend_newline,
            prepend_space,
        }
    }

    fn just(output: impl Into<String>) -> Self {
        Self::new(output, 0, false, false)
    }
}

// ── Angular control-flow helpers ──────────────────────────────────────────────

fn is_content_context(parent_kind: &str) -> bool {
    parent_kind == "ELEMENT" || parent_kind == "ROOT"
}

fn is_block_opener(text: &str) -> bool {
    matches!(
        text,
        "@if" | "@for" | "@switch" | "@case" | "@default" | "@empty"
    )
}

fn is_else_like(text: &str) -> bool {
    text == "@else" || text.starts_with("@else ")
}

// ── Angular binding helpers ───────────────────────────────────────────────────

fn is_angular_binding(attr_name: &str) -> bool {
    let n = attr_name.trim();
    (n.starts_with('[') && n.ends_with(']'))
        || (n.starts_with('(') && n.ends_with(')'))
        || n.starts_with("*ng")
        || n.starts_with("*cdk")
}

fn is_ngclass_attr(attr_name: &str) -> bool {
    let n = attr_name.trim();
    n.eq_ignore_ascii_case("[ngclass]") || n.eq_ignore_ascii_case("ngclass")
}

fn is_class_attr(attr_name: &str) -> bool {
    attr_name.trim().eq_ignore_ascii_case("class")
}

// ── Operator helpers ──────────────────────────────────────────────────────────

/// Count top-level `||` and `&&` operators in `s` (not inside brackets or strings).
fn count_top_level_ops(s: &str) -> usize {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut depth: i32 = 0;
    let mut count = 0;
    let mut i = 0;

    while i < len {
        match bytes[i] {
            b'(' | b'[' | b'{' => { depth += 1; i += 1; }
            b')' | b']' | b'}' => { depth = depth.saturating_sub(1); i += 1; }
            b'"' | b'\'' | b'`' => {
                let q = bytes[i];
                i += 1;
                while i < len {
                    if bytes[i] == b'\\' { i += 2; continue; }
                    if bytes[i] == q { i += 1; break; }
                    i += 1;
                }
            }
            b'|' if depth == 0 && i + 1 < len && bytes[i + 1] == b'|' => { count += 1; i += 2; }
            b'&' if depth == 0 && i + 1 < len && bytes[i + 1] == b'&' => { count += 1; i += 2; }
            _ => { i += 1; }
        }
    }
    count
}

/// Split `s` on top-level `||` / `&&`, returning `(operator, trimmed_expression)` pairs.
/// The first entry always has an empty operator string.
fn split_on_top_level_ops(s: &str) -> Vec<(String, String)> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut depth: i32 = 0;
    let mut parts: Vec<(String, String)> = Vec::new();
    let mut current = String::new();
    let mut current_op = String::new();
    let mut i = 0;

    while i < len {
        match bytes[i] {
            b'(' | b'[' | b'{' => { depth += 1; current.push(bytes[i] as char); i += 1; }
            b')' | b']' | b'}' => {
                depth = depth.saturating_sub(1);
                current.push(bytes[i] as char);
                i += 1;
            }
            b'"' | b'\'' | b'`' => {
                let q = bytes[i];
                current.push(q as char);
                i += 1;
                while i < len {
                    if bytes[i] == b'\\' {
                        current.push('\\');
                        i += 1;
                        if i < len { current.push(bytes[i] as char); i += 1; }
                    } else if bytes[i] == q {
                        current.push(q as char);
                        i += 1;
                        break;
                    } else {
                        current.push(bytes[i] as char);
                        i += 1;
                    }
                }
            }
            b'|' if depth == 0 && i + 1 < len && bytes[i + 1] == b'|' => {
                parts.push((current_op.clone(), current.trim().to_string()));
                current_op = "||".to_string();
                current = String::new();
                i += 2;
                while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
            }
            b'&' if depth == 0 && i + 1 < len && bytes[i + 1] == b'&' => {
                parts.push((current_op.clone(), current.trim().to_string()));
                current_op = "&&".to_string();
                current = String::new();
                i += 2;
                while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') { i += 1; }
            }
            _ => { current.push(bytes[i] as char); i += 1; }
        }
    }

    let last = current.trim().to_string();
    if !last.is_empty() || !current_op.is_empty() {
        parts.push((current_op, last));
    }
    parts
}

fn split_trailing_punctuation(s: &str) -> (String, String) {
    let trimmed = s.trim_end();
    if let Some(last) = trimmed.chars().last() {
        if matches!(last, ',' | ';') {
            let body = trimmed[..trimmed.len() - last.len_utf8()].trim_end().to_string();
            return (body, last.to_string());
        }
    }
    (trimmed.to_string(), String::new())
}

fn unwrap_negated_group(s: &str) -> Option<&str> {
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

fn is_wrappable_condition_expr(s: &str, min_ops: usize) -> bool {
    let t = s.trim();
    if count_top_level_ops(t) >= min_ops {
        return true;
    }
    if unwrap_negated_group(t).is_some() {
        return true;
    }
    false
}

fn split_top_level_commas(s: &str) -> Vec<String> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut in_quote: Option<u8> = None;
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut i = 0usize;
    while i < bytes.len() {
        match in_quote {
            Some(q) => {
                cur.push(bytes[i] as char);
                if bytes[i] == b'\\' {
                    i += 1;
                    if i < bytes.len() {
                        cur.push(bytes[i] as char);
                    }
                } else if bytes[i] == q {
                    in_quote = None;
                }
            }
            None => match bytes[i] {
                b'"' | b'\'' | b'`' => {
                    in_quote = Some(bytes[i]);
                    cur.push(bytes[i] as char);
                }
                b'(' | b'[' | b'{' => {
                    depth += 1;
                    cur.push(bytes[i] as char);
                }
                b')' | b']' | b'}' => {
                    depth = depth.saturating_sub(1);
                    cur.push(bytes[i] as char);
                }
                b',' if depth == 0 => {
                    let part = cur.trim();
                    if !part.is_empty() {
                        out.push(part.to_string());
                    }
                    cur.clear();
                }
                _ => cur.push(bytes[i] as char),
            },
        }
        i += 1;
    }
    let last = cur.trim();
    if !last.is_empty() {
        out.push(last.to_string());
    }
    out
}

fn leading_ws_of_line(s: &str) -> &str {
    let ws_len = s
        .bytes()
        .take_while(|&b| b == b'\t' || b == b' ')
        .count();
    &s[..ws_len]
}

fn split_top_level_ternary(s: &str) -> Option<(String, String, String)> {
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;
    let mut in_quote: Option<u8> = None;
    let mut q_idx: Option<usize> = None;
    let mut colon_idx: Option<usize> = None;

    let mut i = 0usize;
    while i < bytes.len() {
        match in_quote {
            Some(q) => {
                if bytes[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if bytes[i] == q {
                    in_quote = None;
                }
                i += 1;
                continue;
            }
            None => match bytes[i] {
                b'"' | b'\'' | b'`' => {
                    in_quote = Some(bytes[i]);
                    i += 1;
                    continue;
                }
                b'(' | b'[' | b'{' => {
                    depth += 1;
                    i += 1;
                    continue;
                }
                b')' | b']' | b'}' => {
                    depth = depth.saturating_sub(1);
                    i += 1;
                    continue;
                }
                b'?' if depth == 0 && q_idx.is_none() => {
                    q_idx = Some(i);
                    i += 1;
                    continue;
                }
                b':' if depth == 0 && q_idx.is_some() => {
                    // Keep updating; we want the last top-level ':' for robustness.
                    colon_idx = Some(i);
                    i += 1;
                    continue;
                }
                _ => {
                    i += 1;
                    continue;
                }
            },
        }
    }

    let q = q_idx?;
    let c = colon_idx?;
    if q >= c {
        return None;
    }

    let cond = s[..q].trim().to_string();
    let then_expr = s[q + 1..c].trim().to_string();
    let else_expr = s[c + 1..].trim().to_string();
    if cond.is_empty() || then_expr.is_empty() || else_expr.is_empty() {
        return None;
    }
    Some((cond, then_expr, else_expr))
}

fn wrap_ternary(
    expr: &str,
    min_ops: usize,
    indent_size: usize,
    use_tabs: bool,
    current_indent: usize,
    _wrap_in_parens: bool,
) -> Option<String> {
    let (cond, then_expr, else_expr) = split_top_level_ternary(expr)?;

    // Indent continuation inside attribute values:
    // current_indent (tag indent) + 2 levels matches the existing style.
    let cont = if use_tabs {
        "\t".repeat(current_indent + 2)
    } else {
        " ".repeat((current_indent + 2) * indent_size)
    };

    let should_wrap_cond = count_top_level_ops(&cond) >= min_ops
        || (min_ops > 0 && (cond.contains("&&") || cond.contains("||")))
    ;

    let cond_wrapped = if should_wrap_cond {
        let parts = split_on_top_level_ops(&cond);
        if parts.len() >= 2 {
            // Prefer the same grouped style as ngClass wrapping:
            // (
            //     <first>
            //     && <next>
            // )
            if use_tabs {
                // For attribute values, keep `(` flush after the quote,
                // then indent inner condition lines under `?` / `:`.
                let group_indent = cont.clone();
                let inner_indent = format!("{group_indent}\t");
                let mut lines: Vec<String> = Vec::with_capacity(parts.len() + 2);
                // Start on a new line so it visually matches ngClass-style blocks.
                lines.push(format!("{group_indent}("));
                lines.push(format!("{}   {}", inner_indent, parts[0].1));
                for (op, c) in parts.into_iter().skip(1) {
                    lines.push(format!("{}{} {}", inner_indent, op, c));
                }
                lines.push(format!("{group_indent})"));
                format!("\n{}", lines.join("\n"))
            } else {
                let mut lines: Vec<String> = Vec::with_capacity(parts.len());
                lines.push(parts[0].1.clone());
                for (op, c) in parts.into_iter().skip(1) {
                    lines.push(format!("{cont}{op} {c}"));
                }
                lines.join("\n")
            }
        } else {
            cond
        }
    } else {
        cond
    };

    Some(format!("{cond_wrapped}\n{cont}? {then_expr}\n{cont}: {else_expr}"))
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

    // Determine indentation used by entries.
    let mut entry_ws = "";
    for line in inner.split('\n').skip(1) {
        let l = line.trim_end_matches('\r');
        if l.trim().is_empty() {
            continue;
        }
        entry_ws = leading_ws_of_line(l);
        break;
    }
    if entry_ws.is_empty() && use_tabs {
        entry_ws = "\t\t\t"; // reasonable fallback for wrapped attribute values
    }

    let body = trimmed[1..trimmed.len() - 1].trim();
    let entries = split_top_level_commas(body);
    if entries.is_empty() {
        return None;
    }

    // Indent closing brace to the same level as entries (matches typical
    // Angular multiline object-literal style inside quoted bindings).
    let close_ws = entry_ws;
    let mut out_lines: Vec<String> = Vec::new();
    out_lines.push("{".to_string());

    for (idx, raw_entry) in entries.iter().enumerate() {
        let colon = match find_kv_colon(raw_entry) {
            Some(c) => c,
            None => {
                out_lines.push(format!("{}{}", entry_ws, raw_entry));
                continue;
            }
        };
        let key_part = raw_entry[..=colon].trim_end(); // includes ':'
        let value = raw_entry[colon + 1..].trim_start();

        let is_last = idx == entries.len() - 1;
        let suffix = if is_last { "" } else { "," };

        if wrap_in_parens && use_tabs && is_wrappable_condition_expr(value, min_ops) {
            // Build a synthetic single line with the inferred indentation so
            // wrap_line_conditions can render the ideal `key:` + `(`...`)` shape.
            let leading_tabs = entry_ws.bytes().take_while(|&b| b == b'\t').count();
            let ws = "\t".repeat(leading_tabs);
            let synthetic = format!("{}{} {}", ws, key_part, value);
            if let Some(wrapped) =
                wrap_line_conditions(&synthetic, min_ops, indent_size, use_tabs, 0, true)
            {
                let mut lines: Vec<String> =
                    wrapped.split('\n').map(|l| l.to_string()).collect();
                if suffix == "," {
                    if let Some(last) = lines.last_mut() {
                        last.push(',');
                    }
                }
                out_lines.extend(lines);
                continue;
            }
        }

        out_lines.push(format!("{}{} {}", entry_ws, key_part, value.trim_end_matches(',')) + suffix);
    }

    out_lines.push(format!("{}{}", close_ws, "}"));
    Some(out_lines.join("\n"))
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
    // Return content without surrounding quotes; caller wraps with `"`/`'`.
    // Keep closing quote on its own aligned line by ending with `\n{cont}`.
    let mut out = String::new();
    out.push('\n');
    for t in tokens {
        out.push_str(&token_indent);
        out.push_str(t);
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

    let mut out_lines = Vec::with_capacity(entries.len() + 2);
    out_lines.push("{".to_string());
    for (idx, e) in entries.iter().enumerate() {
        let suffix = if idx + 1 == entries.len() { "" } else { "," };
        out_lines.push(format!("{entry_indent}{}{}", e.trim(), suffix));
    }
    out_lines.push(format!("{entry_indent}}}"));
    Some(out_lines.join("\n"))
}

// ── Key-value separator detection ────────────────────────────────────────────

/// Find the byte offset of the first `:` in `s` that is:
/// - at bracket/paren depth 0 and not inside a string literal
/// - followed by a space or tab (not `://`, `::`, etc.)
///
/// Returns the byte index of `:` itself.
fn find_kv_colon(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut depth: i32 = 0;
    let mut in_quote: Option<u8> = None;
    let mut i = 0;

    while i < len {
        match in_quote {
            Some(q) => {
                if bytes[i] == b'\\' { i += 2; continue; }
                if bytes[i] == q { in_quote = None; }
                i += 1;
            }
            None => match bytes[i] {
                b'"' | b'\'' | b'`' => { in_quote = Some(bytes[i]); i += 1; }
                b'(' | b'[' | b'{' => { depth += 1; i += 1; }
                b')' | b']' | b'}' => { depth = depth.saturating_sub(1); i += 1; }
                b':' if depth == 0 => {
                    let after = bytes.get(i + 1).copied().unwrap_or(b'\0');
                    if after == b' ' || after == b'\t' {
                        return Some(i);
                    }
                    i += 1;
                }
                _ => { i += 1; }
            }
        }
    }
    None
}

// ── Line-level condition wrapping ─────────────────────────────────────────────

/// Wrap one content-line's conditions onto multiple lines with precise
/// tab-column alignment.
///
/// For a line like:
///   `\t\t\t\t\t\t'border-accent': condA || condB || condC,`
///
/// Output:
///   `\t\t\t\t\t\t'border-accent':   condA `
///   `\t\t\t\t\t\t\t\t\t\t|| condB `
///   `\t\t\t\t\t\t\t\t\t\t|| condC,`
///
/// The `||` column is chosen so that `|| condX` is visually aligned with `condA`.
/// Enough spaces are added after `:` to push `condA` to the next tab stop that
/// allows a clean `n_tabs × tab_width` continuation.
///
/// Returns `None` when no wrapping is needed.
/// `fallback_cont_indent` — number of indent units to use for continuation when
/// there is no leading whitespace AND no key-value separator (single-line binding).
/// Pass `0` for multiline values (leading whitespace from the line is used instead).
fn wrap_line_conditions(
    raw_line: &str,
    min_ops: usize,
    indent_size: usize,
    use_tabs: bool,
    fallback_cont_indent: usize,
    wrap_in_parens: bool,
) -> Option<String> {
    let line = raw_line.trim_end_matches('\r'); // handle CRLF inside strings

    let n_leading_tabs = line.bytes().take_while(|&b| b == b'\t').count();
    let leading_ws = &line[..n_leading_tabs];
    let content = &line[n_leading_tabs..];

    if content.is_empty() { return None; }

    // Separate the key part (`'key':`) from the conditions expression.
    let (key_part, conditions_str) = match find_kv_colon(content) {
        Some(colon_idx) => {
            // key_part includes the ':' character
            let kp = &content[..=colon_idx];
            let after = content[colon_idx + 1..].trim_start();
            (kp, after)
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
    if parts.len() < 2 { return None; }

    let first_cond = &parts[0].1;

    // ── Tab-aligned continuation ──────────────────────────────────────────────
    if use_tabs && indent_size > 0 {
        if wrap_in_parens && !key_part.is_empty() {
            let group_indent = format!("{}\t", leading_ws);
            let inner_indent = format!("{}\t", group_indent);
            let mut out_lines: Vec<String> = Vec::with_capacity(parts.len() + 2);
            let mut last_parts = parts.clone();
            let last_index = last_parts.len() - 1;
            let (last_body, trailing_suffix) =
                split_trailing_punctuation(&last_parts[last_index].1);
            last_parts[last_index].1 = last_body;
            out_lines.push(format!("{}{}", leading_ws, key_part));
            out_lines.push(format!("{}(", group_indent));
            if negated_group.is_some() {
                let negation_indent = format!("{}\t", group_indent);
                let inner_indent = format!("{}\t", negation_indent);
                out_lines.push(format!("{}!(", negation_indent));
                out_lines.push(format!("{}   {}", inner_indent, first_cond));
                for (op, cond) in last_parts[1..].iter() {
                    out_lines.push(format!("{}{} {}", inner_indent, op, cond));
                }
                out_lines.push(format!("{})", negation_indent));
            } else {
                out_lines.push(format!("{}   {}", inner_indent, first_cond));
                for (op, cond) in last_parts[1..].iter() {
                    out_lines.push(format!("{}{} {}", inner_indent, op, cond));
                }
            }
            out_lines.push(format!("{}){}", group_indent, trailing_suffix));
            return Some(out_lines.join("\n"));
        }

        let tab_width = indent_size;
        let base_col = n_leading_tabs * tab_width;
        let after_key_col = base_col + key_part.len();
        let min_op_col = (after_key_col + 1).saturating_sub(3);
        let next_tab_col = if min_op_col % tab_width == 0 {
            min_op_col
        } else {
            (min_op_col / tab_width + 1) * tab_width
        };
        let first_cond_col = next_tab_col + 3;
        let padding = first_cond_col - after_key_col;
        let n_cont_tabs = next_tab_col / tab_width;
        let cont_prefix = "\t".repeat(n_cont_tabs);

        let mut out_lines: Vec<String> = Vec::with_capacity(parts.len());
        out_lines.push(format!(
            "{}{}{}{} ",
            leading_ws,
            key_part,
            " ".repeat(padding),
            first_cond,
        ));
        for (idx, (op, cond)) in parts[1..].iter().enumerate() {
            let is_last = idx == parts.len() - 2;
            if is_last {
                out_lines.push(format!("{}{} {}", cont_prefix, op, cond));
            } else {
                out_lines.push(format!("{}{} {} ", cont_prefix, op, cond));
            }
        }
        return Some(out_lines.join("\n"));
    }

    // ── Fallback: single-line with no key-value separator ────────────────────
    // (n_leading_tabs == 0 and key_part == "")
    if use_tabs && n_leading_tabs == 0 && key_part.is_empty() && fallback_cont_indent > 0 {
        let cont_prefix = "\t".repeat(fallback_cont_indent);

        let mut out_lines: Vec<String> = Vec::with_capacity(parts.len());
        out_lines.push(first_cond.clone());

        for (idx, (op, cond)) in parts[1..].iter().enumerate() {
            let is_last = idx == parts.len() - 2;
            if is_last {
                out_lines.push(format!("{}{} {}", cont_prefix, op, cond));
            } else {
                out_lines.push(format!("{}{} {} ", cont_prefix, op, cond));
            }
        }

        return Some(out_lines.join("\n"));
    }

    // ── Space-based continuation (use_tabs = false) ───────────────────────────
    {
        let n_leading_spaces = line.bytes().take_while(|&b| b == b' ').count();
        let base_col = n_leading_spaces;
        let after_key_col = base_col + key_part.len();

        let first_cond_col = after_key_col + 1;
        let op_col = first_cond_col.saturating_sub(3);

        let padding = first_cond_col - after_key_col;
        let cont_prefix = " ".repeat(op_col);

        let mut out_lines: Vec<String> = Vec::with_capacity(parts.len());
        out_lines.push(format!(
            "{}{}{}{} ",
            " ".repeat(n_leading_spaces),
            key_part,
            " ".repeat(padding),
            first_cond,
        ));

        for (idx, (op, cond)) in parts[1..].iter().enumerate() {
            let is_last = idx == parts.len() - 2;
            if is_last {
                out_lines.push(format!("{}{} {}", cont_prefix, op, cond));
            } else {
                out_lines.push(format!("{}{} {} ", cont_prefix, op, cond));
            }
        }

        Some(out_lines.join("\n"))
    }
}

// ── Attribute string reformatting ─────────────────────────────────────────────

/// Process a full attribute string value (including the enclosing quote chars).
///
/// For multiline values each content line is processed individually.
/// For single-line values with no key-value structure (e.g. `[buffer]="expr || expr"`)
/// the continuation indent falls back to `current_indent + 2` tabs (attribute
/// level + one continuation level).
///
/// Returns `Some(new_value)` when at least one line was wrapped.
fn process_attribute_string(
    text: &str,
    attr_name: &str,
    min_ops: usize,
    indent_size: usize,
    use_tabs: bool,
    current_indent: usize,
    wrap_in_parens: bool,
    wrap_ternary_expr: bool,
    ngclass_wrap_entries_min: usize,
    class_wrap_tokens_min: usize,
) -> Option<String> {
    if text.len() < 2 { return None; }

    let (quote, inner) = if text.starts_with('"') && text.ends_with('"') {
        ('"', &text[1..text.len() - 1])
    } else if text.starts_with('\'') && text.ends_with('\'') {
        ('\'', &text[1..text.len() - 1])
    } else {
        return None;
    };

    // Safety normalization: plain Angular bindings like `[title]="a || b"`
    // should remain single-line. If such values somehow carry embedded
    // newlines, collapse them back to a compact one-liner.
    if inner.contains('\n')
        && attr_name.trim().starts_with('[')
        && attr_name.trim().ends_with(']')
        && !is_ngclass_attr(attr_name)
        && !inner.contains('{')
        && !inner.contains('?')
    {
        let compact = inner.split_whitespace().collect::<Vec<_>>().join(" ");
        return Some(format!("{}{}{}", quote, compact, quote));
    }

    if is_class_attr(attr_name) {
        if let Some(wrapped_classes) =
            format_class_tokens(inner, class_wrap_tokens_min, current_indent, use_tabs, indent_size)
        {
            return Some(format!("{}{}{}", quote, wrapped_classes, quote));
        }
    }

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
        if let Some(obj) = force_wrap_ngclass_object(inner, current_indent, use_tabs, indent_size)
        {
            return Some(format!("{}{}{}", quote, obj, quote));
        }
    }

    if let Some(obj) = format_object_literal(
        inner,
        min_ops,
        indent_size,
        use_tabs,
        wrap_in_parens,
        force_wrap_ngclass,
    )
    {
        return Some(format!("{}{}{}", quote, obj, quote));
    }

    // Optional: wrap top-level ternary expressions onto multiple lines.
    // (Useful for `[ngStyle]="cond ? {...} : {}"` patterns.)
    if wrap_ternary_expr && !inner.contains('\n') {
        if let Some(wrapped) = wrap_ternary(
            inner,
            min_ops,
            indent_size,
            use_tabs,
            current_indent,
            wrap_in_parens,
        ) {
            return Some(format!("{}{}{}", quote, wrapped, quote));
        }
    }

    // For multiline values, process line by line.
    if inner.contains('\n') {
        let mut result_lines: Vec<String> = Vec::new();
        let mut changed = false;

        for line in inner.split('\n') {
            if changed && line.trim().is_empty() {
                continue;
            }
            match wrap_line_conditions(
                line,
                min_ops,
                indent_size,
                use_tabs,
                0,
                wrap_in_parens,
            ) {
                Some(wrapped) => { result_lines.push(wrapped); changed = true; }
                None => { result_lines.push(line.trim_end_matches('\r').to_string()); }
            }
        }

        if !changed { return None; }
        return Some(format!("{}{}{}", quote, result_lines.join("\n"), quote));
    }

    // Keep plain single-line bindings unchanged (e.g. `[title]="a || b"`).
    // Special cases are handled above:
    //   - ngClass object literals
    //   - ternary expressions when enabled
    None
}

// ── Plugin entry point ────────────────────────────────────────────────────────

#[plugin_fn]
pub fn format_hook(input: String) -> FnResult<String> {
    let node: NodeData = match serde_json::from_str(&input) {
        Ok(n) => n,
        Err(_) => return Ok(String::new()),
    };

    let opts: serde_json::Value = node
        .plugin_options
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(serde_json::json!({}));

    let result: Option<PluginResult> = match node.kind.as_str() {

        // ── STRING values inside Angular binding attributes ───────────────────
        "STRING_DOUBLE" | "STRING_SINGLE"
            if (node.parent_kind == "OPEN_TAG"
                || node.parent_kind == "SELF_CLOSING_TAG")
                && (is_angular_binding(&node.attribute_name)
                    || is_class_attr(&node.attribute_name)) =>
        {
            // wrap_conditions_min counts *conditions* (operands), so
            // wrap when operators >= min_conds - 1.
            let min_conds = opts
                .get("wrap_conditions_min")
                .and_then(|v| v.as_u64())
                .unwrap_or(2) as usize;
            let min_ops = min_conds.saturating_sub(1);
            let wrap_in_parens = opts
                .get("wrap_conditions_in_parens")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            process_attribute_string(
                &node.text,
                &node.attribute_name,
                min_ops,
                node.indent_size,
                node.use_tabs,
                node.current_indent,
                wrap_in_parens,
                opts.get("wrap_ternary")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                opts.get("ngclass_wrap_entries_min")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .unwrap_or(usize::MAX),
                opts.get("class_wrap_tokens_min")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .unwrap_or(usize::MAX),
            )
            .map(PluginResult::just)
        }

        // ── Angular control-flow IDENT tokens ────────────────────────────────
        "IDENT" if is_content_context(&node.parent_kind) => {
            if is_block_opener(&node.text) {
                Some(PluginResult::new(&*node.text, 0, true, false))
            } else if is_else_like(&node.text) {
                Some(PluginResult::new(&*node.text, 0, false, true))
            } else {
                None
            }
        }

        // ── Block delimiters ─────────────────────────────────────────────────
        "TEXT" if is_content_context(&node.parent_kind) => {
            match node.text.as_str() {
                "{" => Some(PluginResult::new("{", 1, false, true)),
                "}" => Some(PluginResult::new("}", -1, true, false)),
                _ => None,
            }
        }

        _ => None,
    };

    match result {
        Some(r) => Ok(serde_json::to_string(&r).unwrap_or_default()),
        None => Ok(String::new()),
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
  "indent_condition_groups": true,
  "inline_short_elements_max_len": 80,
  "plugin": {
    "path": "../target/wasm32-wasip1/release/fua_plugin_angular.wasm",
    "options": {
      "wrap_conditions_min": 2,
      "wrap_conditions_in_parens": true,
      "wrap_ternary": true,
      "ngclass_wrap_entries_min": 2,
      "class_wrap_tokens_min": 6
    }
  }
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

# FILE: examples/test_plain.html
```html
<!DOCTYPE html>
<html   lang="en">
<head>
    <meta     charset="UTF-8">
  <meta name="viewport"    content="width=device-width, initial-scale=1.0">
        <title>Plain HTML Test</title>
  <link rel="stylesheet"     href="styles.css">
</head>
<body>
  <header       class="site-header">
<nav class="nav">
              <a    href="/"   class="nav-logo">Home</a>
    <ul class="nav-links"  >
      <li><a href="/about">About</a></li>
              <li><a   href="/blog"  >Blog</a></li>
      <li><a href="/contact"  >Contact</a></li>
    </ul>
  </nav>
</header>

    <main   class="container">
<section    class="hero">
        <h1 class="hero-title"   >Welcome to the site</h1>
  <p  class="hero-subtitle"  >A simple, fast, clean website.</p>
        <a     href="/get-started"   class="btn btn-primary">Get Started</a>
</section>

<section class="features"   >
  <article  class="card"  >
              <img src="icon1.svg"    alt="Feature one"   width="48"   height="48">
    <h2>Fast</h2>
            <p>   Loads in under a second on any connection.   </p>
  </article>
      <article class="card">
    <img   src="icon2.svg" alt="Feature two" width="48" height="48">
              <h2>  Accessible  </h2>
    <p>Built with semantic HTML and ARIA labels.</p>
  </article>
  <article class="card">
    <img src="icon3.svg" alt="Feature three"   width="48" height="48">
    <h2>Open Source</h2>
    <p>Every line of code is public on GitHub.</p>
  </article>
</section>
    </main>

      <footer  class="site-footer">
  <p>&copy; 2024 My Site.   All rights reserved.</p>
      </footer>
</body>
</html>

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
