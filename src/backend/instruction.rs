use crate::common::object::Object;

#[derive(Clone)]
pub(crate) enum Instruction {
    Return,
    Print,
    DefGlobal,
    SetGlobal,
    GetGlobal,
    Push(Object),
    Pop,

    Negate,
    Not,

    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,

    And,
    Or,
}
