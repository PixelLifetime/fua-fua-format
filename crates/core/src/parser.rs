use logos::{Logos, SpannedIter};
use rowan::{GreenNode, GreenNodeBuilder};
use std::iter::Peekable;

use crate::lexer::Token;
use crate::syntax::SyntaxKind;

fn token_to_kind(token: &Token) -> SyntaxKind {
    match token {
        Token::Whitespace => SyntaxKind::WHITESPACE,
        Token::Comment => SyntaxKind::COMMENT,
        Token::OpenAngle => SyntaxKind::OPEN_ANGLE,
        Token::CloseAngle => SyntaxKind::CLOSE_ANGLE,
        Token::OpenAngleSlash => SyntaxKind::OPEN_ANGLE_SLASH,
        Token::SlashCloseAngle => SyntaxKind::SLASH_CLOSE_ANGLE,
        Token::Equals => SyntaxKind::EQUALS,
        Token::Ident => SyntaxKind::IDENT,
        Token::StringDouble => SyntaxKind::STRING_DOUBLE,
        Token::StringSingle => SyntaxKind::STRING_SINGLE,
        Token::Text => SyntaxKind::TEXT,
        Token::UnclosedQuote => SyntaxKind::UNCLOSED_QUOTE,
    }
}

pub struct Parser<'a> {
    source: &'a str,
    lexer: Peekable<SpannedIter<'a, Token>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Token::lexer(source).spanned().peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    /// Consumes the next token and adds it to the builder
    fn bump(&mut self) {
        if let Some((res, span)) = self.lexer.next() {
            let token = res.unwrap_or(Token::Text);
            let kind = token_to_kind(&token);
            let text = &self.source[span];
            self.builder.token(kind.into(), text);
        }
    }

    pub fn parse(mut self) -> GreenNode {
        self.builder.start_node(SyntaxKind::ROOT.into());

        while self.lexer.peek().is_some() {
            self.parse_node();
        }

        self.builder.finish_node();
        self.builder.finish()
    }

    fn peek_token(&mut self) -> Option<Token> {
        self.lexer
            .peek()
            .map(|(res, _)| res.as_ref().unwrap_or(&Token::Text).clone())
    }

    fn parse_node(&mut self) {
        match self.peek_token() {
            Some(Token::OpenAngle) => self.parse_element(),
            Some(Token::OpenAngleSlash) => {
                self.parse_tag(SyntaxKind::CLOSE_TAG);
            }
            Some(_) => self.bump(),
            None => {}
        }
    }

    fn parse_element(&mut self) {
        self.builder.start_node(SyntaxKind::ELEMENT.into());

        // Peek tag name to identify HTML void elements
        let mut is_void = false;
        let mut peek_iter = self.lexer.clone();
        peek_iter.next(); // skip `<`
        if let Some((Ok(Token::Ident), span)) = peek_iter.next() {
            let tag_name = &self.source[span];
            let lower = tag_name.to_lowercase();
            is_void = matches!(
                lower.as_str(),
                "area"
                    | "base"
                    | "br"
                    | "col"
                    | "embed"
                    | "hr"
                    | "img"
                    | "input"
                    | "link"
                    | "meta"
                    | "source"
                    | "track"
                    | "wbr"
            );
        }

        // Parse open tag
        let is_self_closing = self.parse_tag(SyntaxKind::OPEN_TAG);

        if !is_self_closing && !is_void {
            // Parse children recursively
            while let Some(token) = self.peek_token() {
                if token == Token::OpenAngleSlash {
                    break;
                }
                self.parse_node();
            }

            // Parse close tag if it exists
            if matches!(self.peek_token(), Some(Token::OpenAngleSlash)) {
                self.parse_tag(SyntaxKind::CLOSE_TAG);
            }
        }

        self.builder.finish_node();
    }

    fn parse_tag(&mut self, kind: SyntaxKind) -> bool {
        self.builder.start_node(kind.into());
        let mut is_self_closing = false;

        // bump `<` or `</`
        self.bump();

        // bump everything until `>` or `/>`
        while let Some(token) = self.peek_token() {
            let is_close = token == Token::CloseAngle || token == Token::SlashCloseAngle;

            if token == Token::SlashCloseAngle {
                is_self_closing = true;
            }

            self.bump();

            if is_close {
                break;
            }
        }

        self.builder.finish_node();
        is_self_closing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::SyntaxNode;

    #[test]
    fn test_lossless_parser() {
        let input = r#"
            <!-- Component wrapper -->
            <div id="app" *show="visible" @event="handle">
                hello world! 
                <button [(model)]="value" :disabled="false" />
                <Closing / > </ Closing >
            </div>
        "#;

        let parser = Parser::new(input);
        let green_node = parser.parse();
        let syntax_node = SyntaxNode::new_root(green_node);

        let reconstructed = syntax_node.to_string();

        assert_eq!(
            input, reconstructed,
            "The rebuilt source text must be byte-for-byte identical to the original input."
        );
    }

    #[test]
    fn test_nested_elements_and_lossless() {
        let input = "<div>\n  <p>Hello, <span>world</span>!</p>\n</div>";
        let parser = Parser::new(input);
        let green_node = parser.parse();
        let syntax_node = SyntaxNode::new_root(green_node);

        // Print tree so we can verify visually that ELEMENT nodes are nested correctly
        println!("{:#?}", syntax_node);

        // The Ultimate Check
        let reconstructed = syntax_node.to_string();
        assert_eq!(
            input, reconstructed,
            "The rebuilt source text must be byte-for-byte identical to the original input."
        );
    }

}
