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
                print!("statck [");
                for value in &self.stack {
                    print!(" {value},");
                }
                println!(" ]");
                chunk.debug_instruction(self.ip);
            }

            match self.get_instruction(chunk) {
                Instruction::Return => {
                    let value = self.pop_stack(chunk)?;
                    println!("{value}");
                    break;
                }

                Instruction::Constatnt(value) => self.stack.push(value),

                Instruction::Negate => {
                    let value = self.pop_stack(chunk)?;
                    if let Value::Number(num) = value {
                        self.stack.push(Value::Number(-num));
                    } else {
                        return Err(Error::new(
                            ErrorKind::RuntimeError,
                            format!("`-` is not defined for `{value}`"),
                            Some(chunk.get_position(self.ip - 1)),
                        ));
                    }
                }

                Instruction::Not => {
                    let value = self.pop_stack(chunk)?;
                    if let Value::Boolean(bool) = value {
                        self.stack.push(Value::Boolean(!bool));
                    } else {
                        return Err(Error::new(
                            ErrorKind::RuntimeError,
                            format!("`!` is not defined for `{value}`"),
                            Some(chunk.get_position(self.ip - 1)),
                        ));
                    }
                }

                Instruction::Add => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a + b))
                        }
                        (Value::String(a), Value::String(b)) => {
                            self.stack.push(Value::String(format!("{a}{b}")))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`+` is not defined for `{a}` and `{b}`"),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Subtract => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a - b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`-` is not defined for `{a}` and `{b}`"),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Multiply => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a * b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`*` is not defined for `{a}` and `{b}`"),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Divide => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            if b == &0. {
                                return Err(Error::new(
                                    ErrorKind::RuntimeError,
                                    "Division by 0".to_string(),
                                    Some(chunk.get_position(self.ip - 1)),
                                ));
                            }
                            self.stack.push(Value::Number(a / b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`/` is not defined for `{a}` and `{b}`"),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Equal => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    self.stack.push(Value::Boolean(a == b));
                }

                Instruction::NotEqual => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    self.stack.push(Value::Boolean(a != b));
                }

                Instruction::Greater => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a > b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`>` is not defined for `{a}` and `{b}`"),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::GreaterEqual => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a >= b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`>=` is not defined for `{a}` and `{b}`"),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Lesser => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a < b));
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`<` is not defined for `{a}` and `{b}`"),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::LesserEqual => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Boolean(a <= b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`<=` is not defined for `{a}` and `{b}`"),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::And => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    self.stack
                        .push(Value::Boolean(a.is_truthy() && b.is_truthy()))
                }

                Instruction::Or => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    self.stack
                        .push(Value::Boolean(a.is_truthy() || b.is_truthy()))
                }
            }
        }

        Ok(())
    }

    fn pop_stack(&mut self, chunk: &Chunk) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::new(
            ErrorKind::RuntimeError,
            "Stack underflow".to_string(),
            Some(chunk.get_position(self.ip - 1)),
        ))
    }

    fn get_instruction(&mut self, chunk: &Chunk) -> Instruction {
        self.ip += 1;
        chunk.get_instruction(self.ip - 1)
    }
}
