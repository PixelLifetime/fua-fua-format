use crate::config::FormatterConfig;
use crate::syntax::{SyntaxKind, SyntaxNode};
use rowan::NodeOrToken;

fn is_block_element(tag_name: &str) -> bool {
    let lower = tag_name.to_lowercase();
    matches!(
        lower.as_str(),
        "html" | "head" | "body" | "div" | "main" | "section" | "p" | "ul" | "li" | 
        "header" | "footer" | "form" | "nav" | "aside" | "table" | "tr" | "td" | "th" | 
        "tbody" | "thead" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "button" | 
        "label" | "input" | "textarea" | "select" | "img" | "script" | "style" | 
        "meta" | "link" | "title"
    ) || lower.contains('-')
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

pub struct Formatter {
    config: FormatterConfig,
    output: String,
    current_indent: usize,
}

impl Formatter {
    pub fn new(config: FormatterConfig) -> Self {
        Self {
            config,
            output: String::new(),
            current_indent: 0,
        }
    }

    pub fn get_indent(&self) -> String {
        if self.config.use_tabs {
            "\t".repeat(self.current_indent)
        } else {
            " ".repeat(self.current_indent * self.config.indent_size)
        }
    }

    pub fn format(mut self, root: &SyntaxNode) -> String {
        self.format_node(root);
        self.output
    }

    fn format_node(&mut self, node: &SyntaxNode) {
        let prev_indent = self.current_indent;
        let is_block_elem = node.kind() == SyntaxKind::ELEMENT.into() 
            && get_tag_name(node).as_deref().map_or(false, is_block_element);

        for element in node.children_with_tokens() {
            match element {
                NodeOrToken::Node(n) => {
                    if n.kind() == SyntaxKind::ELEMENT.into() {
                        let child_is_block = get_tag_name(&n).as_deref().map_or(false, is_block_element);
                        
                        if child_is_block {
                            self.output.push('\n');
                            self.output.push_str(&self.get_indent());
                        }
                        
                        self.format_node(&n);
                    } else if n.kind() == SyntaxKind::OPEN_TAG.into() {
                        self.format_node(&n);
                        if is_block_elem {
                            self.current_indent += 1;
                        }
                    } else if n.kind() == SyntaxKind::CLOSE_TAG.into() {
                        if is_block_elem {
                            self.current_indent -= 1;
                            self.output.push('\n');
                            self.output.push_str(&self.get_indent());
                        }
                        self.format_node(&n);
                    } else {
                        self.format_node(&n);
                    }
                }
                NodeOrToken::Token(ref t) => {
                    let text = t.text();
                    let kind = t.kind();
                    
                    if kind == SyntaxKind::WHITESPACE.into() {
                        if node.kind() == SyntaxKind::OPEN_TAG.into() || 
                           node.kind() == SyntaxKind::SELF_CLOSING_TAG.into() {
                            
                            let next = element.next_sibling_or_token();
                            let next_is_bracket = next.as_ref().map_or(false, |e| {
                                e.kind() == SyntaxKind::CLOSE_ANGLE.into() || e.kind() == SyntaxKind::SLASH_CLOSE_ANGLE.into()
                            });

                            if next_is_bracket {
                                if self.config.bracket_same_line {
                                    if next.as_ref().unwrap().kind() == SyntaxKind::SLASH_CLOSE_ANGLE.into() {
                                        self.output.push(' ');
                                    }
                                    continue;
                                } else if self.config.wrap_attributes {
                                    self.output.push('\n');
                                    self.output.push_str(&self.get_indent());
                                    continue;
                                }
                            }

                            if text.contains('\n') || self.config.wrap_attributes {
                                self.output.push('\n');
                                self.output.push_str(&self.get_indent());
                                // One extra level of indent for wrapped attributes
                                if self.config.use_tabs {
                                    self.output.push('\t');
                                } else {
                                    self.output.push_str(&" ".repeat(self.config.indent_size));
                                }
                            } else {
                                self.output.push(' ');
                            }
                        } else if node.kind() == SyntaxKind::CLOSE_TAG.into() {
                            // Drop whitespace completely within close tags `</div >`
                        } else {
                            // Maintain semantic spacing between elements
                            let prev = element.prev_sibling_or_token();
                            let next = element.next_sibling_or_token();
                            
                            // Check if next element is a block. We drop spaces before blocks since their traversal pushes `\n`
                            let next_is_block = next.as_ref().map_or(false, |e| {
                                e.kind() == SyntaxKind::ELEMENT.into() 
                                && e.as_node().and_then(get_tag_name).as_deref().map_or(false, is_block_element)
                            });
                            
                            // If next is CLOSE_TAG of a block, it handles its own terminal newline.
                            let is_next_close_tag_of_block = next.as_ref().map_or(false, |e| {
                                e.kind() == SyntaxKind::CLOSE_TAG.into() && is_block_elem
                            });
                            
                            // Don't inject redundant spaces around } or @else bindings if they're auto-injecting
                            let next_text = next.as_ref().and_then(|e| {
                                if let NodeOrToken::Token(nt) = e { Some(nt.text()) } else { None }
                            });
                            
                            if next_text == Some("}") || next_text.unwrap_or("").starts_with("@else") {
                                continue;
                            }
                            
                            if next_is_block || is_next_close_tag_of_block {
                                continue;
                            } else {
                                // Follow user's inline wrapping intent: 
                                let prev = element.prev_sibling_or_token();
                                let is_after_open_tag = prev.as_ref().map_or(false, |e| e.kind() == SyntaxKind::OPEN_TAG.into());
                                
                                // If they had a manual newline, we snap it perfectly to the current indent level!
                                if text.contains('\n') || (is_after_open_tag && self.config.wrap_content) {
                                    self.output.push('\n');
                                    self.output.push_str(&self.get_indent());
                                } else {
                                    // Collapse contiguous whitespace to a single space inline
                                    self.output.push(' ');
                                }
                            }
                        }
                        continue;
                    } else if kind == SyntaxKind::STRING_DOUBLE.into() {
                        if self.config.single_quotes && text.len() >= 2 {
                            let inner = &text[1..text.len()-1];
                            let escaped = inner.replace("'", "&apos;");
                            self.output.push('\'');
                            self.output.push_str(&escaped);
                            self.output.push('\'');
                        } else {
                            self.output.push_str(text);
                        }
                    } else if kind == SyntaxKind::STRING_SINGLE.into() {
                        if !self.config.single_quotes && text.len() >= 2 {
                            let inner = &text[1..text.len()-1];
                            let escaped = inner.replace("\"", "&quot;");
                            self.output.push('"');
                            self.output.push_str(&escaped);
                            self.output.push('"');
                        } else {
                            self.output.push_str(text);
                        }
                    } else if kind == SyntaxKind::CLOSE_ANGLE.into() || kind == SyntaxKind::SLASH_CLOSE_ANGLE.into() {
                        let prev = element.prev_sibling_or_token();
                        let prev_is_whitespace = prev.as_ref().map_or(false, |e| e.kind() == SyntaxKind::WHITESPACE.into());
                        
                        let in_opening = node.kind() == SyntaxKind::OPEN_TAG.into() || node.kind() == SyntaxKind::SELF_CLOSING_TAG.into();
                        if in_opening && !self.config.bracket_same_line && self.config.wrap_attributes {
                            if !prev_is_whitespace {
                                self.output.push('\n');
                                self.output.push_str(&self.get_indent());
                            }
                        }
                        self.output.push_str(text);
                    } else if kind == SyntaxKind::COMMENT.into() {
                        self.output.push('\n');
                        self.output.push_str(&self.get_indent());
                        self.output.push_str(text);
                    } else if kind == SyntaxKind::IDENT.into() {
                        let is_elem_or_root = node.kind() == SyntaxKind::ELEMENT.into() || node.kind() == SyntaxKind::ROOT.into();
                        
                        let prev = element.prev_sibling_or_token();
                        let is_after_open_tag = prev.as_ref().map_or(false, |e| e.kind() == SyntaxKind::OPEN_TAG.into());
                        
                        if is_after_open_tag && self.config.wrap_content {
                            self.output.push('\n');
                            self.output.push_str(&self.get_indent());
                        }
                        
                        if is_elem_or_root && text.starts_with('@') {
                            if text.starts_with("@else") {
                                if !self.output.ends_with(' ') {
                                    self.output.push(' ');
                                }
                            } else {
                                self.output.push('\n');
                                self.output.push_str(&self.get_indent());
                            }
                        }
                        self.output.push_str(text);
                    } else if kind == SyntaxKind::TEXT.into() {
                        let is_elem_or_root = node.kind() == SyntaxKind::ELEMENT.into() || node.kind() == SyntaxKind::ROOT.into();
                        
                        if is_elem_or_root && text == "}" {
                            self.current_indent = self.current_indent.saturating_sub(1);
                            self.output.push('\n');
                            self.output.push_str(&self.get_indent());
                            self.output.push_str("}");
                        } else if is_elem_or_root && text == "{" {
                            if !self.output.ends_with(' ') {
                                self.output.push(' ');
                            }
                            self.output.push('{');
                            self.current_indent += 1;
                        } else {
                            let prev = element.prev_sibling_or_token();
                            let is_after_open_tag = prev.as_ref().map_or(false, |e| e.kind() == SyntaxKind::OPEN_TAG.into());
                            
                            let mut collapsed = text.replace("  ", " ");
                            
                            if is_after_open_tag && self.config.wrap_content && !collapsed.trim().is_empty() {
                                self.output.push('\n');
                                self.output.push_str(&self.get_indent());
                                collapsed = collapsed.trim_start().to_string();
                            }
                            
                            self.output.push_str(&collapsed);
                        }
                    } else {
                        self.output.push_str(text);
                    }
                }
            }
        }
        
        // Restore indent natively guarding against missing CLOSE_TAG mismatches leaking state offsets.
        self.current_indent = prev_indent;
    }
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
