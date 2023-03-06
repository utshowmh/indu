use crate::common::{
    ast::{
        BinaryExpression, Expression, ExpressionStatement, GroupExpression, LiteralExpression,
        PrintStatement, Statement, UnaryExpression, VariableExpression, VariableStatement,
    },
    error::{Error, ErrorKind},
    object::Object,
    position::Position,
    token::TokenKind,
};

use super::environment::Environment;

pub(crate) struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            environment: Environment::new(),
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
            Statement::Variable(statement) => self.execute_variable_statement(statement),
            Statement::Print(statement) => self.execute_print_statement(statement),
            Statement::Expression(statement) => self.execute_expression_statement(statement),
        }
    }

    fn execute_variable_statement(&mut self, statement: VariableStatement) -> Result<(), Error> {
        let value = match statement.initializer {
            Some(expression) => self.evaluate_expression(expression)?,
            None => Object::Nil,
        };
        self.environment.set(statement.identifier.lexeme, value);

        Ok(())
    }

    fn execute_print_statement(&self, statement: PrintStatement) -> Result<(), Error> {
        let value = self.evaluate_expression(statement.expression)?;
        println!("{}", value);

        Ok(())
    }

    fn execute_expression_statement(&self, statement: ExpressionStatement) -> Result<(), Error> {
        self.evaluate_expression(statement.expression)?;

        Ok(())
    }
}

impl Interpreter {
    fn evaluate_expression(&self, expression: Expression) -> Result<Object, Error> {
        match expression {
            Expression::Binary(expression) => self.evaluate_binary_expression(expression),
            Expression::Unary(expression) => self.evaluate_unary_expression(expression),
            Expression::Group(expression) => self.evaluate_group_expression(expression),
            Expression::Literal(expression) => self.evaluate_literal_expression(expression),
            Expression::Variable(expression) => self.evaluate_variable_expression(expression),
        }
    }

    fn evaluate_binary_expression(&self, expression: BinaryExpression) -> Result<Object, Error> {
        let left_value = self.evaluate_expression(*expression.left)?;
        let right_value = self.evaluate_expression(*expression.right)?;

        match expression.operator.kind {
            TokenKind::Plus => match (&left_value, &right_value) {
                (Object::Number(left_value), Object::Number(right_value)) => {
                    Ok(Object::Number(left_value + right_value))
                }
                (Object::String(left_value), Object::String(right_value)) => {
                    Ok(Object::String(format!("{}{}", left_value, right_value)))
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

            TokenKind::And => match (&left_value, &right_value) {
                (Object::Boolean(left_value), Object::Boolean(right_value)) => {
                    Ok(Object::Boolean(left_value.clone() && right_value.clone()))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            TokenKind::Or => match (&left_value, &right_value) {
                (Object::Boolean(left_value), Object::Boolean(right_value)) => {
                    Ok(Object::Boolean(left_value.clone() || right_value.clone()))
                }
                (_, _) => Err(self.generate_invalid_binary_operation_error(
                    &expression.operator.lexeme,
                    left_value,
                    right_value,
                    expression.operator.position,
                )),
            },

            _ => Err(self.generate_unexpected_operator_error(
                &expression.operator.lexeme,
                "binary",
                expression.operator.position,
            )),
        }
    }

    fn evaluate_unary_expression(&self, expression: UnaryExpression) -> Result<Object, Error> {
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

    fn evaluate_group_expression(&self, expression: GroupExpression) -> Result<Object, Error> {
        self.evaluate_expression(*expression.child)
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
        self.environment.get(expression.identifier)
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
            format!(
                "Invalid unary operation. `{}` is not defined for `{}`",
                operator_lexeme, value
            ),
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
                "Invalid binary operation. `{}` is not defined for `{}` and `{}`",
                operator_lexeme, left_value, right_value
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
                "Unexpexted operator. `{}` is not as {} operator",
                operator_lexeme, current_operation,
            ),
            operator_position,
        )
    }

    fn generate_error(&self, message: String, position: Position) -> Error {
        Error::new(ErrorKind::RuntimeError, message, position)
    }
}
