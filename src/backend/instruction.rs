use crate::common::types::Value;

#[derive(Clone)]
pub(crate) enum Instruction {
    Return,
    Print,

    Push(Value),
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
