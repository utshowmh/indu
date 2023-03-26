use std::collections::HashMap;

use crate::common::{
    error::{Error, ErrorKind},
    object::Object,
    state::State,
    token::Token,
};

#[derive(Debug, Clone)]
pub(crate) struct Environment {
    bindings: HashMap<String, Object>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub(crate) fn define(&mut self, identifier: Token, value: Object) {
        self.bindings.insert(identifier.lexeme, value);
    }

    pub(super) fn assign(&mut self, identifier: Token, value: Object) -> Result<(), State> {
        if self.bindings.get(&identifier.lexeme).is_some() {
            self.define(identifier, value);
            Ok(())
        } else {
            Err(State::Error(Error::new(
                ErrorKind::RuntimeError,
                format!("Undefined variable. `{}` is not defined", identifier.lexeme),
                Some(identifier.position),
            )))
        }
    }

    pub(super) fn access(&self, identifier: Token) -> Result<Object, State> {
        if let Some(value) = self.bindings.get(&identifier.lexeme) {
            Ok(value.clone())
        } else {
            Err(State::Error(Error::new(
                ErrorKind::RuntimeError,
                format!("Undefined variable. `{}` is not defined", identifier.lexeme),
                Some(identifier.position),
            )))
        }
    }
}
