use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Number(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::Nil => write!(f, "nil"),
        }
    }
}
