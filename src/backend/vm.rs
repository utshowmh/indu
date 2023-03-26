use std::collections::HashMap;

use crate::common::{
    error::{Error, ErrorKind},
    object::Object,
};

use super::{chunk::Chunk, instruction::Instruction};

pub(crate) struct VirtualMachine {
    ip: usize,
    stack: Vec<Object>,
    globals: HashMap<String, Object>,
}

impl VirtualMachine {
    pub(crate) fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub(crate) fn interpret(&mut self, chunk: &Chunk) -> Result<(), Error> {
        self.ip = 0;
        self.stack = Vec::new();
        self.run(&chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> Result<(), Error> {
        while chunk.ip_is_valid(self.ip) {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("statck [");
                for object in &self.stack {
                    print!(" {object},");
                }
                println!(" ]");
                chunk.debug_instruction(self.ip);
            }

            match self.get_instruction(chunk) {
                Instruction::Return => break,

                Instruction::Print => println!("{}", self.pop_stack(chunk)?),

                Instruction::DefGlobal => {
                    let object = self.pop_stack(chunk)?;
                    if let Object::String(name) = object {
                        let object = self.pop_stack(chunk)?;
                        self.globals.insert(name, object);
                    } else {
                        unreachable!();
                    }
                }

                Instruction::SetGlobal => {
                    let object = self.pop_stack(chunk)?;
                    if let Object::String(name) = object {
                        if let Some(_) = self.globals.get(&name) {
                            let object = self.pop_stack(chunk)?;
                            self.globals.insert(name, object.clone());
                            self.stack.push(object);
                        } else {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`{name}` is not defined"),
                                Some(chunk.get_position(self.ip - 1)),
                            ));
                        }
                    } else {
                        unreachable!();
                    }
                }

                Instruction::GetGlobal => {
                    if let Object::String(name) = self.pop_stack(chunk)? {
                        if let Some(object) = self.globals.get(&name) {
                            self.stack.push(object.clone());
                        } else {
                            return Err(Error::new(
                                ErrorKind::RuntimeError,
                                format!("`{name}` is not defined"),
                                Some(chunk.get_position(self.ip - 1)),
                            ));
                        }
                    } else {
                        unreachable!();
                    }
                }

                Instruction::Push(object) => self.stack.push(object),

                Instruction::Pop => {
                    self.pop_stack(chunk)?;
                }

                Instruction::Negate => {
                    let object = self.pop_stack(chunk)?;
                    if let Object::Number(num) = object {
                        self.stack.push(Object::Number(-num));
                    } else {
                        return Err(Error::new(
                            ErrorKind::RuntimeError,
                            format!("`-` is not defined for `{object}`"),
                            Some(chunk.get_position(self.ip - 1)),
                        ));
                    }
                }

                Instruction::Not => {
                    let object = self.pop_stack(chunk)?;
                    if let Object::Boolean(bool) = object {
                        self.stack.push(Object::Boolean(!bool));
                    } else {
                        return Err(Error::new(
                            ErrorKind::RuntimeError,
                            format!("`!` is not defined for `{object}`"),
                            Some(chunk.get_position(self.ip - 1)),
                        ));
                    }
                }

                Instruction::Add => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Number(a + b))
                        }
                        (Object::String(a), Object::String(b)) => {
                            self.stack.push(Object::String(format!("{a}{b}")))
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
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Number(a - b))
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
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Number(a * b))
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
                        (Object::Number(a), Object::Number(b)) => {
                            if b == &0. {
                                return Err(Error::new(
                                    ErrorKind::RuntimeError,
                                    "Division by 0".to_string(),
                                    Some(chunk.get_position(self.ip - 1)),
                                ));
                            }
                            self.stack.push(Object::Number(a / b))
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
                    self.stack.push(Object::Boolean(a == b));
                }

                Instruction::NotEqual => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    self.stack.push(Object::Boolean(a != b));
                }

                Instruction::Greater => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Boolean(a > b))
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
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Boolean(a >= b))
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
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Boolean(a < b));
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
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Boolean(a <= b))
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
                        .push(Object::Boolean(a.is_truthy() && b.is_truthy()))
                }

                Instruction::Or => {
                    let b = self.pop_stack(chunk)?;
                    let a = self.pop_stack(chunk)?;
                    self.stack
                        .push(Object::Boolean(a.is_truthy() || b.is_truthy()))
                }
            }
        }

        Ok(())
    }

    fn pop_stack(&mut self, chunk: &Chunk) -> Result<Object, Error> {
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
