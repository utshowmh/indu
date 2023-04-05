use std::collections::HashMap;

use crate::{
    backend::{chunk::Chunk, instruction::Instruction},
    common::{
        ast::{
            AssignmentExpression, BinaryExpression, BlockStatement, ElseStatement, Expression,
            ExpressionStatement, IfStatement, LiteralExpression, PrintStatement, Program,
            ReturnStatement, Statement, UnaryExpression, VariableExpression, VariableStatement,
        },
        error::{Error, ErrorKind},
        object::Object,
        position::Position,
        token::TokenKind,
    },
};

pub(crate) struct Compiler {
    chunk: Chunk,
    bindings: HashMap<String, Expression>,
}

impl Compiler {
    pub(crate) fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            bindings: HashMap::new(),
        }
    }

    pub(crate) fn compile(&mut self, program: Program) -> Result<Chunk, Error> {
        for statement in &program {
            self.compile_statement(statement)?;
        }
        self.chunk
            .add_instruction(Instruction::Return, Position::new(0, 0, 0));
        Ok(self.chunk.clone())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::Function(_) => todo!(),
            Statement::If(statement) => self.compile_if_statement(statement),
            Statement::While(_) => todo!(),
            Statement::Block(statement) => self.compile_block_statement(statement),
            Statement::Variable(statement) => self.compile_variable_statement(statement),
            Statement::Return(statement) => self.compile_return_statement(statement),
            Statement::Print(statement) => self.compile_print_statement(statement),
            Statement::Expression(statement) => self.compile_expression_statement(statement),
        }
    }

    fn compile_if_statement(&mut self, statement: &IfStatement) -> Result<(), Error> {
        self.compile_expression(&statement.condition)?;
        let patch_index = self.chunk.add_instruction(
            Instruction::JumpIfFalse(usize::MAX),
            statement.condition.position(),
        );
        self.compile_block_statement(&statement.then_branch)?;
        if let Some(else_statement) = &*statement.else_branch {
            match else_statement {
                ElseStatement::If(if_statement) => {
                    self.patch_if_statement(patch_index, statement);
                    self.compile_if_statement(if_statement)?;
                }
                ElseStatement::Block(block) => {
                    self.patch_if_statement(patch_index, statement);
                    self.compile_block_statement(block)?;
                }
            }
        } else {
            self.patch_if_statement(patch_index, statement);
        }
        Ok(())
    }

    fn compile_block_statement(&mut self, statement: &BlockStatement) -> Result<(), Error> {
        for statement in &statement.statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_variable_statement(&mut self, statement: &VariableStatement) -> Result<(), Error> {
        self.compile_expression(&statement.initializer)?;
        self.bindings.insert(
            statement.identifier.lexeme.clone(),
            statement.initializer.clone(),
        );
        Ok(())
    }

    fn compile_return_statement(&mut self, statement: &ReturnStatement) -> Result<(), Error> {
        self.compile_expression(&statement.expression)?;
        self.chunk
            .add_instruction(Instruction::Return, statement.expression.position());
        Ok(())
    }

    fn compile_print_statement(&mut self, statement: &PrintStatement) -> Result<(), Error> {
        self.compile_expression(&statement.expression)?;
        self.chunk
            .add_instruction(Instruction::Print, statement.expression.position());
        Ok(())
    }

    fn compile_expression_statement(
        &mut self,
        statement: &ExpressionStatement,
    ) -> Result<(), Error> {
        self.compile_expression(&statement.expression)?;
        self.chunk
            .add_instruction(Instruction::Pop, statement.expression.position());
        Ok(())
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<(), Error> {
        match expression {
            Expression::Assignment(expression) => self.compile_assignment_expression(expression),
            Expression::Binary(expression) => self.compile_binary_expression(expression),
            Expression::Unary(expression) => self.compile_unary_expression(expression),
            Expression::Group(expression) => self.compile_expression(&expression.child),
            Expression::Call(_) => todo!(),
            Expression::Literal(expression) => self.compile_literal_expression(expression),
            Expression::Variable(expression) => self.compile_variable_expression(expression),
        }
    }

    fn compile_assignment_expression(
        &mut self,
        expression: &AssignmentExpression,
    ) -> Result<(), Error> {
        self.bindings
            .get(&expression.identifier.lexeme)
            .ok_or(Error::new(
                ErrorKind::Compiler,
                format!(
                    "Undefined identifier. '{}' is not defined.",
                    expression.identifier.lexeme
                ),
                Some(expression.position()),
            ))?;
        self.compile_expression(&expression.initializer)?;
        self.bindings.insert(
            expression.identifier.lexeme.clone(),
            *expression.initializer.clone(),
        );
        Ok(())
    }

    fn compile_binary_expression(&mut self, expression: &BinaryExpression) -> Result<(), Error> {
        self.compile_expression(&expression.left)?;
        self.compile_expression(&expression.right)?;
        match expression.operator.kind {
            TokenKind::Plus => {
                self.chunk
                    .add_instruction(Instruction::Add, expression.position());
                Ok(())
            }
            TokenKind::Minus => {
                self.chunk
                    .add_instruction(Instruction::Subtract, expression.position());
                Ok(())
            }
            TokenKind::Star => {
                self.chunk
                    .add_instruction(Instruction::Multiply, expression.position());
                Ok(())
            }
            TokenKind::Slash => {
                self.chunk
                    .add_instruction(Instruction::Divide, expression.position());
                Ok(())
            }

            TokenKind::Equal => {
                self.chunk
                    .add_instruction(Instruction::Equal, expression.position());
                Ok(())
            }
            TokenKind::NotEqual => {
                self.chunk
                    .add_instruction(Instruction::NotEqual, expression.position());
                Ok(())
            }
            TokenKind::Greater => {
                self.chunk
                    .add_instruction(Instruction::Greater, expression.position());
                Ok(())
            }
            TokenKind::GreaterEqual => {
                self.chunk
                    .add_instruction(Instruction::GreaterEqual, expression.position());
                Ok(())
            }
            TokenKind::Lesser => {
                self.chunk
                    .add_instruction(Instruction::Lesser, expression.position());
                Ok(())
            }
            TokenKind::LesserEqual => {
                self.chunk
                    .add_instruction(Instruction::LesserEqual, expression.position());
                Ok(())
            }

            TokenKind::And => {
                self.chunk
                    .add_instruction(Instruction::And, expression.position());
                Ok(())
            }
            TokenKind::Or => {
                self.chunk
                    .add_instruction(Instruction::Or, expression.position());
                Ok(())
            }

            _ => Err(Error::new(
                ErrorKind::Compiler,
                format!(
                    "Invalid operator. '{}' is not a binary operator.",
                    expression.operator.lexeme
                ),
                Some(expression.position()),
            )),
        }
    }

    fn compile_unary_expression(&mut self, expression: &UnaryExpression) -> Result<(), Error> {
        self.compile_expression(&expression.right)?;
        match expression.operator.kind {
            TokenKind::Plus => {
                self.chunk
                    .add_instruction(Instruction::Identify, expression.position());
                Ok(())
            }

            TokenKind::Minus => {
                self.chunk
                    .add_instruction(Instruction::Negate, expression.position());
                Ok(())
            }

            TokenKind::Not => {
                self.chunk
                    .add_instruction(Instruction::Not, expression.position());
                Ok(())
            }

            _ => Err(Error::new(
                ErrorKind::Compiler,
                format!(
                    "Invalid operator. '{}' is not an unary operator.",
                    expression.operator.lexeme
                ),
                Some(expression.position()),
            )),
        }
    }

    fn compile_literal_expression(&mut self, expression: &LiteralExpression) -> Result<(), Error> {
        if expression.value.kind == TokenKind::Nil {
            self.chunk
                .add_instruction(Instruction::Push(Object::Nil), expression.position());
        } else if expression.value.kind == TokenKind::Number {
            self.chunk.add_instruction(
                Instruction::Push(Object::Number(expression.value.lexeme.parse().unwrap())),
                expression.position(),
            );
        } else if expression.value.kind == TokenKind::True
            || expression.value.kind == TokenKind::False
        {
            self.chunk.add_instruction(
                Instruction::Push(Object::Boolean(expression.value.lexeme.parse().unwrap())),
                expression.position(),
            );
        } else {
            self.chunk.add_instruction(
                Instruction::Push(Object::String(expression.value.lexeme.clone())),
                expression.position(),
            );
        }

        Ok(())
    }

    fn compile_variable_expression(
        &mut self,
        expression: &VariableExpression,
    ) -> Result<(), Error> {
        let initializer = self
            .bindings
            .get(&expression.identifier.lexeme)
            .ok_or(Error::new(
                ErrorKind::Compiler,
                format!(
                    "Undefined identifier. '{}' is not defined.",
                    expression.identifier.lexeme
                ),
                Some(expression.position()),
            ))?;
        self.compile_expression(&initializer.clone())?;

        Ok(())
    }

    fn patch_if_statement(&mut self, patch_index: usize, statement: &IfStatement) {
        let jump_address = self
            .chunk
            .add_instruction(Instruction::Continue, statement.condition.position());
        self.chunk
            .edit_instruction(patch_index, Instruction::JumpIfFalse(jump_address));
    }
}
