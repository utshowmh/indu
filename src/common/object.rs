#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
