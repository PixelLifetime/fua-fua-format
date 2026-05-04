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

You can use the formatter in two main ways:

- **As a local npm tool in your own project** (recommended for real app repos)
- **Directly from Rust source** (useful while developing this formatter itself)

### Use in your own project (npm)

Install formatter + plugins:

```bash
npm i -D fua-fua @fua-fua/plugin-tailwind @fua-fua/plugin-angular
```

Create `.fua/config.json` in your project root:

```json
{
  "indent_size": 2,
  "use_tabs": false,
  "print_width": 100,
  "wrap_attributes": true,
  "class_wrap_tokens_min": 6,
  "class_wrap_tokens_per_line": 2,
  "plugins": [
    {
      "path": "./node_modules/@fua-fua/plugin-tailwind/dist/fua_plugin_tailwind.wasm",
      "options": {
        "class_wrap_tokens_per_line": 2,
        "group_blank_lines": true
      }
    },
    {
      "path": "./node_modules/@fua-fua/plugin-angular/dist/fua_plugin_angular.wasm",
      "options": {
        "wrap_conditions_min": 2,
        "wrap_conditions_in_parens": true,
        "ngclass_wrap_entries_min": 2
      }
    }
  ]
}
```

Add scripts to your app `package.json`:

```json
{
  "scripts": {
    "format:html": "fua-fua --input \"src/**/*.html\" --config .fua/config.json",
    "format:html:file": "fua-fua --input \"src/app/app.component.html\" --config .fua/config.json --output \"src/app/app.component.html\""
  }
}
```

Run:

```bash
npm run format:html
```

### Optional: format only changed HTML files in pre-commit

Install Husky:

```bash
npm i -D husky
npx husky init
```

Put this in `.husky/pre-commit`:

```sh
#!/usr/bin/env sh
. "$(dirname -- "$0")/_/husky.sh"

git diff --cached --name-only --diff-filter=ACMR \
  | grep -E '\.html$' \
  | while read -r file; do
      npx fua-fua --input "$file" --config .fua/config.json --output "$file" || exit 1
      git add "$file"
    done
```

### Develop from Rust source

```bash
# Format a file and output to stdout
cargo run -p cli -- --input my_file.html

# Format a file explicitly overriding tab behavior and indent size
cargo run -p cli -- --input my_file.html --output formatted.html --use-tabs true

# Format via a configuration file
cargo run -p cli -- --input examples/sample.html --config examples/config.json --output examples/formatted.html
```

### CLI Arguments
* `-i, --input <pattern>`: Input glob pattern (repeatable). Examples: `src/**/*.html`, `templates/*.html`. Reads from `stdin` if not provided.
* `-o, --output <file>`: Output file path. Writes to `stdout` by default.
* `-c, --config <json file>`: Path to your formatting configuration definitions.
* `--indent-size <number>`: Override the indent size explicitly.
* `--use-tabs <bool>`: Override the whitespace strategy explicitly.
* `--plugin <file>`: Load a compiled WASM plugin. Repeat to load multiple plugins.

Multi-file behavior:
- If the input pattern resolves to **one** file, `--output` is supported.
- If the input pattern resolves to **multiple** files, formatting runs **in-place** for each file and `--output` is rejected.

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
  "class_wrap_tokens_min": 6,
  "class_wrap_tokens_per_line": 2,
  "plugins": [
    {
      "path": "../target/wasm32-wasip1/release/fua_plugin_tailwind.wasm",
      "options": {
        "class_wrap_tokens_per_line": 2,
        "group_blank_lines": true
      }
    },
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
* `class_wrap_tokens_min` *(Integer, Optional)*
  Wrap plain `class` attributes when they contain at least this many class tokens.
* `class_wrap_tokens_per_line` *(Integer, Default: 1)*
  Maximum number of class tokens emitted per line when a `class` attribute wraps.
* `plugins` *(Array, Default: empty)*
  Ordered list of optional WASM plugins to load after the default HTML formatter pass.
* `plugin` *(Object, Legacy)*
  Backward-compatible single-plugin entry. New configs should prefer `plugins`.
* `plugins[].options` *(Object, Plugin-specific)*
  Arbitrary JSON options forwarded to the selected plugin on each hook request.

### Tailwind Plugin

`fua-plugin-tailwind` formats plain `class` attributes. It sorts Tailwind utility tokens, groups related utilities with blank lines, and respects the formatter-level `class_wrap_tokens_min` / `class_wrap_tokens_per_line` values. Plugin options can override either class wrapping value:

```json
{
  "path": "../target/wasm32-wasip1/release/fua_plugin_tailwind.wasm",
  "options": {
    "class_wrap_tokens_min": 6,
    "class_wrap_tokens_per_line": 2,
    "group_blank_lines": true
  }
}
```

## Architecture

Fua Fua Format is split into four workspace crates:
- `fua-core`: Generic HTML lexer, parser, formatter, plugin host, and formatting engine.
- `fua-plugin-api`: Stable hook contract shared by the core host and every plugin crate.
- `fua-plugin-tailwind`: Tailwind class sorting, grouping, and grouped class wrapping.
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
- `crates/fua-plugin-tailwind/src/`: Tailwind-specific class token ordering and grouped class emission.
- `crates/fua-plugin-angular/src/`: Angular-specific attribute wrapping, condition handling, shared expression parsing helpers, response builders, and plugin state.

## Testing

Run these commands from the repository root:

```bash
# Run all workspace tests at once
cargo test --workspace

# Run tests for one crate
cargo test -p fua-core
cargo test -p fua-plugin-tailwind
cargo test -p fua-plugin-angular
cargo test -p cli

# Run a specific test by name
cargo test -p fua-plugin-angular wraps_conditions_across_config_sweep
```

Tip: append `-- --nocapture` to see `println!` output during tests.

## Troubleshooting

### Windows glob behavior

Windows shells do not expand globs before passing args to programs. `fua-fua` handles glob expansion internally, so patterns like `src/**/*.html` work cross-platform.

### Config path errors

If config loading fails, the CLI now prints:
- the raw path passed to `--config`
- the resolved absolute path
- current working directory

This makes it easier to spot wrong paths in CI, npm scripts, or monorepo subfolders.
