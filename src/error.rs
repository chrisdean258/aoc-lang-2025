use crate::location::SrcLocation;

#[derive(Debug, Clone)]
pub enum LexError {
    UnexpectedChar(char, SrcLocation),
}

#[derive(Debug, Clone)]
pub enum Error {
    LexError(LexError),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedChar(c, l) => write!(f, "{l}: Unexpected character '{c}'"),
        }
    }
}

impl std::error::Error for LexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::LexError(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::LexError(e) => Some(e),
        }
    }
}
