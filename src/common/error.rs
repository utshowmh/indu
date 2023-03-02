use super::position::Position;

#[derive(Debug)]
pub enum ErrorKind {
    LexerError,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub position: Position,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String, position: Position) -> Self {
        Self {
            kind,
            message,
            position,
        }
    }

    pub fn report(&self, source: &str) {
        eprintln!("ERROR: {} in line {}.\n", self.message, self.position.line);
        let source_chars: Vec<char> = source.chars().collect();
        let invalid_lexeme: String = source_chars[self.position.start..self.position.end]
            .iter()
            .collect();
        eprint!("\t{}", invalid_lexeme);
        eprintln!(" <--- here\n");
    }
}
