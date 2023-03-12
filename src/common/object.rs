use std::{
    fmt::{self, Debug, Display, Formatter},
    rc::Rc,
};

use super::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Function(Function),
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Object {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Self::Function(_) => false,
            Self::String(string) => !string.is_empty(),
            Self::Number(number) => number != &0.,
            Self::Boolean(boolean) => *boolean,
            Self::Nil => false,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Function(function) => write!(f, "<function({})>", function.identifier),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Function {
    pub(crate) identifier: String,
    pub(crate) callee: Rc<dyn Callable>,
}

impl Function {
    pub(crate) fn new(identifier: String, callee: Rc<dyn Callable>) -> Self {
        Self { identifier, callee }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.callee, &other.callee)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<function({})>", self.identifier)
    }
}

pub(crate) trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, arguments: Vec<Object>) -> Result<Object, Error>;
}
