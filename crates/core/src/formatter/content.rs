use super::FormatSession;
use super::context::{is_after_open_tag, is_before_close_tag, is_raw_text_element};
use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

impl FormatSession {
    pub(super) fn format_content_token(
        &mut self,
        parent: &SyntaxNode,
        element: &SyntaxElement,
        token: &SyntaxToken,
        inline_mode: bool,
    ) {
        if is_raw_text_element(parent) {
            self.output.push_str(token.text());
            return;
        }

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

    fn should_omit_content_whitespace(&mut self, next: Option<&SyntaxElement>) -> bool {
        next.is_some_and(|sibling| {
            sibling.kind() == SyntaxKind::CLOSE_TAG
                || sibling.as_node().is_some_and(|node| {
                    node.kind() == SyntaxKind::ELEMENT && !self.inline_mode_for(node, false)
                })
        })
    }

    fn should_wrap_content_whitespace(
        &self,
        element: &SyntaxElement,
        text: &str,
        inline_mode: bool,
    ) -> bool {
        (text.contains('\n') || is_after_open_tag(element)) && !inline_mode
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
        parent.kind() == SyntaxKind::ELEMENT && is_after_open_tag(element) && !inline_mode
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

    pub(super) fn write_raw_text_content(&mut self, raw_content: &str) {
        let Some(block_lines) = raw_text_block_lines(raw_content) else {
            self.output.push_str(raw_content);
            return;
        };

        let common_indent = common_leading_whitespace(&block_lines);
        let content_indent = self.current_indent + 1;

        self.output.push('\n');

        for (index, line) in block_lines.iter().enumerate() {
            if index > 0 {
                self.output.push('\n');
            }

            let line = line.trim_end_matches('\r');
            if line.trim().is_empty() {
                continue;
            }

            self.push_indent_at(content_indent);
            let rebased = line.strip_prefix(&common_indent).unwrap_or(line);
            self.output.push_str(rebased);
        }

        self.push_newlines_with_indent(1);
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

fn raw_text_block_lines(raw_content: &str) -> Option<Vec<&str>> {
    let body = raw_content
        .strip_prefix("\r\n")
        .or_else(|| raw_content.strip_prefix('\n'))?;
    let trailing_newline = body.rfind('\n')?;
    let trailing_indent = &body[trailing_newline + 1..];
    if !trailing_indent.trim().is_empty() {
        return None;
    }

    let body = &body[..trailing_newline];
    Some(body.split('\n').collect())
}

fn common_leading_whitespace(lines: &[&str]) -> String {
    let mut prefixes = lines
        .iter()
        .map(|line| line.trim_end_matches('\r'))
        .filter(|line| !line.trim().is_empty())
        .map(leading_whitespace);

    let Some(mut common) = prefixes.next().map(ToString::to_string) else {
        return String::new();
    };

    for prefix in prefixes {
        while !prefix.starts_with(&common) {
            common.pop();
            if common.is_empty() {
                break;
            }
        }
    }

    common
}

fn leading_whitespace(line: &str) -> &str {
    let width = line
        .bytes()
        .take_while(|byte| matches!(byte, b' ' | b'\t'))
        .count();
    &line[..width]
}
