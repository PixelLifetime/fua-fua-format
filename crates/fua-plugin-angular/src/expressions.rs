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
