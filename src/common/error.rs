use super::position::Position;

#[derive(Debug)]
pub(crate) enum ErrorKind {
    LexerError,
    ParserError,
    RuntimeError,
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

    pub(crate) fn report(&self) {
        eprintln!(
            "[line {}, column {}]",
            self.position.line, self.position.column,
        );
        eprintln!("{:?}: {}.", self.kind, self.message,);
    }
}
