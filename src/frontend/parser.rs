use crate::common::{
    ast::{
        AssignmentExpression, BinaryExpression, Expression, ExpressionStatement, GroupExpression,
        LiteralExpression, PrintStatement, Statement, UnaryExpression, VariableExpression,
        VariableStatement,
    },
    error::{Error, ErrorKind},
    token::{Token, TokenKind},
};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current_position: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_position: 0,
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();

        while !self.current_token_is_eof() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.current_token().kind {
            TokenKind::Var => self.parse_var_statement(),
            TokenKind::Print => self.parse_print_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_var_statement(&mut self) -> Result<Statement, Error> {
        self.advance_current_position();
        let identifier = self.consume_token(TokenKind::Identifier)?;
        let mut initializer = None;
        if self.current_token_matches(&[TokenKind::Assign]) {
            self.advance_current_position();
            initializer = Some(self.parse_expression()?);
        }
        self.consume_token(TokenKind::Semicolon)?;

        Ok(Statement::Variable(VariableStatement::new(
            identifier,
            initializer,
        )))
    }

    fn parse_print_statement(&mut self) -> Result<Statement, Error> {
        self.advance_current_position();
        let expression = self.parse_expression()?;
        self.consume_token(TokenKind::Semicolon)?;
        Ok(Statement::Print(PrintStatement::new(expression)))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let expression = self.parse_expression()?;
        self.consume_token(TokenKind::Semicolon)?;
        Ok(Statement::Expression(ExpressionStatement::new(expression)))
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> Result<Expression, Error> {
        let assign_token = self.current_token();
        let expression = self.parse_binary_expression()?;

        if self.current_token_matches(&[TokenKind::Assign]) {
            self.advance_current_position();
            let initializer = self.parse_assignment_expression()?;
            if let Expression::Variable(expression) = expression {
                return Ok(Expression::Assignment(AssignmentExpression::new(
                    expression.identifier,
                    initializer,
                )));
            } else {
                return Err(Error::new(
                    ErrorKind::ParserError,
                    format!("Invalid assignment target"),
                    assign_token.position,
                ));
            }
        }

        Ok(expression)
    }

    fn parse_binary_expression(&mut self) -> Result<Expression, Error> {
        self.parse_logical_or_expression()
    }

    fn parse_logical_or_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_logical_and_expression()?;
        while self.current_token_matches(&[TokenKind::Or]) {
            let operator = self.next_token();
            let right = self.parse_logical_and_expression()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn parse_logical_and_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_equality_expression()?;
        while self.current_token_matches(&[TokenKind::And]) {
            let operator = self.next_token();
            let right = self.parse_equality_expression()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn parse_equality_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_comparison_expression()?;
        while self.current_token_matches(&[TokenKind::Equal, TokenKind::NotEqual]) {
            let operator = self.next_token();
            let right = self.parse_comparison_expression()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn parse_comparison_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_additive_expression()?;
        while self.current_token_matches(&[
            TokenKind::Greater,
            TokenKind::Lesser,
            TokenKind::GreaterEqual,
            TokenKind::LesserEqual,
        ]) {
            let operator = self.next_token();
            let right = self.parse_additive_expression()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn parse_additive_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_multiplicatiove_expression()?;
        while self.current_token_matches(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.next_token();
            let right = self.parse_multiplicatiove_expression()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn parse_multiplicatiove_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_unary_expression()?;
        while self.current_token_matches(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.next_token();
            let right = self.parse_unary_expression()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, Error> {
        if self.current_token_matches(&[TokenKind::Minus, TokenKind::Not]) {
            let operator = self.next_token();
            let right = self.parse_primary_expression()?;
            Ok(Expression::Unary(UnaryExpression::new(operator, right)))
        } else {
            self.parse_primary_expression()
        }
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, Error> {
        if self.current_token_matches(&[
            TokenKind::Number,
            TokenKind::String,
            TokenKind::True,
            TokenKind::False,
            TokenKind::Nil,
        ]) {
            Ok(Expression::Literal(LiteralExpression::new(
                self.next_token().literal,
            )))
        } else if self.current_token_matches(&[TokenKind::Identifier]) {
            Ok(Expression::Variable(VariableExpression::new(
                self.next_token(),
            )))
        } else if self.current_token_matches(&[TokenKind::OpenParen]) {
            self.advance_current_position();
            let child = self.parse_expression()?;
            self.consume_token(TokenKind::CloseParen)?;
            Ok(Expression::Group(GroupExpression::new(child)))
        } else {
            Err(self.generate_error(format!(
                "Unexpected token `{}`",
                self.current_token().lexeme
            )))
        }
    }

    fn current_token_is_eof(&self) -> bool {
        self.current_token().kind == TokenKind::EOF
    }

    fn current_token(&self) -> Token {
        self.tokens[self.current_position].clone()
    }

    fn advance_current_position(&mut self) {
        self.current_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = self.current_token();
        self.advance_current_position();
        token
    }

    fn current_token_matches(&self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self.current_token().kind)
    }

    fn consume_token(&mut self, kind: TokenKind) -> Result<Token, Error> {
        if self.current_token().kind == kind {
            Ok(self.next_token())
        } else {
            Err(self.generate_error(format!(
                "Unexpected token`{}`, expected `{}`",
                self.current_token().lexeme,
                kind
            )))
        }
    }

    fn generate_error(&self, message: String) -> Error {
        Error::new(
            ErrorKind::ParserError,
            message,
            self.current_token().position.clone(),
        )
    }
}
