use crate::common::{object::Object, position::Position};

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

    pub(crate) fn add_instruction(&mut self, instruction: Instruction, position: Position) {
        self.instructions.push(instruction);
        self.positions.push(position);
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
            Instruction::Return => self.debug_simple_instruction("ret", instruction_index),
            Instruction::Print => self.debug_simple_instruction("print", instruction_index),

            Instruction::Push(object) => {
                self.debug_constant_instruction("push", instruction_index, object);
            }
            Instruction::Pop => self.debug_simple_instruction("pop", instruction_index),

            Instruction::Negate => self.debug_simple_instruction("neg", instruction_index),
            Instruction::Not => self.debug_simple_instruction("not", instruction_index),

            Instruction::Add => self.debug_simple_instruction("add", instruction_index),
            Instruction::Subtract => self.debug_simple_instruction("sub", instruction_index),
            Instruction::Multiply => self.debug_simple_instruction("mul", instruction_index),
            Instruction::Divide => self.debug_simple_instruction("div", instruction_index),

            Instruction::Equal => self.debug_simple_instruction("eq", instruction_index),
            Instruction::NotEqual => self.debug_simple_instruction("neq", instruction_index),
            Instruction::Greater => self.debug_simple_instruction("ge", instruction_index),
            Instruction::GreaterEqual => self.debug_simple_instruction("geq", instruction_index),
            Instruction::Lesser => self.debug_simple_instruction("le", instruction_index),
            Instruction::LesserEqual => self.debug_simple_instruction("leq", instruction_index),

            Instruction::And => self.debug_simple_instruction("and", instruction_index),
            Instruction::Or => self.debug_simple_instruction("or", instruction_index),
        };
    }

    fn debug_simple_instruction(&self, instruction_name: &str, instruction_index: usize) {
        println!(
            "{:04} {} {}",
            instruction_index, self.positions[instruction_index].line, instruction_name,
        );
    }

    fn debug_constant_instruction(
        &self,
        instruction_name: &str,
        instruction_index: usize,
        object: Object,
    ) {
        println!(
            "{:04} {} {} '{}'",
            instruction_index, self.positions[instruction_index].line, instruction_name, object
        );
    }

    pub(crate) fn ip_is_valid(&self, ip: usize) -> bool {
        ip < self.instructions.len()
    }
}
