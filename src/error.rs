use std::{fmt, io, ops::Not};

use crate::{interpreter::RuntimeError, lexer::LexerError, parser::ParserError, utils::colors};

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{}[Lexer Error]{}: {0}", *colors::RED, *colors::RESET)]
    Lexer(#[from] LexerError),
    #[error("{}[Parser Error]{}: {0}", *colors::RED, *colors::RESET)]
    Parser(#[from] ParserErrorReporter),
    #[error("{}[Runtime Error]{}: {0}", *colors::RED, *colors::RESET)]
    Runtime(#[from] RuntimeError),
    #[error("{}[IO Error]{}: {0}", *colors::RED, *colors::RESET)]
    Io(#[from] io::Error),
}

#[derive(Debug, Default)]
pub struct ParserErrorReporter {
    parser_errors: Vec<ParserError>,
}

impl std::error::Error for ParserErrorReporter {}

impl ParserErrorReporter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_errors(&self) -> bool {
        self.parser_errors.is_empty().not()
    }

    pub fn add_parser_error(&mut self, error: ParserError) {
        self.parser_errors.push(error);
    }
}

impl fmt::Display for ParserErrorReporter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, error) in self.parser_errors.iter().enumerate() {
            write!(f, "{error}",)?;

            if i > 0 {
                write!(
                    f,
                    "\n{}[Parser error]{}: {error}.",
                    *colors::RED,
                    *colors::RESET
                )?;
            }
        }
        Ok(())
    }
}
