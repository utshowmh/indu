use super::position::Position;

#[derive(Debug)]
pub(crate) enum ErrorKind {
    LexerError,
}

#[derive(Debug)]
pub(crate) struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) message: String,
    pub(crate) position: Position,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, message: String, position: Position) -> Self {
        Self {
            kind,
            message,
            position,
        }
    }

    pub(crate) fn report(&self, source: &str) {
        eprintln!(
            "{:?}: {} in line {}.\n",
            self.kind, self.message, self.position.line
        );
        let source_chars: Vec<char> = source.chars().collect();
        let invalid_lexeme: String = source_chars[self.position.start..self.position.end]
            .iter()
            .collect();
        eprint!("\t{}", invalid_lexeme);
        eprintln!(" <--- here\n");
    }
}
