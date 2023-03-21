use super::types::Value;
use crate::common::{
    error::{Error, ErrorKind},
    position::Position,
};

use super::instruction::Instruction;

// Sequence of Instruction
#[derive(Clone)]
pub(crate) struct Chunk {
    instructions: Vec<Instruction>,
    lines: Vec<Position>,
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Self {
            instructions: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub(crate) fn add_instruction(&mut self, instruction: Instruction, line: Position) {
        self.instructions.push(instruction);
        self.lines.push(line);
    }

    pub(crate) fn get_instruction(&self, instruction_index: usize) -> Result<Instruction, Error> {
        let instruction = self.instructions.get(instruction_index).ok_or(Error::new(
            ErrorKind::RuntimeError,
            format!("Invalid instruction index"),
            None,
        ))?;
        Ok(instruction.clone())
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
            instruction_index, self.lines[instruction_index].line, instruction_name,
        );
    }

    fn debug_constant_instruction(
        &self,
        instruction_index: usize,
        value: Value,
    ) -> Result<(), Error> {
        println!(
            "{:04} {} {} '{}'",
            instruction_index, self.lines[instruction_index].line, "const", value
        );
        Ok(())
    }

    pub(crate) fn ip_is_valid(&self, ip: usize) -> bool {
        ip < self.instructions.len()
    }
}
