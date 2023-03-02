#[derive(Debug)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl Position {
    pub fn new(start: usize, end: usize, line: usize) -> Self {
        Self { start, end, line }
    }
}
