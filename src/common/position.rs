#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Position {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) line: usize,
}

impl Position {
    pub(crate) fn new(start: usize, end: usize, line: usize) -> Self {
        Self { start, end, line }
    }
}
