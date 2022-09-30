use std::{fmt, io, ops::Not};

use thiserror::Error;

use crate::{interpreter::RuntimeError, parser::ParserError, utils::colors};

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Parser(#[from] ParserErrorReporter),
    #[error("{0}")]
    Runtime(#[from] RuntimeError),
    #[error("{}[IO Error]{}: {0}\n", *colors::RED, *colors::RESET)]
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
        for error in &self.parser_errors {
            writeln!(
                f,
                "{}[Parser error]{}: {error}.",
                *colors::RED,
                *colors::RESET
            )?;
        }
        // for error in &self.runtime_errors {
        //     writeln!(
        //         f,
        //         "{}[Runtime error]{}: {error}.",
        //         *colors::RED,
        //         *colors::RESET
        //     )?;
        // }
        Ok(())
    }
}
