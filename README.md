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
  "wrap_content": true
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

## Architecture

Fua Fua Format consists of two primary workspace crates:
- `core`: Houses the Logos tokenizer (`lexer.rs`), the string tree parser (`parser.rs`), the configuration definitions (`config.rs`), and the top-down indent tree walker formatting engine (`formatter.rs`).
- `cli`: Houses the fast Clap CLI command interface bridging parameters linearly into the `core` parser.
