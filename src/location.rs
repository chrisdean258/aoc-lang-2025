pub type SrcOffset = usize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SrcLocation {
    src: String,
    filename: String,
    line: usize,
    col: usize,
}

#[must_use]
pub fn resolve(
    loc: SrcOffset,
    filename: String,
    src: String,
    _newlines: Vec<usize>,
) -> SrcLocation {
    SrcLocation {
        src,
        filename,
        line: 1,
        col: loc,
    }
}

impl std::fmt::Display for SrcLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.col)
    }
}
