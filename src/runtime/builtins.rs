use std::io::{stdin, stdout};

use crate::common::{
    error::{Error, ErrorKind},
    object::{Callable, Object},
    state::State,
};

use super::interpreter::Interpreter;

pub(super) struct Write;

impl Callable for Write {
    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, State> {
        print!("{}", arguments[0]);
        std::io::Write::flush(&mut stdout()).or(Err(State::Error(Error::new(
            ErrorKind::SystemError,
            "Could not flush stdout".to_string(),
            None,
        ))))?;
        Ok(Object::Nil)
    }
}

pub(super) struct WriteLine;

impl Callable for WriteLine {
    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, State> {
        println!("{}", arguments[0]);
        Ok(Object::Nil)
    }
}

pub(super) struct Read;

impl Callable for Read {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: Vec<Object>) -> Result<Object, State> {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .or(Err(State::Error(Error::new(
                ErrorKind::SystemError,
                "Could not read from stdin".to_string(),
                None,
            ))))?;
        let input = input.trim().to_string();
        if let Ok(number) = input.parse() {
            Ok(Object::Number(number))
        } else {
            Ok(Object::String(input))
        }
    }
}