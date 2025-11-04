#![allow(dead_code)]
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    src: &'a str,
    offset: usize,
    newlines: Vec<usize>,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub const fn new(src: &'a str) -> Self {
        Self {
            src,
            offset: 0,
            newlines: Vec::new(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, ()>;
    fn next(&mut self) -> Option<Self::Item> {
        let unlexed = &self.src[self.offset..];
        //Skip Leading whitespace
        let ws_len = unlexed.find(|c: char| !c.is_whitespace())?;
        self.offset += ws_len;
        let unlexed = &self.src[self.offset..];
        let mut chars = unlexed.chars();
        let c = chars.next()?;
        match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                let len = unlexed
                    .find(|c: char| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(unlexed.len());
                let rv = Some(Ok(Token::new(self.offset, TokenKind::Identifier)));
                self.offset += len;
                rv
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let lexer = Lexer::new("test");
        let tokens = lexer.collect::<Result<Vec<_>, _>>().expect("Lexing Error");
        assert_eq!(vec![Token::new(0, TokenKind::Identifier)], tokens);
    }

    #[test]
    fn test_lex2() {
        let lexer = Lexer::new("hello world");
        let tokens = lexer.collect::<Result<Vec<_>, _>>().expect("Lexing Error");
        assert_eq!(
            vec![
                Token::new(0, TokenKind::Identifier),
                Token::new(6, TokenKind::Identifier)
            ],
            tokens
        );
    }
}
