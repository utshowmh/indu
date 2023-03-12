use std::rc::Rc;

use crate::common::{
    object::{Function, Object},
    position::Position,
    token::{Token, TokenKind},
};

use super::{
    builtins::{Read, Write, WriteLine},
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
        Object::Function(Function::new("write".to_string(), Rc::new(Write))),
    );
    environment.define(
        Token::new(
            TokenKind::Identifier,
            "writeln".to_string(),
            None,
            Position::new(0, 0),
        ),
        Object::Function(Function::new("writeln".to_string(), Rc::new(WriteLine))),
    );
    environment.define(
        Token::new(
            TokenKind::Identifier,
            "read".to_string(),
            None,
            Position::new(0, 0),
        ),
        Object::Function(Function::new("read".to_string(), Rc::new(Read))),
    );

    environment.clone()
}
