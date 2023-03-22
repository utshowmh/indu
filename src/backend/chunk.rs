use crate::common::{
    error::{Error, ErrorKind},
    position::Position,
    types::Value,
};

use super::instruction::Instruction;

// Sequence of Instruction
pub(crate) struct Chunk {
    instructions: Vec<Instruction>,
    positions: Vec<Position>,
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Self {
            instructions: Vec::new(),
            positions: Vec::new(),
        }
    }

    pub(crate) fn add_instruction(&mut self, instruction: Instruction, position: Position) {
        self.instructions.push(instruction);
        self.positions.push(position);
    }

    pub(crate) fn get_instruction(&self, instruction_index: usize) -> Result<Instruction, Error> {
        let instruction = self.instructions.get(instruction_index).ok_or(Error::new(
            ErrorKind::RuntimeError,
            "Invalid instruction index".to_string(),
            None,
        ))?;
        Ok(instruction.clone())
    }

    pub(crate) fn get_position(&self, position_index: usize) -> Result<Position, Error> {
        let position = self.positions.get(position_index).ok_or(Error::new(
            ErrorKind::RuntimeError,
            "Invalid instruction index".to_string(),
            None,
        ))?;
        Ok(position.clone())
    }

    pub(crate) fn debug_instruction(&self, instruction_index: usize) -> Result<(), Error> {
        let instruction = self.get_instruction(instruction_index)?;
        match instruction {
            Instruction::Return => self.debug_simple_instruction("ret", instruction_index),
            Instruction::Constatnt(value) => {
                self.debug_constant_instruction(instruction_index, value)?
            }

            Instruction::Negate => self.debug_simple_instruction("neg", instruction_index),
            Instruction::Add => self.debug_simple_instruction("add", instruction_index),
            Instruction::Subtract => self.debug_simple_instruction("sub", instruction_index),
            Instruction::Multiply => self.debug_simple_instruction("mul", instruction_index),
            Instruction::Divide => self.debug_simple_instruction("div", instruction_index),
        };

        Ok(())
    }

    fn debug_simple_instruction(&self, instruction_name: &str, instruction_index: usize) {
        println!(
            "{:04} {} {}",
            instruction_index, self.positions[instruction_index].line, instruction_name,
        );
    }

    fn debug_constant_instruction(
        &self,
        instruction_index: usize,
        value: Value,
    ) -> Result<(), Error> {
        println!(
            "{:04} {} const '{}'",
            instruction_index, self.positions[instruction_index].line, value
        );
        Ok(())
    }

    pub(crate) fn ip_is_valid(&self, ip: usize) -> bool {
        ip < self.instructions.len()
    }
}
