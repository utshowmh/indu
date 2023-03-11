use std::{io::stdin, rc::Rc};

use crate::common::{
    error::Error,
    object::{Callable, Function, Object},
    position::Position,
    token::{Token, TokenKind},
};

use super::{environment::Environment, interpreter::Interpreter};

struct Write;

impl Callable for Write {
    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, Error> {
        println!("{}", arguments[0]);
        Ok(Object::Nil)
    }
}

struct Read;

impl Callable for Read {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Object>) -> Result<Object, Error> {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        Ok(Object::String(input.trim().to_string()))
    }
}

pub(super) fn define_global_functions(environment: Environment) -> Environment {
    let mut environment = environment;

    environment.define(
        Token::new(
            TokenKind::Identifier,
            "write".to_string(),
            None,
            Position::new(0, 0),
        ),
        Object::Function(Function::new(Rc::new(Write))),
    );

    environment.define(
        Token::new(
            TokenKind::Identifier,
            "read".to_string(),
            None,
            Position::new(0, 0),
        ),
        Object::Function(Function::new(Rc::new(Read))),
    );

    environment.clone()
}
