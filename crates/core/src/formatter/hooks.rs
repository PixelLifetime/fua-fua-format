use super::FormatSession;
use super::context::{ROOT_PARENT_KIND, node_tag_name, syntax_kind_label, token_text};
use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
use fua_plugin_api::{HookContext, HookRequest, LeadingSpacing, NodePhase, Replacement};
use rowan::NodeOrToken;

impl FormatSession {
    pub(super) fn try_handle_node(&mut self, node: &SyntaxNode, phase: NodePhase) -> bool {
        if self.plugin_host.is_empty() {
            return false;
        }

        let tag_name = node_tag_name(node);
        let text = node.to_string();
        let request = HookRequest::node(
            phase,
            syntax_kind_label(node.kind()),
            &text,
            tag_name.as_deref(),
            self.build_node_context(node, tag_name.as_deref()),
        );

        if let Some(replacement) = self.plugin_host.dispatch(&request) {
            self.write_replacement(replacement);
            return true;
        }

        false
    }

    pub(super) fn try_handle_token(
        &mut self,
        parent: &SyntaxNode,
        token: &SyntaxToken,
        attribute_name: Option<&str>,
    ) -> bool {
        if let Some(cached_replacement) = self.take_cached_token_replacement(token) {
            if let Some(replacement) = cached_replacement {
                self.write_replacement(replacement);
                return true;
            }

            return false;
        }

        if self.plugin_host.is_empty() {
            return false;
        }

        if let Some(replacement) = self.dispatch_token_request(parent, token, attribute_name) {
            self.write_replacement(replacement);
            return true;
        }

        false
    }

    fn build_node_context<'a>(
        &self,
        node: &SyntaxNode,
        tag_name: Option<&'a str>,
    ) -> HookContext<'a> {
        let parent_kind = node
            .parent()
            .map(|parent| syntax_kind_label(parent.kind()))
            .unwrap_or(ROOT_PARENT_KIND);

        HookContext::new(
            parent_kind,
            tag_name,
            None,
            self.current_indent,
            self.config.indent_size,
            self.config.use_tabs,
        )
    }

    pub(super) fn prepare_tag_token_replacements(&mut self, node: &SyntaxNode) {
        if self.plugin_host.is_empty()
            || !matches!(
                node.kind(),
                SyntaxKind::OPEN_TAG | SyntaxKind::SELF_CLOSING_TAG
            )
        {
            return;
        }

        let mut saw_tag_name = false;
        let mut current_attribute_name: Option<String> = None;

        for element in node.children_with_tokens() {
            let NodeOrToken::Token(token) = element else {
                continue;
            };

            let attribute_name = current_attribute_name.as_deref();
            self.cache_token_replacement(node, &token, attribute_name);

            if token.kind() == SyntaxKind::IDENT {
                if saw_tag_name {
                    current_attribute_name = Some(token.text().to_string());
                } else {
                    saw_tag_name = true;
                }
            }
        }
    }

    pub(super) fn node_has_multiline_cached_replacement(&self, node: &SyntaxNode) -> bool {
        node.descendants_with_tokens().any(|element| match element {
            NodeOrToken::Token(token) => self
                .cached_token_replacements
                .get(&token_cache_key(&token))
                .and_then(|replacement| replacement.as_ref())
                .is_some_and(replacement_forces_multiline),
            NodeOrToken::Node(_) => false,
        })
    }

    fn cache_token_replacement(
        &mut self,
        parent: &SyntaxNode,
        token: &SyntaxToken,
        attribute_name: Option<&str>,
    ) {
        if self
            .cached_token_replacements
            .contains_key(&token_cache_key(token))
        {
            return;
        }

        let replacement = self.dispatch_token_request(parent, token, attribute_name);
        self.cached_token_replacements
            .insert(token_cache_key(token), replacement);
    }

    fn dispatch_token_request(
        &mut self,
        parent: &SyntaxNode,
        token: &SyntaxToken,
        attribute_name: Option<&str>,
    ) -> Option<Replacement> {
        let tag_name = node_tag_name(parent);
        let previous = token.prev_sibling_or_token();
        let next = token.next_sibling_or_token();
        let request = HookRequest::token(
            syntax_kind_label(token.kind()),
            token.text(),
            HookContext::new(
                syntax_kind_label(parent.kind()),
                tag_name.as_deref(),
                attribute_name,
                self.current_indent,
                self.config.indent_size,
                self.config.use_tabs,
            )
            .with_neighbors(
                previous
                    .as_ref()
                    .map(|element| syntax_kind_label(element.kind())),
                previous.as_ref().and_then(token_text),
                next.as_ref()
                    .map(|element| syntax_kind_label(element.kind())),
                next.as_ref().and_then(token_text),
            ),
        );

        self.plugin_host.dispatch(&request)
    }

    fn take_cached_token_replacement(
        &mut self,
        token: &SyntaxToken,
    ) -> Option<Option<Replacement>> {
        self.cached_token_replacements
            .remove(&token_cache_key(token))
    }

    fn write_replacement(&mut self, replacement: Replacement) {
        self.adjust_indent(replacement.indent_before);
        self.apply_leading_spacing(replacement.leading_spacing);
        self.output.push_str(&replacement.output);
        self.adjust_indent(replacement.indent_after);
    }

    fn apply_leading_spacing(&mut self, spacing: LeadingSpacing) {
        match spacing {
            LeadingSpacing::None => {}
            LeadingSpacing::Space => self.ensure_space(),
            LeadingSpacing::LineBreak => self.push_newlines_with_indent(1),
            LeadingSpacing::BlankLine => self.push_newlines_with_indent(2),
        }
    }
}

fn token_cache_key(token: &SyntaxToken) -> (u32, u32) {
    let range = token.text_range();
    (range.start().into(), range.end().into())
}

fn replacement_forces_multiline(replacement: &Replacement) -> bool {
    replacement.output.contains('\n')
        || matches!(
            replacement.leading_spacing,
            LeadingSpacing::LineBreak | LeadingSpacing::BlankLine
        )
}
