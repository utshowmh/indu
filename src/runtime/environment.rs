use std::collections::HashMap;

use crate::common::{
    error::{Error, ErrorKind},
    object::Object,
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

    pub(super) fn get(&self, identifier: Token) -> Result<Object, Error> {
        if let Some(value) = self.bindings.get(&identifier.lexeme) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                ErrorKind::RuntimeError,
                format!("Undefined variable. `{}` is not defined", identifier.lexeme),
                identifier.position,
            ))
        }
    }

    pub(super) fn set(&mut self, identifier: String, value: Object) {
        self.bindings.insert(identifier, value);
    }
}
