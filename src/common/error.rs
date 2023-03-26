use super::position::Position;

#[derive(Debug)]
pub(crate) enum ErrorKind {
    SystemError,
    LexerError,
    ParserError,
    RuntimeError,
}

#[derive(Debug)]
pub(crate) struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) message: String,
    pub(crate) position: Option<Position>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, message: String, position: Option<Position>) -> Self {
        Self {
            kind,
            message,
            position,
        }
    }

    pub(crate) fn report(&self) {
        if let Some(position) = self.position.clone() {
            eprintln!("[line {}, column {}]", position.line, position.column,);
        }
        eprintln!("{:?}: {}.", self.kind, self.message,);
    }
}
