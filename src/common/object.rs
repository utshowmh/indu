use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
impl Object {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Object::String(string) => !string.is_empty(),
            Object::Number(number) => number != &0.,
            Object::Boolean(boolean) => *boolean,
            Object::Nil => false,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}
