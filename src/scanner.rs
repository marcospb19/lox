use std::str::Chars;

use phf::phf_map;

use crate::token::{Token, TokenType};

/// Compiler-time generated map of keywords.
static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and"    => TokenType::And,
    "class"  => TokenType::Class,
    "else"   => TokenType::Else,
    "false"  => TokenType::False,
    "for"    => TokenType::For,
    "fun"    => TokenType::Fun,
    "if"     => TokenType::If,
    "nil"    => TokenType::Nil,
    "or"     => TokenType::Or,
    "print"  => TokenType::Print,
    "return" => TokenType::Return,
    "super"  => TokenType::Super,
    "this"   => TokenType::This,
    "true"   => TokenType::True,
    "var"    => TokenType::Var,
    "while"  => TokenType::While,
};

/// Iterator that yields tokens from a piece of source code.
pub struct Scanner<'a> {
    /// The start of the token currently being parsed.
    token_start: &'a str,
    /// Source code char-by-char iterator.
    chars: Chars<'a>,
    /// Current line.
    line: usize,
    /// Has made any error (TODO maybe this should be removed).
    has_failed: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            token_start: source_code,
            chars: source_code.chars(),
            line: 1,
            has_failed: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn advance(&mut self) -> char {
        self.advance_checked().unwrap_or('\0')
    }

    fn advance_checked(&mut self) -> Option<char> {
        let ch = self.chars.next();
        if ch == Some('\n') {
            self.line += 1;
        }
        ch
    }

    fn advance_while(&mut self, f: impl Fn(char) -> bool) {
        while !self.is_at_end() && f(self.peek()) {
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

    // Check if matches pattern, and consumes the pattern.
    fn matches(&mut self, pattern: &str) -> bool {
        let matches = self.chars.as_str().starts_with(pattern);

        if matches {
            // Chars is skipping pattern.len() bytes
            self.chars = self.chars.as_str()[pattern.len()..].chars();
        }

        matches
    }

    fn reset_token(&mut self) {
        self.token_start = self.chars.as_str();
    }

    fn identifier(&mut self) -> TokenType {
        self.advance_while(char::is_alphanumeric);

        let text = self.token_slice();

        match KEYWORDS.get(text) {
            Some(keyword_token) => keyword_token.clone(),
            None => TokenType::Identifier,
        }
    }

    fn string(&mut self) -> TokenType {
        self.advance_while(|ch| ch != '"');

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
            panic!();
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes.
        let string = self.token_slice().trim_matches('"');
        TokenType::String(string.into())
    }

    fn token_slice(&mut self) -> &str {
        let span_length = self.token_start.len() - self.chars.as_str().len();
        &self.token_start[..span_length]
    }

    fn number(&mut self) -> TokenType {
        self.advance_while(|ch| ch.is_ascii_digit());

        // Look for a fractional part.
        if self.peek_checked() == Some('.') && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            self.advance_while(|ch| ch.is_ascii_digit());
        }

        let number: f64 = self.token_slice().parse().expect("Decimal parse error.");
        TokenType::Number(number)
    }

    fn error(&mut self, line: usize, message: impl ToString) {
        self.has_failed = true;
        fn report(line: usize, where_: String, message: String) {
            eprintln!("[line {line}] Error{where_}: {message}");
        }
        report(line, "".to_string(), message.to_string());
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use TokenType::*;

        // Set the start of this token
        self.reset_token();

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
                '"' => break self.string(),
                '0'..='9' => break self.number(),
                c if c.is_alphabetic() => break self.identifier(),
                c if c.is_whitespace() => self.reset_token(),
                _ => self.error(self.line, "Unexpected character."),
            }
        };

        let token_text = self.token_slice().to_owned();
        Some(Token::new(token_type, token_text, self.line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_scanner() {
        use TokenType::*;

        let source_code = r#"// this is ignored
            var average = !"text" + (min + max) / 2 or true and false;
        "#;

        let mut scanner = Scanner::new(source_code);

        assert_eq!(scanner.next().unwrap(), Token::new(Var        , "var", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Identifier , "average", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Equal      , "=", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Bang       , "!", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(String("text".into()), "\"text\"", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Plus       , "+", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(LeftParen  , "(", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Identifier , "min", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Plus       , "+", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Identifier , "max", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(RightParen , ")", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Slash      , "/", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Number(2.0), "2", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Or         , "or", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(True       , "true", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(And        , "and", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(False      , "false", 2));
        assert_eq!(scanner.next().unwrap(), Token::new(Semicolon  , ";", 2));
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
    fn test_dont_panic_on_terminating_comment() {
        let source_code = "// asjdnasjdnasjd";
        assert!(Scanner::new(source_code).next().is_none());
    }
}
