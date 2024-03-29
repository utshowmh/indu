use crate::common::{
    error::{Error, ErrorKind},
    object::Object,
};

use super::{chunk::Chunk, instruction::Instruction};

pub(crate) struct VirtualMachine {
    ip: usize,
    stack: Vec<Object>,
}

impl VirtualMachine {
    pub(crate) fn new() -> Self {
        Self {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub(crate) fn interpret(&mut self, chunk: Chunk) -> Result<(), Error> {
        self.ip = 0;
        self.stack = Vec::new();
        self.run(&chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> Result<(), Error> {
        while chunk.ip_is_valid(self.ip) {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("stack [");
                for object in &self.stack {
                    print!(" {object},");
                }
                println!(" ]");
                chunk.debug_instruction(self.ip);
            }

            match self.get_instruction(chunk) {
                Instruction::Return => break,

                Instruction::Print => println!("{}", self.stack.pop().unwrap()),

                Instruction::Push(object) => self.stack.push(object),

                Instruction::Pop => {
                    self.stack.pop().unwrap();
                }

                Instruction::Identify => {
                    let object = self.stack.pop().unwrap();
                    if let Object::Number(num) = object {
                        self.stack.push(Object::Number(num));
                    } else {
                        return Err(Error::new(
                            ErrorKind::Runtime,
                            format!("Invalid operator. Unary operator '-' is not defined for '{object}'."),
                            Some(chunk.get_position(self.ip - 1)),
                        ));
                    }
                }

                Instruction::Negate => {
                    let object = self.stack.pop().unwrap();
                    if let Object::Number(num) = object {
                        self.stack.push(Object::Number(-num));
                    } else {
                        return Err(Error::new(
                            ErrorKind::Runtime,
                            format!("Invalid operator. Unary operator '-' is not defined for '{object}'."),
                            Some(chunk.get_position(self.ip - 1)),
                        ));
                    }
                }

                Instruction::Not => {
                    let object = self.stack.pop().unwrap();
                    if let Object::Boolean(bool) = object {
                        self.stack.push(Object::Boolean(!bool));
                    } else {
                        return Err(Error::new(
                            ErrorKind::Runtime,
                            format!("Invalid operator. Unary operator '!' is not defined for '{object}'."),
                            Some(chunk.get_position(self.ip - 1)),
                        ));
                    }
                }

                Instruction::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Number(a + b))
                        }
                        (Object::String(a), Object::String(b)) => {
                            self.stack.push(Object::String(format!("{a}{b}")))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Runtime,
                                format!("Invalid operator. Binary operator '+' is not defined for '{a}' and '{b}'."),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Subtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Number(a - b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Runtime,
                                format!("Invalid operator. Binary operator '-' is not defined for '{a}' and '{b}'."),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Multiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Number(a * b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Runtime,
                                format!("Invalid operator. Binary operator '*' is not defined for '{a}' and '{b}'."),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Divide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            if b == &0. {
                                return Err(Error::new(
                                    ErrorKind::Runtime,
                                    format!("Division by zero. Can not divide '{a}' by '0'."),
                                    Some(chunk.get_position(self.ip - 1)),
                                ));
                            }
                            self.stack.push(Object::Number(a / b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Runtime,
                                format!("Invalid operator. Binary operator '/' is not defined for '{a}' and '{b}'"),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Equal => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Object::Boolean(a == b));
                }

                Instruction::NotEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Object::Boolean(a != b));
                }

                Instruction::Greater => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Boolean(a > b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Runtime,
                                format!("Invalid operator. Binary operator '>' is not defined for '{a}' and '{b}'."),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::GreaterEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Boolean(a >= b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Runtime,
                                format!("Invalid operator. Binary operator '>=' is not defined for '{a}' and '{b}'."),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::Lesser => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Boolean(a < b));
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Runtime,
                                format!("Invalid operator. Binary operator '<' is not defined for '{a}' and '{b}'."),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::LesserEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (&a, &b) {
                        (Object::Number(a), Object::Number(b)) => {
                            self.stack.push(Object::Boolean(a <= b))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Runtime,
                                format!("Invalid operator. Binary operator '<=' is not defined for '{a}' and '{b}'."),
                                Some(chunk.get_position(self.ip - 1)),
                            ))
                        }
                    };
                }

                Instruction::And => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack
                        .push(Object::Boolean(a.is_truthy() && b.is_truthy()))
                }

                Instruction::Or => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack
                        .push(Object::Boolean(a.is_truthy() || b.is_truthy()))
                }

                Instruction::JumpIfFalse(ip) => {
                    let a = self.stack.pop().unwrap();
                    if !a.is_truthy() {
                        self.ip = ip;
                    }
                }

                Instruction::Continue => {}
            }
        }

        Ok(())
    }

    fn get_instruction(&mut self, chunk: &Chunk) -> Instruction {
        self.ip += 1;
        chunk.get_instruction(self.ip - 1)
    }
}
