use crate::location::SrcOffset;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    Identifier,
    Plus,
    Minus,
    Star,
    Slash,
    Integer,
    Float,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    pub offset: SrcOffset,
    pub kind: TokenKind,
}

impl Token {
    #[must_use]
    pub const fn new(offset: SrcOffset, kind: TokenKind) -> Self {
        Self { offset, kind }
    }
}
