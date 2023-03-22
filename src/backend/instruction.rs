use crate::common::types::Value;

#[derive(Clone)]
pub(crate) enum Instruction {
    Return,
    Constatnt(Value),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}
