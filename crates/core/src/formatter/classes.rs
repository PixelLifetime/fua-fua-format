use super::FormatSession;

impl FormatSession {
    pub(super) fn format_attribute_string(&mut self, text: &str, attribute_name: Option<&str>) {
        if is_class_attribute(attribute_name) {
            if let Some(output) = self.format_wrapped_class_string(text) {
                self.output.push_str(&output);
                return;
            }
        }

        match text.chars().next() {
            Some('"') => self.format_double_quoted_string(text),
            Some('\'') => self.format_single_quoted_string(text),
            _ => self.output.push_str(text),
        }
    }

    fn format_wrapped_class_string(&self, text: &str) -> Option<String> {
        let min_tokens = self.config.class_wrap_tokens_min?;
        let (quote, inner) = quoted_inner(text)?;

        if inner.contains('\n') {
            return None;
        }

        let tokens: Vec<&str> = inner.split_whitespace().collect();
        if tokens.is_empty() || tokens.len() < min_tokens {
            return None;
        }

        let token_indent = self.indent_string_at(self.current_indent + 2);
        let close_indent = self.indent_string_at(self.current_indent + 1);
        let tokens_per_line = self.config.class_wrap_tokens_per_line.max(1);
        let quote = self.output_quote(quote);
        let lines = tokens
            .chunks(tokens_per_line)
            .map(|chunk| chunk.join(" "))
            .collect::<Vec<_>>();

        Some(quote_wrapped_lines(
            quote,
            lines,
            &token_indent,
            &close_indent,
        ))
    }

    fn output_quote(&self, input_quote: char) -> char {
        match (input_quote, self.config.single_quotes) {
            ('"', true) => '\'',
            ('\'', false) => '"',
            _ => input_quote,
        }
    }

    pub(super) fn indent_string_at(&self, depth: usize) -> String {
        if self.config.use_tabs {
            "\t".repeat(depth)
        } else {
            " ".repeat(depth * self.config.indent_size)
        }
    }
}

fn is_class_attribute(attribute_name: Option<&str>) -> bool {
    attribute_name.is_some_and(|name| name.trim().eq_ignore_ascii_case("class"))
}

fn quoted_inner(text: &str) -> Option<(char, &str)> {
    if text.len() < 2 {
        return None;
    }

    let quote = text.chars().next()?;
    if !matches!(quote, '"' | '\'') || !text.ends_with(quote) {
        return None;
    }

    Some((quote, &text[1..text.len() - 1]))
}

fn quote_wrapped_lines(
    quote: char,
    lines: Vec<String>,
    token_indent: &str,
    close_indent: &str,
) -> String {
    let escaped_lines = lines
        .into_iter()
        .map(|line| escape_for_quote(&line, quote))
        .collect::<Vec<_>>();

    let mut out = String::new();
    out.push(quote);
    out.push('\n');

    for line in escaped_lines {
        out.push_str(token_indent);
        out.push_str(&line);
        out.push('\n');
    }

    out.push_str(close_indent);
    out.push(quote);
    out
}

fn escape_for_quote(value: &str, quote: char) -> String {
    match quote {
        '"' => value.replace('"', "&quot;"),
        '\'' => value.replace('\'', "&apos;"),
        _ => value.to_string(),
    }
}
