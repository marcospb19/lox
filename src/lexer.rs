use std::{ops::Not, str::Chars};

use phf::phf_map;

use crate::token::{Position, Token, TokenWithPosition};

/// Compiler-time generated map of keywords.
static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "false"  => Token::Bool(false),
    "true"   => Token::Bool(true),
    "and"    => Token::And,
    "class"  => Token::Class,
    "else"   => Token::Else,
    "for"    => Token::For,
    "fun"    => Token::Fun,
    "if"     => Token::If,
    "nil"    => Token::Nil,
    "or"     => Token::Or,
    "print"  => Token::Print,
    "return" => Token::Return,
    "super"  => Token::Super,
    "this"   => Token::This,
    "var"    => Token::Var,
    "while"  => Token::While,
};

/// Iterator that yields tokens from a piece of source code.
pub struct Scanner<'a> {
    /// The start of the token currently being parsed.
    token_start: &'a str,
    /// Source code char-by-char iterator.
    chars: Chars<'a>,
    /// Current token start position.
    position: Position,
    /// Has made any error (TODO: maybe this should be removed).
    has_failed: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            token_start: source_code,
            chars: source_code.chars(),
            position: Position::default(),
            has_failed: false,
        }
    }

    pub fn try_scan_all(self) -> Result<Vec<Token>, LexerError> {
        self.into_iter()
            .map(|x| Ok(x?.token_type))
            .collect::<Result<Vec<_>, _>>()
    }

    fn is_at_end(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Get lemexe for just parsed token.
    fn token_lexeme(&mut self) -> &str {
        let span_length = self.token_start.len() - self.chars.as_str().len();
        &self.token_start[..span_length]
    }

    /// Reset span start for the next token, to be used in `token_lexeme`.
    ///
    /// Returns the new position for the next token.
    fn reset_token(&mut self) -> Position {
        self.token_start = self.chars.as_str();
        self.position
    }

    fn advance_checked(&mut self) -> Option<char> {
        let ch = self.chars.next();

        if ch == Some('\n') {
            self.position.advance_line();
        } else {
            self.position.advance_column();
        }

        ch
    }

    fn advance(&mut self) -> char {
        self.advance_checked().unwrap_or('\0')
    }

    fn advance_while(&mut self, f: impl Fn(char) -> bool) {
        while self.is_at_end().not() && f(self.peek()) {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        self.peek_checked().unwrap_or('\0')
    }

    fn peek_checked(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_next(&mut self) -> char {
        self.chars.clone().nth(1).unwrap_or('\0')
    }

    /// Check if matches pattern, and if so, consumes the pattern.
    fn matches(&mut self, pattern: &str) -> bool {
        let matches = self.chars.as_str().starts_with(pattern);

        if matches {
            // advance scanner cursor
            for _ in pattern.chars() {
                self.advance();
            }
        }

        matches
    }

    /// Consume an identifier and check if it is a keyword.
    fn consume_identifier(&mut self) -> Token {
        self.advance_while(char::is_alphanumeric);

        let lexeme = self.token_lexeme();

        match KEYWORDS.get(lexeme) {
            Some(keyword_token) => keyword_token.clone(),
            None => Token::Identifier(lexeme.to_owned()),
        }
    }

    fn consume_string(&mut self) -> Result<Token, LexerError> {
        self.advance_while(|ch| ch != '"');

        if self.is_at_end() {
            return Err(LexerError::UnterminatedString);
        }

        // Skip the closing "
        self.advance();

        // Trim the surrounding quotes.
        let string = self.token_lexeme().trim_matches('"');

        Ok(Token::String(string.into()))
    }

    fn consume_number(&mut self) -> Result<Token, LexerError> {
        self.advance_while(|ch| ch.is_ascii_digit());

        // Look for a fractional part.
        if self.peek_checked() == Some('.') && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            self.advance_while(|ch| ch.is_ascii_digit());
        }

        // Parse number
        match self.token_lexeme().parse() {
            Ok(number) => Ok(Token::Number(number)),
            Err(_err) => Err(LexerError::InvalidNumber),
        }
    }

    fn error(&mut self, message: impl ToString) {
        fn report(position: Position, where_: String, message: String) {
            eprintln!("[line {position}] Error{where_}: {message}");
        }
        self.has_failed = true;
        report(self.position, "".to_string(), message.to_string());
    }
}

impl Iterator for Scanner<'_> {
    type Item = Result<TokenWithPosition, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        // Set the start of this token
        let mut token_position = self.reset_token();

        let token_type = loop {
            if self.is_at_end() {
                return None;
            }

            match () {
                _ if self.matches("!=") => break BangEqual,
                _ if self.matches("==") => break EqualEqual,
                _ if self.matches("<=") => break LessEqual,
                _ if self.matches(">=") => break GreaterEqual,
                _ if self.matches("//") => self.advance_while(|ch| ch != '\n'),
                _ => {}
            }

            match self.advance_checked()? {
                '(' => break LeftParen,
                ')' => break RightParen,
                '{' => break LeftBrace,
                '}' => break RightBrace,
                ',' => break Comma,
                '.' => break Dot,
                '-' => break Minus,
                '+' => break Plus,
                ';' => break Semicolon,
                '*' => break Star,
                '!' => break Bang,
                '=' => break Equal,
                '<' => break Less,
                '>' => break Greater,
                '/' => break Slash,
                '"' => {
                    match self.consume_string() {
                        Ok(string) => break string,
                        Err(err) => return Some(Err(err)),
                    }
                }
                '0'..='9' => {
                    match self.consume_number() {
                        Ok(number) => break number,
                        Err(err) => return Some(Err(err)),
                    }
                }
                c if c.is_alphabetic() => break self.consume_identifier(),
                c if c.is_whitespace() => token_position = self.reset_token(),
                _ => self.error("Unexpected character."),
            }
        };

        Some(Ok(TokenWithPosition::new(token_type, token_position)))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("Unterminated string.")]
    UnterminatedString,
    #[error("Invalid number.")]
    InvalidNumber,
}

