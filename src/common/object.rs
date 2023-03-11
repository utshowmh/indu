use std::{
    fmt::{self, Debug, Display, Formatter},
    rc::Rc,
};

use crate::runtime::interpreter::Interpreter;

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
            Object::Function(_) => false,
            Object::String(string) => !string.is_empty(),
            Object::Number(number) => number != &0.,
            Object::Boolean(boolean) => *boolean,
            Object::Nil => false,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Function(_) => write!(f, "<callable>"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Function {
    pub(crate) callee: Rc<dyn Callable>,
}

impl Function {
    pub(crate) fn new(callee: Rc<dyn Callable>) -> Self {
        Self { callee }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.callee, &other.callee)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<callable>")
    }
}

pub(crate) trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, Error>;
}
