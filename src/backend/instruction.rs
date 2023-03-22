use crate::common::types::Value;

#[derive(Clone)]
pub(crate) enum Instruction {
    Return,

    Constatnt(Value),

    Negate,
    Not,

    Add,
    Subtract,
    Multiply,
    Divide,
}
