#![allow(dead_code)]
use crate::error::LexError;
use crate::location;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    filename: &'a str,
    src: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub const fn new(filename: &'a str, src: &'a str) -> Self {
        Self {
            src,
            filename,
            offset: 0,
        }
    }

    fn lex_number(&self) -> Result<(usize, TokenKind), LexError> {
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
        let Some(is_digit) = chars.next().map(|c| c.is_digit(radix)) else {
            return Err(LexError::UnexpectedEOF(
                location::resolve(self.offset + 2, self.filename.into(), self.src.into()),
                "lexing number",
            ));
        };
        if !is_digit {
            return Err(LexError::InvalidIntegerDigit(
                location::resolve(self.offset, self.filename.into(), self.src.into()),
                c,
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
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, LexError>;
    fn next(&mut self) -> Option<Self::Item> {
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
                    .find(|c: char| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(unlexed.len());
                (len, TokenKind::Identifier)
            }
            '+' => (1, TokenKind::Plus),
            '-' => (1, TokenKind::Minus),
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
}
