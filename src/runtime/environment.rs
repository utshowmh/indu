use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use crate::common::{
    error::{Error, ErrorKind},
    object::Object,
    token::Token,
};

#[derive(Debug, Clone)]
pub(crate) struct Environment {
    bindings: HashMap<String, Object>,
    pub(crate) enclosing: Box<Option<Environment>>,
}

impl Environment {
    pub(crate) fn new(enclosing: Option<Environment>) -> Self {
        Self {
            bindings: HashMap::new(),
            enclosing: Box::new(enclosing),
        }
    }

    pub(super) fn define(&mut self, identifier: Token, value: Object) {
        self.bindings.insert(identifier.lexeme, value);
    }

    pub(super) fn assign(&mut self, identifier: Token, value: Object) -> Result<(), Error> {
        if self.bindings.get(&identifier.lexeme).is_some() {
            self.define(identifier, value);
            Ok(())
        } else if let Some(environment) = &mut *self.enclosing {
            environment.define(identifier, value);
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::RuntimeError,
                format!("Undefined variable. `{}` is not defined", identifier.lexeme),
                identifier.position,
            ))
        }
    }

    pub(super) fn access(&self, identifier: Token) -> Result<Object, Error> {
        if let Some(value) = self.bindings.get(&identifier.lexeme) {
            Ok(value.clone())
        } else if let Some(environment) = &*self.enclosing {
            environment.access(identifier)
        } else {
            Err(Error::new(
                ErrorKind::RuntimeError,
                format!("Undefined variable. `{}` is not defined", identifier.lexeme),
                identifier.position,
            ))
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "variables:")?;
        for (name, value) in &self.bindings {
            writeln!(f, "   {name} = {value}")?;
        }
        Ok(())
    }
}
