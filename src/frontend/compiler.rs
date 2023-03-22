use crate::{
    backend::{chunk::Chunk, instruction::Instruction},
    common::{
        ast::{
            BinaryExpression, Expression, LiteralExpression, Program, Statement, UnaryExpression,
        },
        error::{Error, ErrorKind},
        position::Position,
        token::TokenKind,
        types::Value,
    },
};

pub(crate) struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub(crate) fn new() -> Self {
        Self {
            chunk: Chunk::new(),
        }
    }

    pub(crate) fn compile(&mut self, program: Program) -> Result<&Chunk, Error> {
        for statement in program {
            self.compile_statement(statement)?;
        }
        self.chunk
            .add_instruction(Instruction::Return, Position::new(0, 0));
        Ok(&self.chunk)
    }

    fn compile_statement(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Expression(expression) => self.compile_expression(expression.expression)?,
            _ => todo!(),
        }

        Ok(())
    }

    fn compile_expression(&mut self, expression: Expression) -> Result<(), Error> {
        match expression {
            Expression::Binary(expression) => self.compile_binary_expression(expression),
            Expression::Unary(expression) => self.compile_unary_expression(expression),
            Expression::Group(expression) => self.compile_expression(*expression.child),
            Expression::Literal(expression) => self.compile_literal_expression(expression),
            _ => todo!(),
        }
    }

    fn compile_binary_expression(&mut self, expression: BinaryExpression) -> Result<(), Error> {
        self.compile_expression(*expression.left)?;
        self.compile_expression(*expression.right)?;
        match expression.operator.kind {
            TokenKind::Plus => self
                .chunk
                .add_instruction(Instruction::Add, expression.operator.position),
            TokenKind::Minus => self
                .chunk
                .add_instruction(Instruction::Subtract, expression.operator.position),
            TokenKind::Star => self
                .chunk
                .add_instruction(Instruction::Multiply, expression.operator.position),
            TokenKind::Slash => self
                .chunk
                .add_instruction(Instruction::Divide, expression.operator.position),
            _ => {
                return Err(Error::new(
                    ErrorKind::CompilerError,
                    format!("`{}` is not a binary operator", expression.operator.lexeme),
                    Some(expression.operator.position),
                ))
            }
        }

        Ok(())
    }

    fn compile_unary_expression(&mut self, expression: UnaryExpression) -> Result<(), Error> {
        match expression.operator.kind {
            TokenKind::Minus => {
                self.compile_expression(*expression.right)?;
                self.chunk
                    .add_instruction(Instruction::Negate, expression.operator.position);
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::CompilerError,
                    format!("`{}` is not an unary operator", expression.operator.lexeme),
                    Some(expression.operator.position),
                ))
            }
        }

        Ok(())
    }

    fn compile_literal_expression(&mut self, expression: LiteralExpression) -> Result<(), Error> {
        self.chunk.add_instruction(
            Instruction::Constatnt(Value::Number(expression.value.lexeme.parse().unwrap())), // We're making sure it's a number (f64) in scanner.
            expression.position(),
        );
        Ok(())
    }
}
