#![feature(box_syntax)]

mod expr;
mod printer;
mod scanner;
mod token;

use std::{
    env,
    ffi::OsString,
    io::{self, BufRead, BufReader},
    path::Path,
};

use fs_err as fs;

use crate::scanner::Scanner;

fn main() -> io::Result<()> {
    let paths: Vec<OsString> = env::args_os().skip(1).collect();

    if paths.is_empty() {
        run_prompt()?;
    } else {
        for arg in &paths {
            run_file(arg)?;
        }
    }

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let reader = io::stdin();
    let reader = BufReader::new(reader);

    for line in reader.lines() {
        let line = line?;
        print!("> ");
        run_scanner(&line);
    }

    Ok(())
}

pub fn run_file(path: impl AsRef<Path>) -> io::Result<()> {
    let file_contents = fs::read_to_string(path.as_ref())?;
    run_scanner(&file_contents);

    Ok(())
}

pub fn run_scanner(text: &str) {
    let scanner = Scanner::new(text);

    for token in scanner {
        println!("{:?}", token);
    }
}
