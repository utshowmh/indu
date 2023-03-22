use crate::common::{
    error::{Error, ErrorKind},
    types::Value,
};

use super::{chunk::Chunk, instruction::Instruction};

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
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    println!("{value}");
                    break;
                }
                Instruction::Constatnt(value) => self.stack.push(value),
                Instruction::Negate => {
                    let value = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    if let Value::Number(num) = value {
                        self.stack.push(Value::Number(-num));
                    } else {
                        panic!()
                    }
                }
                Instruction::Add => {
                    let b = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    let a = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a + b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`+` is not defined for {a} and {b}"),
                                Some(chunk.get_position(self.ip)?),
                            ))
                        }
                    };
                }
                Instruction::Subtract => {
                    let b = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    let a = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a - b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`-` is not defined for {a} and {b}"),
                                Some(chunk.get_position(self.ip)?),
                            ))
                        }
                    };
                }
                Instruction::Multiply => {
                    let b = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    let a = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a * b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`*` is not defined for {a} and {b}"),
                                Some(chunk.get_position(self.ip)?),
                            ))
                        }
                    };
                }
                Instruction::Divide => {
                    let b = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    let a = self.stack.pop().ok_or(Error::new(
                        ErrorKind::RuntimeError,
                        "Stack underflow".to_string(),
                        None,
                    ))?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a / b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`/` is not defined for {a} and {b}"),
                                Some(chunk.get_position(self.ip)?),
                            ))
                        }
                    };
                }
            }
        }

        Ok(())
    }

    fn get_instruction(&mut self, chunk: &Chunk) -> Result<Instruction, Error> {
        self.ip += 1;
        chunk.get_instruction(self.ip - 1)
    }
}
