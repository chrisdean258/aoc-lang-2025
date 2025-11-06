#![allow(clippy::missing_errors_doc)]
#![allow(dead_code)]
use crate::{
    error::Error as A25Error,
    lex::Lexer,
    token::{Token, TokenKind},
};
use itertools::PutBack;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected EOF while parsing {0}")]
    UnexpectedEOF(&'static str),
    #[error("Unexpected token while parsing {1}: {0:?}")]
    UnexpectedToken(Token, &'static str),
}

#[derive(Debug)]
pub struct Tree {
    exprs: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Value(Token),
    BinOp(Box<Expr>, Token, Box<Expr>),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum Precedence {
    Lowest,
    Plus,
    Times,
    Highest,
}

impl Precedence {
    const fn next(self) -> Self {
        match self {
            Self::Lowest => Self::Plus,
            Self::Plus => Self::Times,
            Self::Times | Self::Highest => Self::Highest,
        }
    }

    fn less_or_eq(self, kind: &TokenKind) -> bool {
        match (self, kind) {
            (Self::Lowest | Self::Plus, _) => true,
            (Self::Times, TokenKind::Plus) => false,
            (Self::Times, TokenKind::Star) => true,
            (Self::Highest, TokenKind::Star) => true,
            _ => todo!("Operator precedence"),
        }
    }
}

pub fn parse<P: AsRef<Path>>(lexer: &mut Lexer<P>) -> Result<Tree, A25Error> {
    let tree = Tree {
        exprs: parse_exprs(lexer)?,
    };
    Ok(tree)
}

fn parse_exprs<P: AsRef<Path>>(_lexer: &mut Lexer<P>) -> Result<Vec<Expr>, A25Error> {
    let exprs = Vec::new();
    Ok(exprs)
}

fn parse_primary<P: AsRef<Path>>(lexer: &mut PutBack<Lexer<P>>) -> Result<Expr, A25Error> {
    let Some(token) = lexer.next() else {
        return Err(Error::UnexpectedEOF("primary expression").into());
    };
    let token = token?;
    Ok(match token.kind {
        TokenKind::Identifier => Expr::Value(token),
        TokenKind::Integer => Expr::Value(token),
        TokenKind::Float => Expr::Value(token),
        _ => return Err(Error::UnexpectedToken(token, "primary expression").into()),
    })
}

fn parse_expr<P: AsRef<Path>>(
    lexer: &mut PutBack<Lexer<P>>,
    lhs: Expr,
    precedence: Precedence,
) -> Result<Expr, A25Error> {
    if precedence == Precedence::Highest {
        return parse_primary(lexer);
    }
    let Some(mut lookahead) = lexer.next() else {
        return Ok(lhs);
    };
    let mut op = lookahead?;
    while precedence.less_or_eq(&op.kind) {
        let rhs = parse_primary(lexer)?;
        lookahead = match lexer.next() {
            Some(t) => t,
            _ => todo!(),
        }
        while lookahead?.kind
    }
    todo!()
}
