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
