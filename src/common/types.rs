#[derive(Clone, PartialEq)]
pub(crate) enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Value {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(bool) => *bool,
            Value::Number(num) => num != &0.,
            Value::String(str) => !str.is_empty(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
        }
    }
}
