use std::fmt::Display;

use crate::common::position::Position;

use super::instruction::Instruction;

#[derive(Clone)]
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

    pub(crate) fn add_instruction(
        &mut self,
        instruction: Instruction,
        position: Position,
    ) -> usize {
        self.instructions.push(instruction);
        self.positions.push(position);
        self.instructions.len() - 1
    }

    pub(crate) fn edit_instruction(&mut self, instruction_index: usize, instruction: Instruction) {
        self.instructions[instruction_index] = instruction;
    }

    pub(crate) fn get_instruction(&self, instruction_index: usize) -> Instruction {
        self.instructions[instruction_index].clone()
    }

    pub(crate) fn get_position(&self, position_index: usize) -> Position {
        self.positions[position_index].clone()
    }

    #[allow(unused)]
    pub(crate) fn debug_instruction(&self, instruction_index: usize) {
        let instruction = self.get_instruction(instruction_index);
        match instruction {
            Instruction::Return => self.debug_simple_instruction("return", instruction_index),
            Instruction::Print => self.debug_simple_instruction("print", instruction_index),

            Instruction::Push(object) => {
                self.debug_complex_instruction("push", instruction_index, object);
            }
            Instruction::Pop => self.debug_simple_instruction("pop", instruction_index),

            Instruction::Identify => self.debug_simple_instruction("identify", instruction_index),
            Instruction::Negate => self.debug_simple_instruction("negate", instruction_index),
            Instruction::Not => self.debug_simple_instruction("not", instruction_index),

            Instruction::Add => self.debug_simple_instruction("add", instruction_index),
            Instruction::Subtract => self.debug_simple_instruction("sub", instruction_index),
            Instruction::Multiply => self.debug_simple_instruction("multiply", instruction_index),
            Instruction::Divide => self.debug_simple_instruction("divide", instruction_index),

            Instruction::Equal => self.debug_simple_instruction("equal", instruction_index),
            Instruction::NotEqual => self.debug_simple_instruction("not_equal", instruction_index),
            Instruction::Greater => self.debug_simple_instruction("greater", instruction_index),
            Instruction::GreaterEqual => {
                self.debug_simple_instruction("greater_equal", instruction_index)
            }
            Instruction::Lesser => self.debug_simple_instruction("lesser", instruction_index),
            Instruction::LesserEqual => {
                self.debug_simple_instruction("lesser_equal", instruction_index)
            }

            Instruction::And => self.debug_simple_instruction("and", instruction_index),
            Instruction::Or => self.debug_simple_instruction("or", instruction_index),

            Instruction::JumpIfFalse(ip) => {
                self.debug_complex_instruction("jump_if_false", instruction_index, ip)
            }

            Instruction::Continue => self.debug_simple_instruction("continue", instruction_index),
        };
    }

    fn debug_simple_instruction(&self, instruction_name: &str, instruction_index: usize) {
        println!(
            "{:014} {} {}",
            instruction_index, self.positions[instruction_index].line, instruction_name,
        );
    }

    fn debug_complex_instruction<T>(
        &self,
        instruction_name: &str,
        instruction_index: usize,
        operand: T,
    ) where
        T: Display,
    {
        println!(
            "{:014} {} {} '{}'",
            instruction_index, self.positions[instruction_index].line, instruction_name, operand
        );
    }

    pub(crate) fn ip_is_valid(&self, ip: usize) -> bool {
        ip < self.instructions.len()
    }
}
