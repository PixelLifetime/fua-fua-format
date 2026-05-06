use crate::context::is_ngclass_attr;
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

    let (quote, inner_raw) = if text.starts_with('"') && text.ends_with('"') {
        ('"', &text[1..text.len() - 1])
    } else if text.starts_with('\'') && text.ends_with('\'') {
        ('\'', &text[1..text.len() - 1])
    } else {
        return None;
    };
    let normalized_inner = normalize_optional_chain_spacing(inner_raw);
    let inner = normalized_inner.as_deref().unwrap_or(inner_raw);
    let normalized_changed = normalized_inner.is_some();

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

    let bracket_binding = attr_name.trim().starts_with('[') && attr_name.trim().ends_with(']');
    if force_wrap_ngclass && !inner.contains('\n') {
        if let Some(wrapped) =
            force_wrap_ngclass_object(inner, current_indent, use_tabs, indent_size)
        {
            if bracket_binding {
                let normalized = normalize_object_block_for_binding(
                    &wrapped,
                    current_indent,
                    use_tabs,
                    indent_size,
                );
                return Some(wrap_binding_block(quote, &normalized, current_indent, use_tabs, indent_size));
            }
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

    let force_object_block = force_wrap_ngclass || (bracket_binding && looks_like_object_literal(inner));
    if let Some(wrapped) = format_object_literal(
        inner,
        min_ops,
        indent_size,
        use_tabs,
        wrap_in_parens,
        force_object_block,
    ) {
        if bracket_binding {
            let normalized =
                normalize_object_block_for_binding(&wrapped, current_indent, use_tabs, indent_size);
            return Some(wrap_binding_block(quote, &normalized, current_indent, use_tabs, indent_size));
        }
        return Some(format!("{quote}{wrapped}{quote}"));
    }

    if options
        .get("wrap_ternary")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        let ternary_target = if inner.contains('\n')
            && attr_name.trim().starts_with('[')
            && attr_name.trim().ends_with(']')
            && !is_ngclass_attr(attr_name)
        {
            inner.split_whitespace().collect::<Vec<_>>().join(" ")
        } else {
            inner.to_string()
        };

        if let Some(wrapped) = wrap_ternary(
            ternary_target.as_str(),
            min_ops,
            indent_size,
            use_tabs,
            current_indent,
            bracket_binding,
        ) {
            return Some(format!("{quote}{wrapped}{quote}"));
        }
    }

    if !inner.contains('\n') {
        return None;
    }

    let mut result_lines = Vec::new();
    let mut changed = normalized_changed;
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
    block_in_binding: bool,
) -> Option<String> {
    let (condition, then_expr, else_expr) = split_top_level_ternary(expr)?;
    let continuation = if use_tabs {
        "\t".repeat(current_indent + 2)
    } else {
        " ".repeat((current_indent + 2) * indent_size)
    };
    let binding_close_indent = if use_tabs {
        "\t".repeat(current_indent + 1)
    } else {
        " ".repeat((current_indent + 1) * indent_size)
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
                lines.join("\n")
            } else {
                let mut lines = vec![format!("{continuation}{}", parts[0].1)];
                for (op, item) in parts.into_iter().skip(1) {
                    lines.push(format!("{continuation}{op} {item}"));
                }
                lines.join("\n")
            }
        } else {
            if block_in_binding {
                format!("{continuation}{condition}")
            } else {
                condition
            }
        }
    } else {
        if block_in_binding {
            format!("{continuation}{condition}")
        } else {
            condition
        }
    };

    let body = format!("{wrapped_condition}\n{continuation}? {then_expr}\n{continuation}: {else_expr}");
    if block_in_binding {
        Some(format!("\n{body}\n{binding_close_indent}"))
    } else {
        Some(body)
    }
}

fn wrap_binding_block(
    quote: char,
    body: &str,
    current_indent: usize,
    use_tabs: bool,
    indent_size: usize,
) -> String {
    let binding_close_indent = if use_tabs {
        "\t".repeat(current_indent + 1)
    } else {
        " ".repeat((current_indent + 1) * indent_size)
    };
    format!("{quote}\n{body}\n{binding_close_indent}{quote}")
}

