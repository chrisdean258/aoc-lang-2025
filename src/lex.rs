#![allow(dead_code)]
use crate::{
    location,
    location::SrcLocation,
    token::{Token, TokenKind},
};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Lexer<'a, P: AsRef<Path>> {
    filename: P,
    src: &'a str,
    offset: usize,
    error: bool,
}

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0}: Unexpected character '{1}'")]
    UnexpectedChar(SrcLocation, char),
    #[error("{0}: Expected digit with radix {1} found '{2}'")]
    InvalidIntegerDigit(SrcLocation, char, u32),
    #[error("{0}: Unexpected EOF when {1}")]
    UnexpectedEOF(SrcLocation, &'static str),
}

impl<'a, P: AsRef<Path>> Lexer<'a, P> {
    #[must_use]
    pub const fn new(filename: P, src: &'a str) -> Self {
        Self {
            src,
            filename,
            offset: 0,
            error: false,
        }
    }

    fn lex_number(&self) -> Result<(usize, TokenKind), Error> {
        let unlexed = &self.src[self.offset..];
        let mut chars = unlexed.chars();
        let _c = chars
            .next()
            .expect("This should be only called with a gauranteed int");
        let Some(c) = chars.next() else {
            return Ok((1, TokenKind::Integer));
        };

        let radix = if c == 'x' {
            16
        } else if c == 'o' {
            8
        } else if c == 'b' {
            2
        } else {
            return Ok((1, TokenKind::Integer));
        };
        let Some(poss_digit) = chars.next() else {
            return Err(Error::UnexpectedEOF(
                location::resolve(
                    self.offset + 2,
                    self.filename.as_ref().to_string_lossy().into(),
                    self.src.into(),
                ),
                "lexing number",
            ));
        };
        if !poss_digit.is_digit(radix) {
            return Err(Error::InvalidIntegerDigit(
                location::resolve(
                    self.offset + 2,
                    self.filename.as_ref().to_string_lossy().into(),
                    self.src.into(),
                ),
                poss_digit,
                radix,
            ));
        }
        let unlexed = &self.src[self.offset + 2..];
        let (l, v) = Self::lex_number_with_radix(unlexed, radix);
        Ok((l + 2, v))
    }

    fn lex_number_with_radix(unlexed: &str, radix: u32) -> (usize, TokenKind) {
        let num_digits = |unlexed: &str| {
            unlexed
                .find(|c: char| !c.is_digit(radix))
                .unwrap_or(unlexed.len())
        };
        let len = num_digits(unlexed);
        if unlexed[len..].starts_with('.') {
            let unlexed = &unlexed[len + 1..];
            let len2 = num_digits(unlexed);
            if len2 > 0 {
                return (len + len2 + 1, TokenKind::Float);
            }
        }
        (len, TokenKind::Integer)
    }

    fn lex_single_token(&mut self) -> Option<Result<Token, Error>> {
        let unlexed = &self.src[self.offset..];
        //Skip Leading whitespace
        let ws_len = unlexed.find(|c: char| !c.is_whitespace())?;
        self.offset += ws_len;
        let unlexed = &self.src[self.offset..];
        let mut chars = unlexed.chars();
        let c = chars.next()?;
        let (len, tk) = match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                let len = unlexed
                    .find(|c: char| !c.is_alphanumeric() && c != '_' && c != '\'')
                    .unwrap_or(unlexed.len());
                (len, TokenKind::Identifier)
            }
            '+' => (1, TokenKind::Plus),
            '*' => (1, TokenKind::Star),
            '-' => (1, TokenKind::Minus),
            '/' => (1, TokenKind::Slash),
            '1'..='9' => Self::lex_number_with_radix(unlexed, 10),
            '0' => match self.lex_number() {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            },
            _ => return None,
        };

        let rv = Token::new(self.offset, tk);
        self.offset += len;
        Some(Ok(rv))
    }
}

impl<P: AsRef<Path>> Iterator for Lexer<'_, P> {
    type Item = Result<Token, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }
        let val = self.lex_single_token();
        self.error = matches!(val, Some(Err(_)));
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {
        let lexer = Lexer::new("test", "test");
        let tokens = lexer.collect::<Result<Vec<_>, _>>().expect("Lexing Error");
        assert_eq!(vec![Token::new(0, TokenKind::Identifier)], tokens);
    }

    #[test]
    fn ids() {
        let lexer = Lexer::new("test", "hello world");
        let tokens = lexer.collect::<Result<Vec<_>, _>>().expect("Lexing Error");
        assert_eq!(
            vec![
                Token::new(0, TokenKind::Identifier),
                Token::new(6, TokenKind::Identifier)
            ],
            tokens
        );
    }

    #[test]
    fn ids_tick() {
        let lexer = Lexer::new("test", "hello hello' hello");
        let tokens = lexer.collect::<Result<Vec<_>, _>>().expect("Lexing Error");
        assert_eq!(
            vec![
                Token::new(0, TokenKind::Identifier),
                Token::new(6, TokenKind::Identifier),
                Token::new(13, TokenKind::Identifier)
            ],
            tokens
        );
    }

    #[test]
    fn ids_plus() {
        let lexer = Lexer::new("test", "hello + world");
        let tokens = lexer.collect::<Result<Vec<_>, _>>().expect("Lexing Error");
        assert_eq!(
            vec![
                Token::new(0, TokenKind::Identifier),
                Token::new(6, TokenKind::Plus),
                Token::new(8, TokenKind::Identifier)
            ],
            tokens
        );
    }

    #[test]
    fn num_plus() {
        let lexer = Lexer::new("test", "1 + 1.0");
        let tokens = lexer.collect::<Result<Vec<_>, _>>().expect("Lexing Error");
        assert_eq!(
            vec![
                Token::new(0, TokenKind::Integer),
                Token::new(2, TokenKind::Plus),
                Token::new(4, TokenKind::Float)
            ],
            tokens
        );
    }
    #[test]
    fn hex() {
        let lexer = Lexer::new("test", "0xff");
        let tokens = lexer.collect::<Result<Vec<_>, _>>().expect("Lexing Error");
        assert_eq!(vec![Token::new(0, TokenKind::Integer),], tokens);
    }

    #[test]
    fn hex_plus_id() {
        let lexer = Lexer::new("test", "0xff + is");
        let tokens = lexer.collect::<Result<Vec<_>, _>>().expect("Lexing Error");
        assert_eq!(
            vec![
                Token::new(0, TokenKind::Integer),
                Token::new(5, TokenKind::Plus),
                Token::new(7, TokenKind::Identifier),
            ],
            tokens
        );
    }

    #[test]
    fn bad_hex() {
        let lexer = Lexer::new("test", "0xm");
        let err = lexer
            .collect::<Result<Vec<_>, _>>()
            .expect_err("We lexed 0xm?");
        assert!(matches!(
            err,
            Error::InvalidIntegerDigit(SrcLocation { .. }, 'm', 16)
        ));
    }

    #[test]
    fn hex_eof() {
        let lexer = Lexer::new("test", "0x");
        let err = lexer
            .collect::<Result<Vec<_>, _>>()
            .expect_err("We lexed 0xm?");
        assert!(matches!(
            err,
            Error::UnexpectedEOF(SrcLocation { .. }, "lexing number")
        ));
    }
}
