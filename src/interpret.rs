#![allow(clippy::missing_errors_doc)]
#![allow(dead_code)]

use crate::{
    error::Error as A25Error,
    parse::{Expr, Tree},
    token::{Token, TokenKind},
    value::Value,
};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No such variable: {0}")]
    NoSuchVariable(String),
    #[error("No such variable: {0}")]
    NotAnLVal(String),
}

pub struct Interpretter {
    scopes: Vec<HashMap<String, Value>>,
}

pub fn interpret(tree: &Tree) -> Result<Value, A25Error> {
    let mut interpretter = Interpretter {
        scopes: vec![HashMap::new()],
    };
    Ok(interpretter.interpret(tree)?)
}

impl Interpretter {
    fn interpret(&mut self, tree: &Tree) -> Result<Value, Error> {
        let mut rv = Value::None(0);
        for expr in &tree.exprs {
            rv = self.expr(expr)?;
        }
        Ok(rv)
    }

    fn expr(&mut self, e: &Expr) -> Result<Value, Error> {
        match e {
            Expr::Value(i) => Ok(Value::Int(*i)),
            Expr::Variable(s) => Ok(*self
                .scopes
                .last()
                .expect("Always should have a scope")
                .get(s)
                .ok_or_else(|| Error::NoSuchVariable(s.into()))?),
            Expr::BinOp(left, op, right) => self.binop(left, op, right),
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn binop(&mut self, left: &Expr, op: &Token, right: &Expr) -> Result<Value, Error> {
        if matches!(op.kind, TokenKind::Equal) {
            return self.assign(left, right);
        }
        let left = self.expr(left)?;
        let right = self.expr(right)?;
        Ok(match (left, op.kind, right) {
            (Value::Int(a), TokenKind::Plus, Value::Int(b)) => Value::Int(a + b),
            (Value::Int(a), TokenKind::Minus, Value::Int(b)) => Value::Int(a - b),
            (Value::Int(a), TokenKind::Star, Value::Int(b)) => Value::Int(a * b),
            (Value::Int(a), TokenKind::Slash, Value::Int(b)) => Value::Float(a as f64 / b as f64),

            (Value::Float(a), TokenKind::Plus, Value::Int(b)) => Value::Float(a + b as f64),
            (Value::Float(a), TokenKind::Minus, Value::Int(b)) => Value::Float(a - b as f64),
            (Value::Float(a), TokenKind::Star, Value::Int(b)) => Value::Float(a * b as f64),
            (Value::Float(a), TokenKind::Slash, Value::Int(b)) => Value::Float(a / b as f64),

            (Value::Int(a), TokenKind::Plus, Value::Float(b)) => Value::Float(a as f64 + b),
            (Value::Int(a), TokenKind::Minus, Value::Float(b)) => Value::Float(a as f64 - b),
            (Value::Int(a), TokenKind::Star, Value::Float(b)) => Value::Float(a as f64 * b),
            (Value::Int(a), TokenKind::Slash, Value::Float(b)) => Value::Float(a as f64 / b),

            (Value::Float(a), TokenKind::Plus, Value::Float(b)) => Value::Float(a + b),
            (Value::Float(a), TokenKind::Minus, Value::Float(b)) => Value::Float(a - b),
            (Value::Float(a), TokenKind::Star, Value::Float(b)) => Value::Float(a * b),
            (Value::Float(a), TokenKind::Slash, Value::Float(b)) => Value::Float(a / b),

            (_l, e, _r) => unreachable!("Parsing parsed {e:?} as binary operator"),
        })
    }

    fn assign(&mut self, left: &Expr, right: &Expr) -> Result<Value, Error> {
        let Expr::Variable(s) = left else {
            return Err(Error::NotAnLVal(format!("{left:?}")));
        };
        let val = self.expr(right)?;
        self.scopes
            .last_mut()
            .expect("Always have a scope")
            .insert(s.clone(), val);
        Ok(val)
    }
}
