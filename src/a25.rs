#![allow(clippy::missing_errors_doc)]
use crate::{error::Error, interpret::interpret, lex, parse};
use std::{fs, path::Path};

pub fn run_file<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let src_code = fs::read_to_string(path.as_ref())?;
    run(path, &src_code)
}

pub fn run<P: AsRef<Path>>(path: P, src: &str) -> Result<(), Error> {
    let lexer = lex::Lexer::new(path, src);
    let tree = parse::parse(lexer)?;
    let val = interpret(&tree)?;
    println!("{val:?}");
    Ok(())
}
