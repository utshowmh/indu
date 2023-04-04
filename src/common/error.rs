use std::fmt::Display;

use super::position::Position;

#[derive(Debug)]
pub(crate) enum ErrorKind {
    System,
    Lexer,
    Parser,
    Runtime,
    Compiler,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::System => write!(f, "SystemError"),
            ErrorKind::Lexer => write!(f, "LexerError"),
            ErrorKind::Parser => write!(f, "ParserError"),
            ErrorKind::Compiler => write!(f, "CompilerError"),
            ErrorKind::Runtime => write!(f, "RuntimeError"),
        }
    }
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

    pub(crate) fn report_in_source(&self, source: &str) {
        if let Some(position) = self.position.clone() {
            let lines = source.split("\n").collect::<Vec<_>>();
            let line = lines[position.line];

            eprintln!("Error in line {}:", position.line + 1);
            eprint!("{} | ", position.line + 1);
            eprintln!("{line}");

            eprint!("    ");
            if position.line == 0 {
                for _ in 0..position.start {
                    eprint!(" ");
                }
            } else {
                for _ in lines[position.line - 1].len()..position.start {
                    // FIXME: I am a bug!
                    eprint!(" ");
                }
            }
            for _ in position.start..position.end {
                eprint!("^");
            }
            eprintln!(" here");
        }
        eprintln!("{}: {}", self.kind, self.message);
    }

    pub(crate) fn report(&self) {
        if let Some(position) = self.position.clone() {
            eprintln!(
                "[line {}, column {}:{}]",
                position.line + 1,
                position.start,
                position.end
            );
        }
        eprintln!("{}: {}", self.kind, self.message);
    }
}
