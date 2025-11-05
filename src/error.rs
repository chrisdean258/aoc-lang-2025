use crate::lex::Error as LexError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0}")]
    LexError(#[from] LexError),
}