#[cfg(test)]
mod tests {
    use Token::*;

    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_scanning_statement() {
        let source_code = r#"var foo = !"text" + (min + max) / 2 or true and false;"#;

        let mut scanner = Scanner::new(source_code);
        let mut n = || scanner.next().unwrap().unwrap();

        assert_eq!(n(), TokenWithPosition::new(Var                     , Position::new(1, 1)));
        assert_eq!(n(), TokenWithPosition::new(Identifier("foo".into()), Position::new(1, 5)));
        assert_eq!(n(), TokenWithPosition::new(Equal                   , Position::new(1, 9)));
        assert_eq!(n(), TokenWithPosition::new(Bang                    , Position::new(1, 11)));
        assert_eq!(n(), TokenWithPosition::new(String("text".into())   , Position::new(1, 12)));
        assert_eq!(n(), TokenWithPosition::new(Plus                    , Position::new(1, 19)));
        assert_eq!(n(), TokenWithPosition::new(LeftParen               , Position::new(1, 21)));
        assert_eq!(n(), TokenWithPosition::new(Identifier("min".into()), Position::new(1, 22)));
        assert_eq!(n(), TokenWithPosition::new(Plus                    , Position::new(1, 26)));
        assert_eq!(n(), TokenWithPosition::new(Identifier("max".into()), Position::new(1, 28)));
        assert_eq!(n(), TokenWithPosition::new(RightParen              , Position::new(1, 31)));
        assert_eq!(n(), TokenWithPosition::new(Slash                   , Position::new(1, 33)));
        assert_eq!(n(), TokenWithPosition::new(Number(2.0)             , Position::new(1, 35)));
        assert_eq!(n(), TokenWithPosition::new(Or                      , Position::new(1, 37)));
        assert_eq!(n(), TokenWithPosition::new(Bool(true)              , Position::new(1, 40)));
        assert_eq!(n(), TokenWithPosition::new(And                     , Position::new(1, 45)));
        assert_eq!(n(), TokenWithPosition::new(Bool(false)             , Position::new(1, 49)));
        assert_eq!(n(), TokenWithPosition::new(Semicolon               , Position::new(1, 54)));
        assert!(scanner.next().is_none());
    }

    #[rustfmt::skip]
        #[test]
        fn test_scanning_expression() {
            let source_code = "1 - (2 * 3) < 4 == false";
            let mut scanner = Scanner::new(source_code);
            let mut n = || scanner.next().unwrap().unwrap();

            assert_eq!(n(), TokenWithPosition::new(Number(1.0), Position::new(1, 1)));
            assert_eq!(n(), TokenWithPosition::new(Minus      , Position::new(1, 3)));
            assert_eq!(n(), TokenWithPosition::new(LeftParen  , Position::new(1, 5)));
            assert_eq!(n(), TokenWithPosition::new(Number(2.0), Position::new(1, 6)));
            assert_eq!(n(), TokenWithPosition::new(Star       , Position::new(1, 8)));
            assert_eq!(n(), TokenWithPosition::new(Number(3.0), Position::new(1, 10)));
            assert_eq!(n(), TokenWithPosition::new(RightParen , Position::new(1, 11)));
            assert_eq!(n(), TokenWithPosition::new(Less       , Position::new(1, 13)));
            assert_eq!(n(), TokenWithPosition::new(Number(4.0), Position::new(1, 15)));
            assert_eq!(n(), TokenWithPosition::new(EqualEqual , Position::new(1, 17)));
            assert_eq!(n(), TokenWithPosition::new(Bool(false), Position::new(1, 20)));
            assert!(scanner.next().is_none());
        }

    #[test]
    fn test_multiple_comments() {
        let source_code = "\
                // fn main() {\n\
                // println!(\"hello world\");\n\
                // }\n\
            ";
        assert!(Scanner::new(source_code).next().is_none());
    }

    #[test]
    fn test_dont_panic_on_comment_at_end() {
        let source_code = "// asjdnasjdnasjd";
        assert!(Scanner::new(source_code).next().is_none());
    }

    #[test]
    fn test_unterminated_string_error() {
        let source_code = r#"
                print 1 + "qwerty;
            "#;
        assert!(Scanner::new(source_code).try_scan_all().is_err());
    }
}
