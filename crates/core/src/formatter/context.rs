use crate::syntax::{SyntaxElement, SyntaxKind, SyntaxNode};
use rowan::NodeOrToken;

pub(super) const ROOT_PARENT_KIND: &str = "NONE";

pub(super) fn node_tag_name(node: &SyntaxNode) -> Option<String> {
    match node.kind() {
        SyntaxKind::ELEMENT => element_tag_name(node),
        SyntaxKind::OPEN_TAG | SyntaxKind::CLOSE_TAG | SyntaxKind::SELF_CLOSING_TAG => {
            tag_node_name(node)
        }
        _ => None,
    }
}

pub(super) fn syntax_kind_label(kind: SyntaxKind) -> &'static str {
    match kind {
        SyntaxKind::WHITESPACE => "WHITESPACE",
        SyntaxKind::COMMENT => "COMMENT",
        SyntaxKind::OPEN_ANGLE => "OPEN_ANGLE",
        SyntaxKind::CLOSE_ANGLE => "CLOSE_ANGLE",
        SyntaxKind::OPEN_ANGLE_SLASH => "OPEN_ANGLE_SLASH",
        SyntaxKind::SLASH_CLOSE_ANGLE => "SLASH_CLOSE_ANGLE",
        SyntaxKind::EQUALS => "EQUALS",
        SyntaxKind::IDENT => "IDENT",
        SyntaxKind::STRING_DOUBLE => "STRING_DOUBLE",
        SyntaxKind::STRING_SINGLE => "STRING_SINGLE",
        SyntaxKind::TEXT => "TEXT",
        SyntaxKind::UNCLOSED_QUOTE => "UNCLOSED_QUOTE",
        SyntaxKind::ROOT => "ROOT",
        SyntaxKind::ELEMENT => "ELEMENT",
        SyntaxKind::OPEN_TAG => "OPEN_TAG",
        SyntaxKind::CLOSE_TAG => "CLOSE_TAG",
        SyntaxKind::SELF_CLOSING_TAG => "SELF_CLOSING_TAG",
        SyntaxKind::ATTRIBUTE => "ATTRIBUTE",
        SyntaxKind::ERROR => "ERROR",
    }
}

pub(super) fn structure_is_simple(node: &SyntaxNode) -> bool {
    !node.children_with_tokens().any(|element| {
        element.kind() == SyntaxKind::ELEMENT || element.kind() == SyntaxKind::COMMENT
    })
}

pub(super) fn is_raw_text_element(node: &SyntaxNode) -> bool {
    node_tag_name(node)
        .as_deref()
        .is_some_and(is_raw_text_tag_name)
}

pub(super) fn is_after_open_tag(element: &SyntaxElement) -> bool {
    previous_non_whitespace(element)
        .as_ref()
        .is_some_and(|sibling| sibling.kind() == SyntaxKind::OPEN_TAG)
}

pub(super) fn is_before_close_tag(element: &SyntaxElement) -> bool {
    next_non_whitespace(element)
        .as_ref()
        .is_some_and(|sibling| sibling.kind() == SyntaxKind::CLOSE_TAG)
}

pub(super) fn previous_non_whitespace(element: &SyntaxElement) -> Option<SyntaxElement> {
    let mut current = element.prev_sibling_or_token();
    while let Some(sibling) = current.clone() {
        if sibling.kind() == SyntaxKind::WHITESPACE {
            current = sibling.prev_sibling_or_token();
        } else {
            break;
        }
    }
    current
}

pub(super) fn next_non_whitespace(element: &SyntaxElement) -> Option<SyntaxElement> {
    let mut current = element.next_sibling_or_token();
    while let Some(sibling) = current.clone() {
        if sibling.kind() == SyntaxKind::WHITESPACE {
            current = sibling.next_sibling_or_token();
        } else {
            break;
        }
    }
    current
}

pub(super) fn token_text(element: &SyntaxElement) -> Option<&str> {
    match element {
        NodeOrToken::Node(_) => None,
        NodeOrToken::Token(token) => Some(token.text()),
    }
}

fn element_tag_name(node: &SyntaxNode) -> Option<String> {
    for element in node.children_with_tokens() {
        if let NodeOrToken::Node(tag) = element
            && matches!(
                tag.kind(),
                SyntaxKind::OPEN_TAG | SyntaxKind::SELF_CLOSING_TAG
            )
        {
            return tag_node_name(&tag);
        }
    }

    None
}

fn tag_node_name(node: &SyntaxNode) -> Option<String> {
    node.children_with_tokens()
        .find_map(|element| match element {
            NodeOrToken::Token(token) if token.kind() == SyntaxKind::IDENT => {
                Some(token.text().to_string())
            }
            _ => None,
        })
}

fn is_raw_text_tag_name(tag_name: &str) -> bool {
    matches!(tag_name, "script" | "style")
}
