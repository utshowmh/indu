use std::{
    fmt::{self, Debug, Display, Formatter},
    rc::Rc,
};

use crate::runtime::{environment::Environment, interpreter::Interpreter};

use super::{ast::FunctionStatement, state::State};

#[derive(Debug, Clone)]
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

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<function({})>", self.identifier)
    }
}

pub(crate) trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, State>;
}

pub(crate) struct UserDefinedFunction {
    pub(crate) statement: FunctionStatement,
}

impl UserDefinedFunction {
    pub(crate) fn new(statement: FunctionStatement) -> Self {
        Self { statement }
    }
}

impl Callable for UserDefinedFunction {
    fn arity(&self) -> usize {
        self.statement.parameters.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, State> {
        let mut environment = Environment::new(Some(interpreter.environment.clone()));
        for (identifier, value) in self.statement.parameters.iter().zip(arguments) {
            environment.define(identifier.clone(), value);
        }
        interpreter.environment = environment;
        match interpreter.execute_statement(*self.statement.block.clone()) {
            Ok(_) => Ok(Object::Nil),
            Err(state) => match state {
                State::Return(object) => {
                    interpreter.environment =
                        if let Some(environment) = *interpreter.environment.parent.clone() {
                            environment
                        } else {
                            Environment::new(None)
                        };
                    Ok(object)
                }
                _ => Err(state),
            },
        }
    }
}
