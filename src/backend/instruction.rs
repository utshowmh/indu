use super::types::Value;

#[derive(Clone)]
pub enum Instruction {
    Return,
    Constatnt(Value),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}
