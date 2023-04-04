#[derive(Clone, PartialEq)]
pub(crate) enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Object {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(bool) => *bool,
            Object::Number(n) => n != &0.,
            Object::String(s) => !s.is_empty(),
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Boolean(object) => write!(f, "{object}"),
            Self::Number(object) => write!(f, "{object}"),
            Self::String(object) => write!(f, "{object}"),
        }
    }
}