fn normalize_object_block_for_binding(
    wrapped: &str,
    current_indent: usize,
    use_tabs: bool,
    indent_size: usize,
) -> String {
    let object_indent = if use_tabs {
        "\t".repeat(current_indent + 2)
    } else {
        " ".repeat((current_indent + 2) * indent_size)
    };
    let entry_indent = if use_tabs {
        "\t".repeat(current_indent + 3)
    } else {
        " ".repeat((current_indent + 3) * indent_size)
    };

    wrapped
        .split('\n')
        .map(|line| {
            let content = line.trim();
            if content.starts_with('{') || content.starts_with('}') {
                format!("{object_indent}{content}")
            } else {
                format!("{entry_indent}{content}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn looks_like_object_literal(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.starts_with('{') && trimmed.ends_with('}')
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
                    let next = bytes.get(index + 1).copied().unwrap_or_default();
                    // Ignore optional chaining/nullish coalescing operators.
                    if next == b'.' || next == b'?' {
                        index += 1;
                        continue;
                    }
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

fn normalize_optional_chain_spacing(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut index = 0usize;
    let mut changed = false;

    while index < bytes.len() {
        if bytes[index] == b'?' {
            let mut lookahead = index + 1;
            while lookahead < bytes.len() && bytes[lookahead].is_ascii_whitespace() {
                lookahead += 1;
            }

            if lookahead < bytes.len() && (bytes[lookahead] == b'.' || bytes[lookahead] == b'?') {
                // Optional chaining cannot be split by whitespace/newlines before the `?`.
                // Normalize `foo \n ? .bar` and `foo ? .bar` to `foo?.bar`.
                while out
                    .as_bytes()
                    .last()
                    .is_some_and(|b| b.is_ascii_whitespace())
                {
                    out.pop();
                    changed = true;
                }
                out.push('?');
                out.push(bytes[lookahead] as char);
                changed |= lookahead > index + 1;
                index = lookahead + 1;
                continue;
            }
        }

        out.push(bytes[index] as char);
        index += 1;
    }

    changed.then_some(out)
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

#[cfg(test)]
mod tests {
    use super::process_attribute_string;
    use crate::expressions::{count_top_level_ops, split_on_top_level_ops, unwrap_negated_group};
    use serde_json::json;

    #[allow(dead_code)]
    struct NgClassEntry<'a> {
        key: &'a str,
        value: &'a str,
    }

    #[allow(dead_code)]
    fn generate_ngclass_expected(
        entries: &[NgClassEntry<'_>],
        wrap_conditions_min: usize,
        wrap_conditions_in_parens: bool,
        current_indent: usize,
        indent_size: usize,
        use_tabs: bool,
    ) -> String {
        let quote = '"';
        let quote_indent = if use_tabs {
            "\t".repeat(current_indent + 1)
        } else {
            " ".repeat((current_indent + 1) * indent_size)
        };
        let entry_indent = if use_tabs {
            "\t".repeat(current_indent + 2)
        } else {
            " ".repeat((current_indent + 2) * indent_size)
        };
        let group_indent = if use_tabs {
            format!("{entry_indent}\t")
        } else {
            format!("{entry_indent}{}", " ".repeat(indent_size))
        };
        let inner_indent = if use_tabs {
            format!("{group_indent}\t")
        } else {
            format!("{group_indent}{}", " ".repeat(indent_size))
        };

        let mut lines = vec!["{".to_string()];
        for (index, entry) in entries.iter().enumerate() {
            let is_last = index + 1 == entries.len();
            let suffix = if is_last { "" } else { "," };
            let negated = unwrap_negated_group(entry.value);
            let target_expr = negated.unwrap_or(entry.value);
            let should_wrap = wrap_conditions_in_parens
                && count_top_level_ops(target_expr) >= wrap_conditions_min.saturating_sub(1)
                && split_on_top_level_ops(target_expr).len() >= 2;

            if should_wrap {
                let parts = split_on_top_level_ops(target_expr);
                lines.push(format!("{entry_indent}{}:", entry.key));
                lines.push(format!("{group_indent}("));
                if negated.is_some() {
                    let neg_indent = if use_tabs {
                        format!("{group_indent}\t")
                    } else {
                        format!("{group_indent}{}", " ".repeat(indent_size))
                    };
                    let neg_inner_indent = if use_tabs {
                        format!("{neg_indent}\t")
                    } else {
                        format!("{neg_indent}{}", " ".repeat(indent_size))
                    };
                    lines.push(format!("{neg_indent}!("));
                    lines.push(format!("{neg_inner_indent}   {}", parts[0].1));
                    for (op, expr) in parts.iter().skip(1) {
                        lines.push(format!("{neg_inner_indent}{op} {expr}"));
                    }
                    lines.push(format!("{neg_indent})"));
                } else {
                    lines.push(format!("{inner_indent}   {}", parts[0].1));
                    for (op, expr) in parts.iter().skip(1) {
                        lines.push(format!("{inner_indent}{op} {expr}"));
                    }
                }
                lines.push(format!("{group_indent}){suffix}"));
            } else {
                lines.push(format!("{entry_indent}{}: {}{}", entry.key, entry.value, suffix));
            }
        }
        lines.push(format!("{entry_indent}}}"));

        format!("{quote}\n{}\n{quote_indent}{quote}", lines.join("\n"))
    }

    #[test]
    fn generates_ngclass_object_wrapping_like_real_template_case() {
        let _entries = [
            NgClassEntry {
                key: "'border-accent'",
                value: "this.hasNameChanges || this.hasDescriptionChanges || this.hasImageChanges",
            },
            NgClassEntry {
                key: "'border-quaternary'",
                value: "!(this.hasNameChanges || this.hasDescriptionChanges || this.hasImageChanges)",
            },
        ];
        let input = "\"{\n\t\t'border-accent': this.hasNameChanges || this.hasDescriptionChanges || this.hasImageChanges,\n\t\t'border-quaternary': !(this.hasNameChanges || this.hasDescriptionChanges || this.hasImageChanges)\n}\"";
        let options = json!({
            "wrap_conditions_min": 2,
            "wrap_conditions_in_parens": true,
            "ngclass_wrap_entries_min": 2
        });

        // Multiline ngClass uses existing leading indentation from the source text.
        let actual = process_attribute_string(input, "[ngClass]", &options, 0, 4, true);
        let expected = "\"\n\t\t{\n\t\t\t'border-accent':\n\t\t\t(\n\t\t\tthis.hasNameChanges\n\t\t\t|| this.hasDescriptionChanges\n\t\t\t|| this.hasImageChanges\n\t\t\t),\n\t\t\t'border-quaternary':\n\t\t\t(\n\t\t\t!(\n\t\t\tthis.hasNameChanges\n\t\t\t|| this.hasDescriptionChanges\n\t\t\t|| this.hasImageChanges\n\t\t\t)\n\t\t\t)\n\t\t}\n\t\"".to_string();
        assert_eq!(Some(expected), actual);
    }

    #[test]
    fn forces_single_line_ngclass_into_multiline_entries() {
        let input =
            "\"{'border-accent': this.hasImageChanges, 'border-quaternary': !this.hasImageChanges}\"";
        let options = json!({
            "ngclass_wrap_entries_min": 2
        });

        let actual = process_attribute_string(input, "[ngClass]", &options, 1, 4, true);
        let expected = "\"\n\t\t\t{\n\t\t\t\t'border-accent': this.hasImageChanges,\n\t\t\t\t'border-quaternary': !this.hasImageChanges\n\t\t\t}\n\t\t\"".to_string();

        assert_eq!(Some(expected), actual);
    }

    #[test]
    fn keeps_ngclass_unchanged_when_threshold_not_reached() {
        let input = "\"{'border-accent': this.hasImageChanges}\"";
        let options = json!({
            "ngclass_wrap_entries_min": 2,
            "wrap_conditions_min": 2,
            "wrap_conditions_in_parens": true
        });

        let actual = process_attribute_string(input, "[ngClass]", &options, 1, 4, true);
        let expected =
            "\"\n\t\t\t{\n\t\t\t\t'border-accent': this.hasImageChanges\n\t\t\t}\n\t\t\"".to_string();
        assert_eq!(Some(expected), actual);
    }

    #[test]
    fn wraps_ternary_without_leading_blank_line() {
        let input = "\"this.ready && this.visible ? 'yes' : 'no'\"";
        let options = json!({
            "wrap_ternary": true,
            "wrap_conditions_min": 2
        });

        let actual = process_attribute_string(input, "style.background-image", &options, 0, 4, true)
            .expect("ternary should be wrapped");
        let inner = &actual[1..actual.len() - 1];

        assert!(!inner.starts_with('\n'));
    }

    #[test]
    fn keeps_optional_chaining_ternary_valid() {
        let input = "\"this.playlist?.imageUrl ? 'url(' + this.playlist?.imageUrl + ')' : 'none'\"";
        let options = json!({
            "wrap_ternary": true,
            "wrap_conditions_min": 2
        });

        let actual =
            process_attribute_string(input, "style.background-image", &options, 0, 4, true)
                .expect("ternary should still be recognized");
        let inner = &actual[1..actual.len() - 1];

        assert!(inner.contains("this.playlist?.imageUrl"));
        assert!(inner.contains("? 'url(' + this.playlist?.imageUrl + ')'"));
        assert!(inner.contains(": 'none'"));
        assert!(!inner.contains("? .imageUrl"));
    }

    #[test]
    fn normalizes_broken_optional_chain_spacing_in_multiline_binding() {
        let input = "\"this.playlist\n\t\t? .imageUrl ? 'x' : 'none'\"";
        let options = json!({
            "wrap_ternary": true,
            "wrap_conditions_min": 2
        });

        let actual =
            process_attribute_string(input, "[style.background-image]", &options, 0, 4, true)
                .expect("normalization should produce an update");
        let inner = &actual[1..actual.len() - 1];

        assert!(inner.contains("?.imageUrl"));
        assert!(!inner.contains("playlist\n"));
        assert!(!inner.contains("? .imageUrl"));
    }

    #[test]
    fn wraps_multiline_ternary_in_bracket_binding_consistently() {
        let input = "\"this.isFavorited()\n? 'fa-solid fa-star text-yellow-500'\n: 'fa-regular fa-star'\"";
        let options = json!({
            "wrap_ternary": true,
            "wrap_conditions_min": 2
        });

        let actual = process_attribute_string(input, "[ngClass]", &options, 1, 4, true)
            .expect("ternary should be normalized and wrapped");
        let inner = &actual[1..actual.len() - 1];

        assert_eq!(
            inner,
            "\n\t\t\tthis.isFavorited()\n\t\t\t? 'fa-solid fa-star text-yellow-500'\n\t\t\t: 'fa-regular fa-star'\n\t\t"
        );
    }

    #[test]
    fn wraps_object_literal_in_bracket_binding_as_block() {
        let input = "\"{ invisible: !this.displayed, visible: this.displayed }\"";
        let options = json!({
            "wrap_conditions_min": 2,
            "wrap_conditions_in_parens": true
        });

        let actual = process_attribute_string(input, "[ngClass]", &options, 1, 4, true)
            .expect("object literal should be formatted");
        let inner = &actual[1..actual.len() - 1];

        assert!(inner.starts_with("\n\t\t\t{"));
        assert!(inner.contains("\n\t\t\t\tinvisible: !this.displayed,"));
        assert!(inner.contains("\n\t\t\t\tvisible: this.displayed"));
        assert!(inner.ends_with("\n\t\t"));
    }
}
