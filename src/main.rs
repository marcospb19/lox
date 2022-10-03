#![feature(box_syntax)]

mod ast_printer;
mod error;
mod expression;
mod interpreter;
mod lexer;
mod parser;
mod statement;
mod token;
mod utils;

use std::{
    env,
    ffi::OsString,
    io::{self, Write},
    path::Path,
    process,
};

use fs_err as fs;

use crate::{
    error::{ParserErrorReporter, Result},
    expression::Expression,
    interpreter::{interpret_program, Environment},
    lexer::Scanner,
    parser::Parser,
};

fn main() {
    run().unwrap_or_else(|err| {
        eprint!("{err}");
        process::exit(1);
    });
}

fn run() -> Result<()> {
    let paths: Vec<OsString> = env::args_os().skip(1).collect();

    if paths.is_empty() {
        start_repl()?;
    } else {
        for arg in &paths {
            interpret_lox_file(arg.as_ref())?;
        }
    }

    Ok(())
}

fn start_repl() -> Result<()> {
    let mut state = Environment::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line)?;

        if bytes == 0 {
            return Ok(());
        }

        // If errors appear, report them and keep the REPL running.
        match interpret_lox_with_state(&line, &mut state) {
            Ok(_) => {}
            Err(err) => eprintln!("{err}"),
        }
    }
}

fn interpret_lox_file(path: &Path) -> Result<()> {
    let file_contents = fs::read_to_string(path)?;
    interpret_lox(&file_contents)?;

    Ok(())
}

fn interpret_lox(text: &str) -> Result<()> {
    let mut environment = Environment::new();
    interpret_lox_with_state(text, &mut environment)
}

fn interpret_lox_with_state(text: &str, state: &mut Environment) -> Result<()> {
    let tokens = Scanner::new(text).try_scan_all()?;
    let statements = Parser::new(&tokens).try_parse()?;

    interpret_program(statements, state).map_err(From::from)
}
