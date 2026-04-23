use super::FormatSession;
use super::context::{is_raw_text_element, structure_is_simple};
use crate::plugins::PluginHost;
use crate::syntax::{SyntaxKind, SyntaxNode};
use fua_plugin_api::NodePhase;
use rowan::NodeOrToken;

impl FormatSession {
    pub(super) fn format(mut self, root: &SyntaxNode) -> String {
        self.format_node(root, false);
        self.finish()
    }

    fn format_node(&mut self, node: &SyntaxNode, inline_mode: bool) {
        if self.try_handle_node(node, NodePhase::Enter) {
            return;
        }

        match node.kind() {
            SyntaxKind::ROOT => self.format_children(node, inline_mode),
            SyntaxKind::ELEMENT => self.format_element(node, inline_mode),
            SyntaxKind::OPEN_TAG => self.format_open_tag(node, inline_mode),
            SyntaxKind::CLOSE_TAG => self.format_close_tag(node, inline_mode),
            SyntaxKind::SELF_CLOSING_TAG => self.format_self_closing_tag(node, inline_mode),
            _ => self.format_children(node, inline_mode),
        }

        self.try_handle_node(node, NodePhase::Exit);
    }

    fn format_children(&mut self, node: &SyntaxNode, inline_mode: bool) {
        for element in node.children_with_tokens() {
            match element {
                NodeOrToken::Node(child) => self.format_nested_node(&child, inline_mode),
                NodeOrToken::Token(ref token) => {
                    self.format_content_token(node, &element, token, inline_mode)
                }
            }
        }
    }

    fn format_element(&mut self, node: &SyntaxNode, inherited_inline: bool) {
        if is_raw_text_element(node) {
            self.format_raw_text_element(node, inherited_inline);
            return;
        }

        let inline_mode = self.inline_mode_for(node, inherited_inline);
        let has_close_tag = self.has_close_tag(node);
        let starting_indent = self.current_indent;

        for element in node.children_with_tokens() {
            match element {
                NodeOrToken::Node(child) => match child.kind() {
                    SyntaxKind::OPEN_TAG => {
                        let tag_inline = self.inline_mode_for(&child, inline_mode);
                        self.format_open_tag(&child, tag_inline);
                        if has_close_tag && !inline_mode {
                            self.current_indent += 1;
                        }
                    }
                    SyntaxKind::CLOSE_TAG => {
                        if has_close_tag && !inline_mode {
                            self.current_indent = self.current_indent.saturating_sub(1);
                            self.push_newlines_with_indent(1);
                        }
                        let tag_inline = self.inline_mode_for(&child, inline_mode);
                        self.format_close_tag(&child, tag_inline);
                    }
                    SyntaxKind::SELF_CLOSING_TAG => {
                        let tag_inline = self.inline_mode_for(&child, inline_mode);
                        self.format_self_closing_tag(&child, tag_inline)
                    }
                    _ => self.format_nested_node(&child, inline_mode),
                },
                NodeOrToken::Token(ref token) => {
                    self.format_content_token(node, &element, token, inline_mode)
                }
            }
        }

        self.current_indent = starting_indent;
    }

    fn format_raw_text_element(&mut self, node: &SyntaxNode, inherited_inline: bool) {
        let inline_mode = self.inline_mode_for(node, inherited_inline);
        let starting_indent = self.current_indent;
        let mut raw_content = String::new();
        let mut pending_close_tag = None;

        for element in node.children_with_tokens() {
            match element {
                NodeOrToken::Node(child) => match child.kind() {
                    SyntaxKind::OPEN_TAG => {
                        let tag_inline = self.inline_mode_for(&child, inline_mode);
                        self.format_open_tag(&child, tag_inline);
                    }
                    SyntaxKind::CLOSE_TAG => {
                        pending_close_tag = Some(child);
                    }
                    SyntaxKind::SELF_CLOSING_TAG => {
                        let tag_inline = self.inline_mode_for(&child, inline_mode);
                        self.format_self_closing_tag(&child, tag_inline);
                    }
                    _ => raw_content.push_str(&child.to_string()),
                },
                NodeOrToken::Token(ref token) => raw_content.push_str(token.text()),
            }
        }

        if !raw_content.is_empty() {
            self.write_raw_text_content(&raw_content);
        }

        if let Some(close_tag) = pending_close_tag {
            let tag_inline = self.inline_mode_for(&close_tag, inline_mode);
            self.format_close_tag(&close_tag, tag_inline);
        } else {
            for element in node.children_with_tokens() {
                if let NodeOrToken::Token(ref token) = element {
                    self.format_content_token(node, &element, token, inline_mode);
                }
            }
        }

        self.current_indent = starting_indent;
    }

    fn format_nested_node(&mut self, node: &SyntaxNode, inline_mode: bool) {
        let child_inline = self.inline_mode_for(node, inline_mode);

        if self.should_break_before_child(node, inline_mode, child_inline) {
            self.push_newlines_with_indent(1);
        }

        self.format_node(node, child_inline);
    }

    pub(super) fn inline_mode_for(&mut self, node: &SyntaxNode, inherited_inline: bool) -> bool {
        self.prepare_inline_layout(node);

        (inherited_inline || self.should_inline_node(node))
            && !self.node_has_multiline_cached_replacement(node)
    }

    fn should_inline_node(&self, node: &SyntaxNode) -> bool {
        structure_is_simple(node)
            && self.inline_rendered_width(node) <= self.available_inline_width()
    }

    fn prepare_inline_layout(&mut self, node: &SyntaxNode) {
        match node.kind() {
            SyntaxKind::ELEMENT => {
                for child in node.children() {
                    if matches!(
                        child.kind(),
                        SyntaxKind::OPEN_TAG | SyntaxKind::SELF_CLOSING_TAG
                    ) {
                        self.prepare_tag_token_replacements(&child);
                    }
                }
            }
            SyntaxKind::OPEN_TAG | SyntaxKind::SELF_CLOSING_TAG => {
                self.prepare_tag_token_replacements(node);
            }
            _ => {}
        }
    }

    fn available_inline_width(&self) -> usize {
        self.config
            .print_width
            .saturating_sub(self.current_line_width())
    }

    fn inline_rendered_width(&self, node: &SyntaxNode) -> usize {
        let mut preview = FormatSession::new(self.config.clone(), PluginHost::new());
        preview.current_indent = self.current_indent;
        preview.format_node(node, true);

        if preview.output.contains('\n') {
            usize::MAX
        } else {
            preview.output.chars().count()
        }
    }

    fn has_close_tag(&self, node: &SyntaxNode) -> bool {
        node.children()
            .any(|child| child.kind() == SyntaxKind::CLOSE_TAG)
    }

    fn should_break_before_child(
        &self,
        node: &SyntaxNode,
        inline_mode: bool,
        child_inline: bool,
    ) -> bool {
        matches!(node.kind(), SyntaxKind::ELEMENT) && !inline_mode && !child_inline
    }
}
