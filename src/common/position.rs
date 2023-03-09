#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Position {
    pub(crate) column: usize,
    pub(crate) line: usize,
}

impl Position {
    pub(crate) fn new(column: usize, line: usize) -> Self {
        Self { column, line }
    }
}
