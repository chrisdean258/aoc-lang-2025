#![allow(clippy::missing_errors_doc)]
#![allow(dead_code)]
use crate::{
    error::Error as A25Error,
    lex::Lexer,
    token::{Token, TokenKind},
};
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
    pub exprs: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Value(i64),
    Variable(String),
    BinOp(Box<Expr>, Token, Box<Expr>),
}

type Precedence = i64;
const fn predence_of(k: TokenKind) -> Precedence {
    match k {
        TokenKind::Equal => 10,
        TokenKind::Plus | TokenKind::Minus => 20,
        TokenKind::Star | TokenKind::Slash => 30,
        _ => -1,
    }
}

const fn is_left_associative(k: TokenKind) -> bool {
    matches!(k, TokenKind::Equal)
}

pub fn parse(mut lexer: Lexer) -> Result<Tree, A25Error> {
    let tree = Tree {
        exprs: parse_exprs(&mut lexer)?,
    };
    Ok(tree)
}

fn parse_exprs(lexer: &mut Lexer) -> Result<Vec<Expr>, A25Error> {
    let mut exprs = Vec::new();
    while lexer.peek().is_some() {
        exprs.push(parse_expr(lexer)?);
    }
    Ok(exprs)
}

fn parse_expr(lexer: &mut Lexer) -> Result<Expr, A25Error> {
    let lhs = parse_primary(lexer)?;
    parse_expr_1(lexer, lhs, 0)
}

macro_rules! must {
    ($lexer:ident, $context:expr) => {{
        let Some(token) = $lexer.next() else {
            return Err(Error::UnexpectedEOF($context).into());
        };
        token
    }};
}

fn parse_primary(lexer: &mut Lexer) -> Result<Expr, A25Error> {
    let token = must!(lexer, "primary expression")?;
    Ok(match token.kind {
        TokenKind::Identifier => Expr::Variable(token.id(lexer).into()),
        TokenKind::Float => {
            todo!()
        }
        TokenKind::Integer => Expr::Value(token.int(lexer)),
        TokenKind::OParen => {
            let expr = parse_expr(lexer)?;
            let token = must!(lexer, "parenthesizes expression")?;
            if !matches!(token.kind, TokenKind::CParen) {
                return Err(
                    Error::UnexpectedToken(token, "end of parenthesised expression").into(),
                );
            }
            expr
        }
        _ => return Err(Error::UnexpectedToken(token, "primary expression").into()),
    })
}

fn parse_expr_1(
    lexer: &mut Lexer,
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
        while lap > op_pres || (is_left_associative(op.kind) && lap >= op_pres) {
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
            [Expr::Value(1)]
        ));
    }

    #[test]
    fn plus() {
        let lexer = Lexer::new("test", "1 + 1");
        let tree = parse(lexer);
        assert_eq!(format!("{tree:?}"), "Ok(Tree { exprs: [BinOp(Value(1), Token { offset: 2, len: 1, kind: Plus }, Value(1))] })");
    }
    #[test]
    fn plus_times() {
        let lexer = Lexer::new("test", "1 + 1 * 1");
        let tree = parse(lexer);
        assert_eq!(format!("{tree:?}"), "Ok(Tree { exprs: [BinOp(Value(1), Token { offset: 2, len: 1, kind: Plus }, BinOp(Value(1), Token { offset: 6, len: 1, kind: Star }, Value(1)))] })");
    }

    #[test]
    fn parens() {
        let lexer = Lexer::new("test", "1 * (1 + 1)");
        let tree = parse(lexer);
        assert_eq!(format!("{tree:?}"), "Ok(Tree { exprs: [BinOp(Value(1), Token { offset: 2, len: 1, kind: Star }, BinOp(Value(1), Token { offset: 7, len: 1, kind: Plus }, Value(1)))] })");
    }

    #[test]
    fn left_associative_eq() {
        let lexer = Lexer::new("test", "a = b = c");
        let tree = parse(lexer);
        assert_eq!(format!("{tree:?}"), "Ok(Tree { exprs: [BinOp(Variable(\"a\"), Token { offset: 2, len: 1, kind: Equal }, BinOp(Variable(\"b\"), Token { offset: 6, len: 1, kind: Equal }, Variable(\"c\")))] })");
    }
}
