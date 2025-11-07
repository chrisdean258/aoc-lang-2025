#![allow(clippy::missing_errors_doc)]
#![allow(dead_code)]
use crate::{
    error::Error as A25Error,
    lex::Lexer,
    token::{Token, TokenKind},
};
use std::iter::Peekable;
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

type Precedence = i64;
const fn predence_of(k: TokenKind) -> Precedence {
    match k {
        TokenKind::Plus | TokenKind::Minus => 10,
        TokenKind::Star | TokenKind::Slash => 20,
        _ => -1,
    }
}

pub fn parse(lexer: Lexer) -> Result<Tree, A25Error> {
    let mut lexer = lexer.peekable();
    let tree = Tree {
        exprs: parse_exprs(&mut lexer)?,
    };
    Ok(tree)
}

fn parse_exprs(lexer: &mut Peekable<Lexer>) -> Result<Vec<Expr>, A25Error> {
    let mut exprs = Vec::new();
    while lexer.peek().is_some() {
        exprs.push(parse_expr(lexer)?);
    }
    Ok(exprs)
}

fn parse_expr(lexer: &mut Peekable<Lexer>) -> Result<Expr, A25Error> {
    let lhs = parse_primary(lexer)?;
    parse_expr_1(lexer, lhs, 0)
}

fn parse_primary(lexer: &mut Peekable<Lexer>) -> Result<Expr, A25Error> {
    let Some(token) = lexer.next() else {
        return Err(Error::UnexpectedEOF("primary expression").into());
    };
    let token = token?;
    Ok(match token.kind {
        TokenKind::Identifier | TokenKind::Integer | TokenKind::Float => Expr::Value(token),
        _ => return Err(Error::UnexpectedToken(token, "primary expression").into()),
    })
}

fn parse_expr_1(
    lexer: &mut Peekable<Lexer>,
    mut lhs: Expr,
    min_precedence: Precedence,
) -> Result<Expr, A25Error> {
    let Some(lookahead_) = lexer.peek() else {
        return Ok(lhs);
    };
    let mut lookahead = lookahead_.clone()?;
    let mut op = lookahead;
    let mut op_pres = predence_of(op.kind);
    while op_pres >= min_precedence {
        let _ = lexer.next();
        let mut rhs = parse_primary(lexer)?;
        let Some(lookahead_) = lexer.peek() else {
            return Ok(Expr::BinOp(Box::new(lhs), op, Box::new(rhs)));
        };
        lookahead = lookahead_.clone()?;
        let mut lap = predence_of(lookahead.kind);
        while lap > op_pres {
            rhs = parse_expr_1(lexer, rhs, op_pres + i64::from(lap > op_pres))?;
            let Some(lookahead_) = lexer.peek() else {
                return Ok(Expr::BinOp(Box::new(lhs), op, Box::new(rhs)));
            };
            lookahead = lookahead_.clone()?;
            lap = predence_of(lookahead.kind);
        }
        lhs = Expr::BinOp(Box::new(lhs), op, Box::new(rhs));
        op = lookahead;
        op_pres = predence_of(op.kind);
    }
    Ok(lhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num() {
        let lexer = Lexer::new("test", "1");
        let tree = parse(lexer);
        assert!(matches!(tree, Ok(Tree { .. },)));
        assert!(matches!(
            &tree.expect("already checked").exprs[..],
            [Expr::Value(Token {
                offset: 0,
                kind: TokenKind::Integer,
            })]
        ));
    }

    #[test]
    fn plus() {
        let lexer = Lexer::new("test", "1 + 1");
        let tree = parse(lexer);
        assert_eq!(format!("{tree:?}"), "Ok(Tree { exprs: [BinOp(Value(Token { offset: 0, kind: Integer }), Token { offset: 2, kind: Plus }, Value(Token { offset: 4, kind: Integer }))] })");
    }
    #[test]
    fn plus_times() {
        let lexer = Lexer::new("test", "1 + 1 * 1");
        let tree = parse(lexer);
        assert_eq!(format!("{tree:?}"), "Ok(Tree { exprs: [BinOp(Value(Token { offset: 0, kind: Integer }), Token { offset: 2, kind: Plus }, BinOp(Value(Token { offset: 4, kind: Integer }), Token { offset: 6, kind: Star }, Value(Token { offset: 8, kind: Integer })))] })");
    }
}
