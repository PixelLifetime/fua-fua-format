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
