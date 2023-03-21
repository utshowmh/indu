use crate::common::error::{Error, ErrorKind};

use super::{chunk::Chunk, instruction::Instruction, types::Value};

pub(crate) struct VirtualMachine {
    ip: usize,
    stack: Vec<Value>,
}

impl VirtualMachine {
    pub(crate) fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub(crate) fn interpret(&mut self, chunk: &Chunk, debug: bool) -> Result<(), Error> {
        self.run(&chunk, debug)
    }

    fn run(&mut self, chunk: &Chunk, debug: bool) -> Result<(), Error> {
        while chunk.ip_is_valid(self.ip) {
            if debug {
                println!("stack: {:?}", self.stack);
                chunk.debug_instruction(self.ip)?;
            }

            match self.get_instruction(chunk)? {
                Instruction::Return => {
                    let value = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        format!("Stack underflow"),
                        None,
                    ))?;
                    println!("{value}");
                    break;
                }
                Instruction::Constatnt(value) => self.stack.push(value),
                Instruction::Negate => {
                    let value = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        format!("Stack underflow"),
                        None,
                    ))?;
                    self.stack.push(-value);
                }
                Instruction::Add => self.binary_operation(|a, b| a + b)?,
                Instruction::Subtract => self.binary_operation(|a, b| a - b)?,
                Instruction::Multiply => self.binary_operation(|a, b| a * b)?,
                Instruction::Divide => self.binary_operation(|a, b| a / b)?,
            }
        }

        Ok(())
    }

    fn get_instruction(&mut self, chunk: &Chunk) -> Result<Instruction, Error> {
        self.ip += 1;
        Ok(chunk.get_instruction(self.ip - 1)?)
    }

    fn binary_operation(
        &mut self,
        operation: fn(a: Value, b: Value) -> Value,
    ) -> Result<(), Error> {
        let b = self.stack.pop().ok_or(Error::new(
            ErrorKind::RuntimeError,
            format!("Stack underflow"),
            None,
        ))?;
        let a = self.stack.pop().ok_or(Error::new(
            ErrorKind::RuntimeError,
            format!("Stack underflow"),
            None,
        ))?;
        self.stack.push(operation(a, b));
        Ok(())
    }
}
