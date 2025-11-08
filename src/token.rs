use crate::{lex::Lexer, location::SrcOffset};
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenKind {
    Identifier,
    Plus,
    Minus,
    Star,
    Slash,
    Integer,
    Float,
    Equal,
    OParen,
    CParen,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Token {
    pub offset: SrcOffset,
    pub len: usize,
    pub kind: TokenKind,
}

impl Token {
    #[must_use]
    pub const fn new(offset: SrcOffset, len: usize, kind: TokenKind) -> Self {
        Self { offset, len, kind }
    }

    pub fn int(&self, lexer: &Lexer) -> i64 {
        let src = &lexer.src[self.offset..self.offset + self.len];
        src.parse().expect("Only call this on known good data")
    }

    pub fn id<'a, 'b>(&'a self, lexer: &'b Lexer) -> &'b str {
        &lexer.src[self.offset..self.offset + self.len]
    }
}
