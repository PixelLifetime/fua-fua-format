use num_derive::{FromPrimitive, ToPrimitive};
use rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FromPrimitive, ToPrimitive)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    // Lexer tokens (matching `lexer::Token` variants)
    WHITESPACE = 0,
    COMMENT,
    OPEN_ANGLE,
    CLOSE_ANGLE,
    OPEN_ANGLE_SLASH,
    SLASH_CLOSE_ANGLE,
    EQUALS,
    IDENT,
    STRING_DOUBLE,
    STRING_SINGLE,
    TEXT,
    UNCLOSED_QUOTE,

    // Structural concepts (Parser nodes)
    ROOT,
    ELEMENT,
    OPEN_TAG,
    CLOSE_TAG,
    SELF_CLOSING_TAG,
    ATTRIBUTE,

    // Catch-all for rowan
    ERROR,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HtmlLang;

impl Language for HtmlLang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        num_traits::FromPrimitive::from_u16(raw.0).unwrap_or(SyntaxKind::ERROR)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<HtmlLang>;
pub type SyntaxToken = rowan::SyntaxToken<HtmlLang>;
pub type SyntaxElement = rowan::SyntaxElement<HtmlLang>;
