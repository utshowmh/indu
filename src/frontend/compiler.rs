use crate::{
    backend::{chunk::Chunk, instruction::Instruction},
    common::{
        ast::{
            AssignmentExpression, BinaryExpression, BlockStatement, CallExpression, Expression,
            ExpressionStatement, GroupExpression, LiteralExpression, PrintStatement, Program,
            ReturnStatement, Statement, UnaryExpression, VariableExpression, VariableStatement,
        },
        binding::Binding,
        error::{Error, ErrorKind},
        object::Object,
        position::Position,
        token::TokenKind,
    },
};

pub(crate) struct Compiler {
    chunk: Chunk,
    bindings: Vec<Binding>,
    scope_index: usize,
}

impl Compiler {
    pub(crate) fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            bindings: Vec::new(),
            scope_index: 0,
        }
    }

    pub(crate) fn compile(&mut self, program: Program) -> Result<&Chunk, Error> {
        for statement in &program {
            self.compile_statement(statement)?;
        }

        Ok(&self.chunk)
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::Function(_) => todo!(),
            Statement::If(_) => todo!(),
            Statement::While(_) => todo!(),
            Statement::Block(statement) => self.compile_block_statement(statement),
            Statement::Variable(statement) => self.compile_variable_statement(statement),
            Statement::Return(statement) => self.compile_return_statement(statement),
            Statement::Print(statement) => self.compile_print_statement(statement),
            Statement::Expression(statement) => self.compile_expression_statement(statement),
        }
    }

    fn compile_block_statement(&mut self, statement: &BlockStatement) -> Result<(), Error> {
        todo!()
    }

    fn compile_variable_statement(&mut self, statement: &VariableStatement) -> Result<(), Error> {
        todo!()
    }

    fn compile_return_statement(&self, statement: &ReturnStatement) -> Result<(), Error> {
        todo!()
    }

    fn compile_print_statement(&mut self, statement: &PrintStatement) -> Result<(), Error> {
        todo!()
    }

    fn compile_expression_statement(
        &mut self,
        statement: &ExpressionStatement,
    ) -> Result<(), Error> {
        todo!()
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<(), Error> {
        match expression {
            Expression::Assignment(expression) => self.compile_assignment_expression(expression),
            Expression::Binary(expression) => self.compile_binary_expression(expression),
            Expression::Unary(expression) => self.compile_unary_expression(expression),
            Expression::Group(expression) => self.compile_group_expression(expression),
            Expression::Call(expression) => self.compile_call_expression(expression),
            Expression::Literal(expression) => self.compile_literal_expression(expression),
            Expression::Variable(expression) => self.compile_variable_expression(expression),
        }
    }

    fn compile_assignment_expression(
        &mut self,
        expression: &AssignmentExpression,
    ) -> Result<(), Error> {
        todo!()
    }

    fn compile_binary_expression(&mut self, expression: &BinaryExpression) -> Result<(), Error> {
        todo!()
    }

    fn compile_unary_expression(&mut self, expression: &UnaryExpression) -> Result<(), Error> {
        todo!()
    }

    fn compile_call_expression(&mut self, expression: &CallExpression) -> Result<(), Error> {
        todo!()
    }

    fn compile_group_expression(&mut self, expression: &GroupExpression) -> Result<(), Error> {
        self.compile_expression(&*expression.child)
    }

    fn compile_literal_expression(&mut self, expression: &LiteralExpression) -> Result<(), Error> {
        match expression.value.kind {
            TokenKind::Number => {
                let object = expression.value.lexeme.parse().unwrap();
                self.chunk.add_instruction(
                    Instruction::Push(Object::Number(object)),
                    expression.position(),
                );
            }
            TokenKind::String => {
                let object = expression.value.lexeme.parse().unwrap();
                self.chunk.add_instruction(
                    Instruction::Push(Object::String(object)),
                    expression.position(),
                );
            }
            TokenKind::False | TokenKind::True => {
                let object = expression.value.lexeme.parse().unwrap();
                self.chunk.add_instruction(
                    Instruction::Push(Object::Boolean(object)),
                    expression.position(),
                );
            }
            TokenKind::Nil => {
                self.chunk
                    .add_instruction(Instruction::Push(Object::Nil), expression.position());
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn compile_variable_expression(
        &mut self,
        expression: &VariableExpression,
    ) -> Result<(), Error> {
        todo!()
    }
}
