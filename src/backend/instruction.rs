use crate::common::object::Object;

#[derive(Clone)]
pub(crate) enum Instruction {
    Return,
    Print,

    Push(Object),
    Pop,

    Identify,
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

    JumpIfFalse(usize),

    Continue,
}
