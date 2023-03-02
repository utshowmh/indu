#[derive(Debug)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
