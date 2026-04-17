use crate::config::FormatterConfig;
use crate::plugins::{NodeData, PluginManager, PluginResult};
use crate::syntax::{SyntaxKind, SyntaxNode};
use rowan::NodeOrToken;

fn is_block_element(_tag_name: &str) -> bool {
    true
}

fn get_tag_name(node: &SyntaxNode) -> Option<String> {
    for element in node.children_with_tokens() {
        if let NodeOrToken::Node(n) = element {
            if n.kind() == SyntaxKind::OPEN_TAG.into() {
                for tag_element in n.children_with_tokens() {
                    if let NodeOrToken::Token(t) = tag_element {
                        if t.kind() == SyntaxKind::IDENT.into() {
                            return Some(t.text().to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn kind_label(kind: SyntaxKind) -> &'static str {
    match kind {
        SyntaxKind::ELEMENT          => "ELEMENT",
        SyntaxKind::ROOT             => "ROOT",
        SyntaxKind::OPEN_TAG         => "OPEN_TAG",
        SyntaxKind::CLOSE_TAG        => "CLOSE_TAG",
        SyntaxKind::SELF_CLOSING_TAG => "SELF_CLOSING_TAG",
        _                            => "OTHER",
    }
}

pub struct Formatter {
    config: FormatterConfig,
    output: String,
    current_indent: usize,
    plugin_manager: PluginManager,
    /// Most-recently-seen IDENT inside an opening tag (for attribute-name hint).
    last_attr_name: String,
    /// Cached JSON of `config.plugin.options`.
    plugin_options_json: Option<String>,

    // ── Angular @if / @for condition state ────────────────────────────────────
    /// Depth of `(` chars seen after an Angular block-opener IDENT.
    /// 0 = not inside a condition; 1 = directly inside `@if (`; etc.
    angular_cond_depth: usize,
    /// The `current_indent` value at the moment the block-opener was emitted.
    /// Condition lines are indented relative to this value.
    angular_cond_base_indent: usize,
    /// True immediately after emitting an Angular block-opener (`@if`, `@for`…)
    /// so the next TEXT `(` token triggers condition-mode.
    just_saw_block_opener: bool,
}

impl Formatter {
    pub fn new(config: FormatterConfig) -> Self {
        let plugin_options_json = config.plugin.options.as_ref().map(|v| v.to_string());
        Self {
            config,
            output: String::new(),
            current_indent: 0,
            plugin_manager: PluginManager::new(),
            last_attr_name: String::new(),
            plugin_options_json,
            angular_cond_depth: 0,
            angular_cond_base_indent: 0,
            just_saw_block_opener: false,
        }
    }

    pub fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.plugin_manager.load_wasm(path)
    }

    pub fn get_indent(&self) -> String {
        if self.config.use_tabs {
            "\t".repeat(self.current_indent)
        } else {
            " ".repeat(self.current_indent * self.config.indent_size)
        }
    }

    fn indent_str(n: usize, use_tabs: bool, indent_size: usize) -> String {
        if use_tabs { "\t".repeat(n) } else { " ".repeat(n * indent_size) }
    }

    fn push_newlines_with_indent(&mut self, count: usize) {
        let mut trim_len = self.output.len();
        for c in self.output.chars().rev() {
            if c == ' ' || c == '\t' {
                trim_len -= c.len_utf8();
            } else {
                break;
            }
        }
        self.output.truncate(trim_len);

        let mut trailing_newlines = 0;
        for c in self.output.chars().rev() {
            if c == '\n' {
                trailing_newlines += 1;
            } else {
                break;
            }
        }

        while trailing_newlines < count {
            self.output.push('\n');
            trailing_newlines += 1;
        }
        self.output.push_str(&self.get_indent());
    }

    fn emit_plugin_result(&mut self, result: PluginResult) {
        if result.indent_delta < 0 {
            self.current_indent = self
                .current_indent
                .saturating_sub((-result.indent_delta) as usize);
        }
        if result.prepend_newline {
            self.push_newlines_with_indent(1);
        } else if result.prepend_space {
            let last = self.output.chars().next_back().unwrap_or('\0');
            if last != ' ' && last != '\t' && last != '\n' {
                self.output.push(' ');
            }
        }
        self.output.push_str(&result.output);
        if result.indent_delta > 0 {
            self.current_indent += result.indent_delta as usize;
        }
    }

    fn make_node_data<'a>(
        kind: &'a str,
        text: &'a str,
        parent_kind: &'a str,
        attr_name: &'a str,
        plugin_options: Option<&'a str>,
        current_indent: usize,
        indent_size: usize,
        use_tabs: bool,
    ) -> NodeData<'a> {
        NodeData {
            kind,
            text,
            parent_kind,
            attribute_name: attr_name,
            current_indent,
            indent_size,
            use_tabs,
            plugin_options,
        }
    }

    pub fn format(mut self, root: &SyntaxNode) -> String {
        self.format_node_internal(root, false);
        self.output
    }

    fn format_node_internal(&mut self, node: &SyntaxNode, force_inline: bool) {
        let prev_indent = self.current_indent;
        let is_block_elem = node.kind() == SyntaxKind::ELEMENT.into()
            && get_tag_name(node)
                .as_deref()
                .map_or(false, is_block_element);

        for element in node.children_with_tokens() {
            match element {
                NodeOrToken::Node(n) => {
                    if n.kind() == SyntaxKind::ELEMENT.into() {
                        let child_is_block =
                            get_tag_name(&n).as_deref().map_or(false, is_block_element);
                        let force_inline_child = is_element_simple_and_short(
                            &n,
                            self.config.inline_short_elements_max_len,
                        );

                        if child_is_block && !force_inline && !force_inline_child {
                            self.push_newlines_with_indent(1);
                        }

                        self.format_node_internal(&n, force_inline || force_inline_child);
                    } else if n.kind() == SyntaxKind::OPEN_TAG.into() {
                        self.format_node_internal(&n, force_inline);
                        if is_block_elem {
                            self.current_indent += 1;
                        }
                    } else if n.kind() == SyntaxKind::CLOSE_TAG.into() {
                        if is_block_elem {
                            self.current_indent = self.current_indent.saturating_sub(1);
                            if !force_inline {
                                self.push_newlines_with_indent(1);
                            }
                        }
                        self.format_node_internal(&n, force_inline);
                    } else {
                        self.format_node_internal(&n, force_inline);
                    }
                }

                NodeOrToken::Token(ref t) => {
                    let text = t.text();
                    let kind = t.kind();
                    let is_content = node.kind() == SyntaxKind::ELEMENT.into()
                        || node.kind() == SyntaxKind::ROOT.into();

                    // ── WHITESPACE ───────────────────────────────────────────
                    if kind == SyntaxKind::WHITESPACE.into() {
                        if node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into()
                        {
                            let next = element.next_sibling_or_token();
                            let next_is_bracket = next.as_ref().map_or(false, |e| {
                                e.kind() == SyntaxKind::CLOSE_ANGLE.into()
                                    || e.kind() == SyntaxKind::SLASH_CLOSE_ANGLE.into()
                            });

                            if next_is_bracket {
                                if self.config.bracket_same_line || force_inline {
                                    if next.as_ref().unwrap().kind()
                                        == SyntaxKind::SLASH_CLOSE_ANGLE.into()
                                    {
                                        self.output.push(' ');
                                    }
                                    continue;
                                } else if self.config.wrap_attributes {
                                    self.push_newlines_with_indent(1);
                                    continue;
                                }
                            }

                            if text.contains('\n') || self.config.wrap_attributes {
                                if force_inline {
                                    self.output.push(' ');
                                } else {
                                    self.push_newlines_with_indent(1);
                                    if self.config.use_tabs {
                                        self.output.push('\t');
                                    } else {
                                        self.output.push_str(
                                            &" ".repeat(self.config.indent_size),
                                        );
                                    }
                                }
                            } else {
                                self.output.push(' ');
                            }
                        } else if node.kind() == SyntaxKind::CLOSE_TAG.into() {
                            // drop whitespace inside `</div >`
                        } else {
                            // ── Content whitespace ───────────────────────────
                            let has_nl = text.contains('\n');

                            // Inside an Angular @if / @for condition paren:
                            // optionally indent based on the current paren depth.
                            if self.angular_cond_depth > 0 && has_nl {
                                let next = element.next_sibling_or_token();
                                let next_is_close_paren =
                                    next.as_ref().map_or(false, |e| match e {
                                        NodeOrToken::Token(t) => {
                                            t.kind() == SyntaxKind::TEXT.into()
                                                && t.text() == ")"
                                        }
                                        _ => false,
                                    });

                                let target = if self.config.indent_condition_groups {
                                    self.angular_cond_base_indent
                                        + self.angular_cond_depth
                                        - usize::from(next_is_close_paren)
                                } else if next_is_close_paren {
                                    self.angular_cond_base_indent
                                } else {
                                    self.angular_cond_base_indent + 1
                                };

                                let saved = self.current_indent;
                                self.current_indent = target;
                                self.push_newlines_with_indent(1);
                                self.current_indent = saved;
                                continue;
                            }

                            let next = element.next_sibling_or_token();
                            let next_text = next.as_ref().and_then(|e| {
                                if let NodeOrToken::Token(nt) = e {
                                    Some(nt.text())
                                } else {
                                    None
                                }
                            });

                            let newlines = text.matches('\n').count();

                            if self.plugin_manager.has_plugin()
                                && next_text.map(|t| t.starts_with('@')).unwrap_or(false)
                            {
                                if newlines > 1 {
                                    self.push_newlines_with_indent(2);
                                }
                                continue;
                            }

                            let next_is_block = next.as_ref().map_or(false, |e| {
                                if e.kind() == SyntaxKind::ELEMENT.into() {
                                    let n = e.as_node().unwrap();
                                    !is_element_simple_and_short(
                                        n,
                                        self.config.inline_short_elements_max_len,
                                    )
                                } else {
                                    false
                                }
                            });

                            let is_next_close_tag_of_block = next
                                .as_ref()
                                .map_or(false, |e| e.kind() == SyntaxKind::CLOSE_TAG.into());

                            if next_is_block || is_next_close_tag_of_block {
                                if newlines > 1 {
                                    self.push_newlines_with_indent(2);
                                }
                                continue;
                            }

                            let mut prev_non_ws = element.prev_sibling_or_token();
                            while let Some(p) = &prev_non_ws {
                                if p.kind() == SyntaxKind::WHITESPACE.into() {
                                    prev_non_ws = p.prev_sibling_or_token();
                                } else {
                                    break;
                                }
                            }
                            let is_after_open_tag = prev_non_ws
                                .as_ref()
                                .map_or(false, |e| e.kind() == SyntaxKind::OPEN_TAG.into());

                            if (has_nl
                                || (is_after_open_tag && self.config.wrap_content))
                                && !force_inline
                            {
                                if newlines > 1 {
                                    self.push_newlines_with_indent(2);
                                } else {
                                    self.push_newlines_with_indent(1);
                                }
                            } else if force_inline {
                                let is_before_close_tag = next.as_ref().map_or(false, |e| {
                                    e.kind() == SyntaxKind::CLOSE_TAG.into()
                                });
                                if !is_after_open_tag && !is_before_close_tag {
                                    self.output.push(' ');
                                }
                            } else {
                                self.output.push(' ');
                            }
                        }
                        continue;

                    // ── STRING DOUBLE ─────────────────────────────────────────
                    } else if kind == SyntaxKind::STRING_DOUBLE.into() {
                        let in_tag = node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into();

                        if in_tag && self.plugin_manager.has_plugin() {
                            let attr = self.last_attr_name.clone();
                            let opts = self.plugin_options_json.clone();
                            let node_data = Self::make_node_data(
                                "STRING_DOUBLE",
                                text,
                                kind_label(node.kind()),
                                &attr,
                                opts.as_deref(),
                                self.current_indent,
                                self.config.indent_size,
                                self.config.use_tabs,
                            );
                            if let Some(result) =
                                self.plugin_manager.call_format_hook(&node_data)
                            {
                                self.emit_plugin_result(result);
                                continue;
                            }
                        }

                        if self.config.single_quotes && text.len() >= 2 {
                            let inner = &text[1..text.len() - 1];
                            let escaped = inner.replace("'", "&apos;");
                            self.output.push('\'');
                            self.output.push_str(&escaped);
                            self.output.push('\'');
                        } else {
                            self.output.push_str(text);
                        }

                    // ── STRING SINGLE ─────────────────────────────────────────
                    } else if kind == SyntaxKind::STRING_SINGLE.into() {
                        let in_tag = node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into();

                        if in_tag && self.plugin_manager.has_plugin() {
                            let attr = self.last_attr_name.clone();
                            let opts = self.plugin_options_json.clone();
                            let node_data = Self::make_node_data(
                                "STRING_SINGLE",
                                text,
                                kind_label(node.kind()),
                                &attr,
                                opts.as_deref(),
                                self.current_indent,
                                self.config.indent_size,
                                self.config.use_tabs,
                            );
                            if let Some(result) =
                                self.plugin_manager.call_format_hook(&node_data)
                            {
                                self.emit_plugin_result(result);
                                continue;
                            }
                        }

                        if !self.config.single_quotes && text.len() >= 2 {
                            let inner = &text[1..text.len() - 1];
                            let escaped = inner.replace("\"", "&quot;");
                            self.output.push('"');
                            self.output.push_str(&escaped);
                            self.output.push('"');
                        } else {
                            self.output.push_str(text);
                        }

                    // ── CLOSE ANGLE / SELF-CLOSE ──────────────────────────────
                    } else if kind == SyntaxKind::CLOSE_ANGLE.into()
                        || kind == SyntaxKind::SLASH_CLOSE_ANGLE.into()
                    {
                        let prev = element.prev_sibling_or_token();
                        let prev_is_whitespace = prev
                            .as_ref()
                            .map_or(false, |e| e.kind() == SyntaxKind::WHITESPACE.into());

                        let in_opening = node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into();
                        if in_opening
                            && !self.config.bracket_same_line
                            && self.config.wrap_attributes
                        {
                            if !prev_is_whitespace && !force_inline {
                                self.push_newlines_with_indent(1);
                            }
                        }
                        self.output.push_str(text);

                    // ── COMMENT ───────────────────────────────────────────────
                    } else if kind == SyntaxKind::COMMENT.into() {
                        self.push_newlines_with_indent(1);
                        self.output.push_str(text);

                    // ── IDENT ─────────────────────────────────────────────────
                    } else if kind == SyntaxKind::IDENT.into() {
                        let in_tag = node.kind() == SyntaxKind::OPEN_TAG.into()
                            || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into();

                        if in_tag {
                            self.last_attr_name = text.to_string();
                        }

                        let is_elem_or_root = node.kind() == SyntaxKind::ELEMENT.into()
                            || node.kind() == SyntaxKind::ROOT.into();

                        let mut prev_non_ws = element.prev_sibling_or_token();
                        while let Some(p) = &prev_non_ws {
                            if p.kind() == SyntaxKind::WHITESPACE.into() {
                                prev_non_ws = p.prev_sibling_or_token();
                            } else {
                                break;
                            }
                        }
                        let is_after_open_tag = prev_non_ws
                            .as_ref()
                            .map_or(false, |e| e.kind() == SyntaxKind::OPEN_TAG.into());

                        if is_after_open_tag && self.config.wrap_content && !force_inline {
                            self.push_newlines_with_indent(1);
                        }

                        if is_elem_or_root {
                            let opts = self.plugin_options_json.clone();
                            let node_data = Self::make_node_data(
                                "IDENT",
                                text,
                                kind_label(node.kind()),
                                "",
                                opts.as_deref(),
                                self.current_indent,
                                self.config.indent_size,
                                self.config.use_tabs,
                            );
                            if let Some(result) =
                                self.plugin_manager.call_format_hook(&node_data)
                            {
                                // If the plugin pushed to a new line (block opener like @if),
                                // the next `(` starts the condition state machine.
                                if result.prepend_newline {
                                    self.just_saw_block_opener = true;
                                }
                                self.emit_plugin_result(result);
                                continue;
                            }
                        }

                        self.output.push_str(text);

                    // ── TEXT (content nodes) ──────────────────────────────────
                    } else if kind == SyntaxKind::TEXT.into() {
                        let is_elem_or_root = node.kind() == SyntaxKind::ELEMENT.into()
                            || node.kind() == SyntaxKind::ROOT.into();

                        if is_elem_or_root {
                            // ── Angular condition paren tracking ──────────────
                            if text == "(" {
                                if self.just_saw_block_opener {
                                    // `@if (` — enter condition mode
                                    self.just_saw_block_opener = false;
                                    self.angular_cond_depth = 1;
                                    self.angular_cond_base_indent = self.current_indent;
                                } else if self.angular_cond_depth > 0 {
                                    self.angular_cond_depth += 1;
                                }
                                self.output.push('(');
                                continue;
                            }

                            if text == ")" && self.angular_cond_depth > 0 {
                                self.angular_cond_depth =
                                    self.angular_cond_depth.saturating_sub(1);
                                self.output.push(')');
                                continue;
                            }

                            // ── Plugin hook for other TEXT tokens ─────────────
                            let opts = self.plugin_options_json.clone();
                            let node_data = Self::make_node_data(
                                "TEXT",
                                text,
                                kind_label(node.kind()),
                                "",
                                opts.as_deref(),
                                self.current_indent,
                                self.config.indent_size,
                                self.config.use_tabs,
                            );
                            if let Some(result) =
                                self.plugin_manager.call_format_hook(&node_data)
                            {
                                self.emit_plugin_result(result);
                                continue;
                            }
                        }

                        // ── Default TEXT handling (collapse whitespace) ────────
                        let mut prev_non_ws = element.prev_sibling_or_token();
                        while let Some(p) = &prev_non_ws {
                            if p.kind() == SyntaxKind::WHITESPACE.into() {
                                prev_non_ws = p.prev_sibling_or_token();
                            } else {
                                break;
                            }
                        }
                        let is_after_open_tag = prev_non_ws
                            .as_ref()
                            .map_or(false, |e| e.kind() == SyntaxKind::OPEN_TAG.into());

                        let mut next_non_ws = element.next_sibling_or_token();
                        while let Some(n) = &next_non_ws {
                            if n.kind() == SyntaxKind::WHITESPACE.into() {
                                next_non_ws = n.next_sibling_or_token();
                            } else {
                                break;
                            }
                        }
                        let is_before_close_tag = next_non_ws
                            .as_ref()
                            .map_or(false, |e| e.kind() == SyntaxKind::CLOSE_TAG.into());

                        let mut collapsed = if force_inline {
                            let mut s = text
                                .replace('\r', "")
                                .replace('\n', " ")
                                .replace('\t', " ");
                            while s.contains("  ") {
                                s = s.replace("  ", " ");
                            }
                            if is_after_open_tag {
                                s = s.trim_start().to_string();
                            }
                            if is_before_close_tag {
                                s = s.trim_end().to_string();
                            }
                            s
                        } else {
                            text.replace("  ", " ")
                        };

                        if is_after_open_tag
                            && self.config.wrap_content
                            && !collapsed.trim().is_empty()
                            && !force_inline
                        {
                            self.output.push('\n');
                            self.output.push_str(&self.get_indent());
                            collapsed = collapsed.trim_start().to_string();
                        }

                        self.output.push_str(&collapsed);

                    // ── FALLTHROUGH ───────────────────────────────────────────
                    } else {
                        // Reset block-opener flag on any unrecognised token
                        self.just_saw_block_opener = false;
                        self.output.push_str(text);
                    }
                }
            }
        }

        self.current_indent = prev_indent;
    }
}

fn is_element_simple_and_short(node: &SyntaxNode, max_len: usize) -> bool {
    let mut has_complex = false;
    for c in node.children_with_tokens() {
        if c.kind() == SyntaxKind::ELEMENT.into() || c.kind() == SyntaxKind::COMMENT.into() {
            has_complex = true;
            break;
        }
    }
    if has_complex {
        return false;
    }

    let mut total_len = 0;
    for t in node.descendants_with_tokens() {
        if let NodeOrToken::Token(tok) = t {
            if tok.kind() != SyntaxKind::WHITESPACE.into() {
                total_len += tok.text().len();
            }
        }
    }

    total_len <= max_len
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_smart_whitespace_formatting() {
        let input = "<div id=\"app\">   <p>Hello <span>world</span></p></div>";

        let parser = Parser::new(input);
        let green_node = parser.parse();
        let syntax_node = SyntaxNode::new_root(green_node);

        let formatter = Formatter::new(FormatterConfig::default());
        let output = formatter.format(&syntax_node);

        let expected = "\n<div id=\"app\">\n  <p>Hello <span>world</span>\n  </p>\n</div>";

        assert_eq!(output, expected);
    }
}
