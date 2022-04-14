use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One Or Two Character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    Nil,
    True,
    False,

    // Keywords
    And,
    Class,
    Else,
    Fun,
    For,
    If,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
}

impl Token {
    pub fn is_literal(&self) -> bool {
        use Token::*;

        matches!(
            self,
            False | True | Nil | Identifier(_) | Number(_) | String(_)
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Token::*;

        let txt = match self {
            Identifier(inner) | String(inner) => return inner.fmt(f),
            Number(inner) => return inner.fmt(f),
            LeftParen => "(",
            RightParen => ")",
            LeftBrace => "{",
            RightBrace => "}",
            Comma => ",",
            Dot => ".",
            Minus => "-",
            Plus => "+",
            Semicolon => ";",
            Slash => "/",
            Star => "*",
            Bang => "!",
            BangEqual => "!=",
            Equal => "=",
            EqualEqual => "==",
            Greater => ">",
            GreaterEqual => ">=",
            Less => "<",
            LessEqual => "<=",
            Nil => "nil",
            True => "true",
            False => "false",
            And => "and",
            Class => "class",
            Else => "else",
            Fun => "fun",
            For => "for",
            If => "if",
            Or => "or",
            Print => "print",
            Return => "return",
            Super => "super",
            This => "this",
            Var => "var",
            While => "while",
        };

        txt.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenWithPosition {
    pub token_type: Token,
    pub position: Position,
}

impl TokenWithPosition {
    pub fn new(token_type: Token, position: Position) -> Self {
        Self {
            token_type,
            position,
        }
    }
}

impl fmt::Display for TokenWithPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.token_type.fmt(f)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    #[cfg(test)]
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    pub fn advance_column(&mut self) {
        self.column += 1;
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { line: 1, column: 1 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
