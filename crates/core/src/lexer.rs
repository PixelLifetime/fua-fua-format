use logos::{Lexer, Logos};

fn lex_comment<'a>(lex: &mut Lexer<'a, Token>) -> Option<()> {
    let remainder = lex.remainder();
    if let Some(pos) = remainder.find("-->") {
        lex.bump(pos + 3);
        Some(())
    } else {
        // Permissive: if comment is not closed, consume to EOF
        lex.bump(remainder.len());
        Some(())
    }
}

// In Logos 0.14, the error token is replaced by returning Result<Token, ()> or custom error.
// We'll define a simple Error type or just use default.
#[derive(Logos, Debug, PartialEq, Clone)]
// #[logos(skip "")] // Do not skip anything to remain lossless
pub enum Token {
    #[regex(r"[ \t\n\r\f]+")]
    Whitespace,

    #[token("<!--", lex_comment)]
    Comment,

    #[token("<")]
    OpenAngle,

    #[token(">")]
    CloseAngle,

    #[token("</")]
    OpenAngleSlash,

    #[token("/>")]
    SlashCloseAngle,

    #[token("=")]
    Equals,

    /// Permissive identifiers: tags and attributes.
    /// Captures standard names as well as Angular (*ngIf, [(ngModel)]) and Vue (@click, :disabled).
    #[regex(r"[a-zA-Z0-9_\-\*\[\]\(\)\@\:\$]+")]
    Ident,

    #[regex(r#""[^"]*""#)]
    StringDouble,

    #[regex(r"'[^']*'")]
    StringSingle,

    /// Permissive text nodes: capture any sequence of characters not handled by other tokens.
    /// This includes punctuation, non-ascii characters, etc.
    #[regex(r#"[^a-zA-Z0-9_\-\*\[\]\(\)\@\:\$<>= \t\n\r\f"']+"#)]
    Text,

    /// Fallback for unbalanced quotes or other strict errors
    #[regex(r#"["']"#)]
    UnclosedQuote,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<(Token, &str)> {
        let mut lexer = Token::lexer(input);
        let mut tokens = Vec::new();
        while let Some(res) = lexer.next() {
            let token = res.expect("Should not return error with permissive lexer");
            tokens.push((token, lexer.slice()));
        }
        tokens
    }

    #[test]
    fn test_standard_html() {
        let input = r#"<div id="main">hello</div>"#;
        let tokens = lex(input);
        
        assert_eq!(
            tokens,
            vec![
                (Token::OpenAngle, "<"),
                (Token::Ident, "div"),
                (Token::Whitespace, " "),
                (Token::Ident, "id"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"main\""),
                (Token::CloseAngle, ">"),
                (Token::Ident, "hello"),
                (Token::OpenAngleSlash, "</"),
                (Token::Ident, "div"),
                (Token::CloseAngle, ">"),
            ]
        );
    }

    #[test]
    fn test_framework_syntax() {
        let input = r#"<button @click="doIt" *ngIf="show" [(ngModel)]="val" :disabled="true" />"#;
        let tokens = lex(input);

        assert_eq!(
            tokens,
            vec![
                (Token::OpenAngle, "<"),
                (Token::Ident, "button"),
                (Token::Whitespace, " "),
                (Token::Ident, "@click"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"doIt\""),
                (Token::Whitespace, " "),
                (Token::Ident, "*ngIf"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"show\""),
                (Token::Whitespace, " "),
                (Token::Ident, "[(ngModel)]"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"val\""),
                (Token::Whitespace, " "),
                (Token::Ident, ":disabled"),
                (Token::Equals, "="),
                (Token::StringDouble, "\"true\""),
                (Token::Whitespace, " "),
                (Token::SlashCloseAngle, "/>"),
            ]
        );
    }

    #[test]
    fn test_lossless_properties() {
        let input = "<!-- some comment -->\n <p>text</p>";
        let tokens = lex(input);

        assert_eq!(
            tokens,
            vec![
                (Token::Comment, "<!-- some comment -->"),
                (Token::Whitespace, "\n "),
                (Token::OpenAngle, "<"),
                (Token::Ident, "p"),
                (Token::CloseAngle, ">"),
                (Token::Ident, "text"),
                (Token::OpenAngleSlash, "</"),
                (Token::Ident, "p"),
                (Token::CloseAngle, ">"),
            ]
        );
        
        // Ensure total length matches input length
        let mut total_len = 0;
        for (_, slice) in &tokens {
            total_len += slice.len();
        }
        assert_eq!(total_len, input.len());
    }

    #[test]
    fn test_lossless_unclosed_comment() {
        let input = "<!-- unclosed";
        let tokens = lex(input);

        assert_eq!(tokens, vec![(Token::Comment, "<!-- unclosed")]);
    }
    
    #[test]
    fn test_raw_text() {
        let input = "hello, world! 你好!";
        let tokens = lex(input);

        assert_eq!(
            tokens,
            vec![
                (Token::Ident, "hello"),
                (Token::Text, ","),
                (Token::Whitespace, " "),
                (Token::Ident, "world"),
                (Token::Text, "!"),
                (Token::Whitespace, " "),
                (Token::Text, "你好!"),
            ]
        );
    }
}