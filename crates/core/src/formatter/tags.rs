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
        let inline_mode = self.inline_mode_for(node, inline_mode);
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
            SyntaxKind::STRING_DOUBLE | SyntaxKind::STRING_SINGLE => {
                self.format_attribute_string(token.text(), state.attribute_name())
            }
            SyntaxKind::CLOSE_ANGLE | SyntaxKind::SLASH_CLOSE_ANGLE => {
                self.format_tag_closing_bracket(node.kind(), element, token.text(), inline_mode)
            }
            _ => self.output.push_str(token.text()),
        }
    }

    fn format_attribute_spacing(&mut self, element: &SyntaxElement, inline_mode: bool) {
        let should_wrap_attributes = self.config.wrap_attributes || !inline_mode;
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

            if should_wrap_attributes {
                self.push_newlines_with_indent(1);
                return;
            }
        }

        if should_wrap_attributes {
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
        let should_wrap_attributes = self.config.wrap_attributes || !inline_mode;

        if matches!(
            node_kind,
            SyntaxKind::OPEN_TAG | SyntaxKind::SELF_CLOSING_TAG
        ) && !self.config.bracket_same_line
            && should_wrap_attributes
            && !prev_is_whitespace
            && !inline_mode
        {
            self.push_newlines_with_indent(1);
        }

        self.output.push_str(text);
    }
}
