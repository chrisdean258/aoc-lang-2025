use crate::{lex::Error as LexError, parse::Error as ParseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    LexError(#[from] LexError),
    #[error("{0}")]
    ParseError(#[from] ParseError),
    #[error("{0}")]
    StdIOError(#[from] std::io::Error),
}
