use std::rc::Rc;

use crate::common::{
    object::{Function, Object},
    position::Position,
    token::{Token, TokenKind},
};

use super::{
    builtins::{Read, Write},
    environment::Environment,
};

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
