use crate::common::{
    ast::{
        AssignmentExpression, BinaryExpression, BlockStatement, CallExpression, Expression,
        ExpressionStatement, FunctionStatement, GroupExpression, IfStatement, LiteralExpression,
        Statement, UnaryExpression, VariableExpression, VariableStatement, WhileStatement,
    },
    error::{Error, ErrorKind},
    object::Object,
    position::Position,
    token::TokenKind,
};

use super::{environment::Environment, globals::define_global_functions};

pub(crate) struct Interpreter {
    pub(crate) environment: Environment,
}

impl Interpreter {
    pub(crate) fn new(environment: Environment) -> Self {
        Self {
            environment: define_global_functions(environment),
        }
    }

    pub(crate) fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), Error> {
        for statement in statements {
            self.execute_statement(statement)?;
        }

        Ok(())
    }
}

impl Interpreter {
    fn execute_statement(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Function(statement) => self.execute_function_statement(statement),
            Statement::If(statement) => self.execute_if_statement(statement),
            Statement::While(statement) => self.execute_while_statement(statement),
            Statement::Block(statement) => self.execute_block_statement(statement),
            Statement::Variable(statement) => self.execute_variable_statement(statement),
            Statement::Expression(statement) => self.execute_expression_statement(statement),
        }
    }

    fn execute_function_statement(&self, _statement: FunctionStatement) -> Result<(), Error> {
        todo!()
    }

    fn execute_if_statement(&mut self, statement: IfStatement) -> Result<(), Error> {
        let condition = self.evaluate_expression(statement.condition)?;
        if condition.is_truthy() {
            self.execute_statement(*statement.then_block)?;
        } else if let Some(else_block) = *statement.else_block {
            self.execute_statement(else_block)?;
        }

        Ok(())
    }

    fn execute_while_statement(&mut self, statement: WhileStatement) -> Result<(), Error> {
        let mut condition = self.evaluate_expression(statement.condition.clone())?;
        while condition.is_truthy() {
            self.execute_statement(*statement.do_block.clone())?;
            condition = self.evaluate_expression(statement.condition.clone())?;
        }

        Ok(())
    }

    fn execute_block_statement(&mut self, statement: BlockStatement) -> Result<(), Error> {
        for statement in statement.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_variable_statement(&mut self, statement: VariableStatement) -> Result<(), Error> {
        let value = match statement.initializer {
            Some(expression) => self.evaluate_expression(expression)?,
            None => Object::Nil,
        };
        self.environment.define(statement.identifier, value);

        Ok(())
    }

    fn execute_expression_statement(
        &mut self,
        statement: ExpressionStatement,
    ) -> Result<(), Error> {
        self.evaluate_expression(statement.expression)?;

        Ok(())
    }
}

impl Interpreter {
    fn evaluate_expression(&mut self, expression: Expression) -> Result<Object, Error> {
        match expression {
            Expression::Assignment(expression) => self.evaluate_assignment_expression(expression),
            Expression::Binary(expression) => self.evaluate_binary_expression(expression),
            Expression::Unary(expression) => self.evaluate_unary_expression(expression),
            Expression::Group(expression) => self.evaluate_group_expression(expression),
            Expression::Call(expression) => self.evaluate_call_expression(expression),
            Expression::Literal(expression) => self.evaluate_literal_expression(expression),
            Expression::Variable(expression) => self.evaluate_variable_expression(expression),
        }
    }

    fn evaluate_assignment_expression(
        &mut self,
        expression: AssignmentExpression,
    ) -> Result<Object, Error> {
        let value = self.evaluate_expression(*expression.initializer)?;
        self.environment
            .assign(expression.identifier, value.clone())?;
        Ok(value)
    }

