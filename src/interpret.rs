#![allow(clippy::missing_errors_doc)]
#![allow(dead_code)]
use crate::{
    error::Error,
    parse::{Expr, Tree},
};

pub struct Interpretter {}

pub fn interpret(tree: Tree) -> Result<(), Error> {
    let mut interpretter = Interpretter {};
    interpretter.interpret(tree)
}

impl Interpretter {
    fn interpret(&mut self, tree: Tree) -> Result<(), Error> {
        for expr in tree.exprs {
            self.expr(expr)?;
        }
        Ok(())
    }

    fn expr(&mut self, e: Expr) -> Result<(), Error> {
        todo!()
    }
}