    fn evaluate_binary_expression(
        &mut self,
        expression: BinaryExpression,
    ) -> Result<Object, Error> {
        let left_value = self.evaluate_expression(*expression.left)?;
        let right_value = self.evaluate_expression(*expression.right)?;

        match expression.operator.kind {
            TokenKind::Plus => match (&left_value, &right_value) {
                (Object::Number(left_value), Object::Number(right_value)) => {
                    Ok(Object::Number(left_value + right_value))
                }
                (Object::String(left_value), Object::String(right_value)) => {
                    Ok(Object::String(format!("{left_value}{right_value}")))
                }
                (Object::String(left_value), Object::Number(right_value)) => {
                    Ok(Object::String(format!("{left_value}{right_value}")))
                }
                (Object::Number(left_value), Object::String(right_value)) => {
                    Ok(Object::String(format!("{left_value}{right_value}")))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            TokenKind::Minus => match (&left_value, &right_value) {
                (Object::Number(left_value), Object::Number(right_value)) => {
                    Ok(Object::Number(left_value - right_value))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            TokenKind::Star => match (&left_value, &right_value) {
                (Object::Number(left_value), Object::Number(right_value)) => {
                    Ok(Object::Number(left_value * right_value))
                }
                (Object::String(left_value), Object::Number(right_value)) => {
                    let mut string = String::new();
                    for _ in 0..*right_value as usize {
                        string.push_str(left_value);
                    }
                    Ok(Object::String(string))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            TokenKind::Slash => match (&left_value, &right_value) {
                (Object::Number(left_value), Object::Number(right_value)) => {
                    Ok(Object::Number(left_value / right_value))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            TokenKind::Equal => Ok(Object::Boolean(left_value == right_value)),

            TokenKind::NotEqual => Ok(Object::Boolean(left_value != right_value)),

            TokenKind::Greater => match (&left_value, &right_value) {
                (Object::Number(left_value), Object::Number(right_value)) => {
                    Ok(Object::Boolean(left_value > right_value))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            TokenKind::GreaterEqual => match (&left_value, &right_value) {
                (Object::Number(left_value), Object::Number(right_value)) => {
                    Ok(Object::Boolean(left_value >= right_value))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            TokenKind::Lesser => match (&left_value, &right_value) {
                (Object::Number(left_value), Object::Number(right_value)) => {
                    Ok(Object::Boolean(left_value < right_value))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            TokenKind::LesserEqual => match (&left_value, &right_value) {
                (Object::Number(left_value), Object::Number(right_value)) => {
                    Ok(Object::Boolean(left_value <= right_value))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            TokenKind::And => {
                if !left_value.is_truthy() {
                    Ok(left_value)
                } else {
                    Ok(right_value)
                }
            }

            TokenKind::Or => {
                if left_value.is_truthy() {
                    Ok(left_value)
                } else {
                    Ok(right_value)
                }
            }

            _ => Err(self.generate_unexpected_operator_error(
                &expression.operator.lexeme,
                "binary",
                expression.operator.position,
            )),
        }
    }

    fn evaluate_unary_expression(&mut self, expression: UnaryExpression) -> Result<Object, Error> {
        let value = self.evaluate_expression(*expression.right)?;

        match expression.operator.kind {
            TokenKind::Not => match value {
                Object::Boolean(value) => Ok(Object::Boolean(!value)),
                _ => self.generate_invalid_unary_operation_error(
                    &expression.operator.lexeme,
                    value,
                    expression.operator.position,
                ),
            },

            TokenKind::Minus => match value {
                Object::Number(value) => Ok(Object::Number(-value)),
                _ => self.generate_invalid_unary_operation_error(
                    &expression.operator.lexeme,
                    value,
                    expression.operator.position,
                ),
            },

            _ => Err(self.generate_unexpected_operator_error(
                &expression.operator.lexeme,
                "unary",
                expression.operator.position,
            )),
        }
    }

    fn evaluate_group_expression(&mut self, expression: GroupExpression) -> Result<Object, Error> {
        self.evaluate_expression(*expression.child)
    }

    fn evaluate_call_expression(&mut self, expression: CallExpression) -> Result<Object, Error> {
        let callee_position = expression.position();
        let callee = self.evaluate_expression(*expression.callee)?;
        let mut arguments = Vec::new();
        for argument in expression.arguments {
            arguments.push(self.evaluate_expression(argument)?);
        }

        if let Object::Function(function) = callee {
            if function.callee.arity() == arguments.len() {
                function.callee.call(arguments)
            } else {
                Err(self.generate_error(
                    format!(
                        "Expected {} arguments, got {}",
                        function.callee.arity(),
                        arguments.len()
                    ),
                    callee_position,
                ))
            }
        } else {
            Err(self.generate_error(format!("`{callee}` is not callable"), callee_position))
        }
    }

    fn evaluate_literal_expression(&self, expression: LiteralExpression) -> Result<Object, Error> {
        if let Some(value) = expression.value {
            Ok(value)
        } else {
            panic!("FIXME: Add proper error handling.")
        }
    }

    fn evaluate_variable_expression(
        &self,
        expression: VariableExpression,
    ) -> Result<Object, Error> {
        self.environment.access(expression.identifier)
    }
}

impl Interpreter {
    fn generate_invalid_unary_operation_error(
        &self,
        operator_lexeme: &str,
        value: Object,
        operator_position: Position,
    ) -> Result<Object, Error> {
        Err(self.generate_error(
            format!("Invalid unary operation. `{operator_lexeme}` is not defined for `{value}`"),
            operator_position,
        ))
    }

    fn generate_invalid_binary_operation_error(
        &self,
        operator_lexeme: &str,
        left_value: Object,
        right_value: Object,
        operator_position: Position,
    ) -> Error {
        self.generate_error(
            format!(
                "Invalid binary operation. `{operator_lexeme}` is not defined for `{left_value}` and `{right_value}`"
            ),
            operator_position,
        )
    }

    fn generate_unexpected_operator_error(
        &self,
        operator_lexeme: &str,
        current_operation: &str,
        operator_position: Position,
    ) -> Error {
        self.generate_error(
            format!(
                "Unexpexted operator. `{operator_lexeme}` is not as {current_operation} operator",
            ),
            operator_position,
        )
    }

    fn generate_error(&self, message: String, position: Position) -> Error {
        Error::new(ErrorKind::RuntimeError, message, Some(position))
    }
}
