use crate::common::{
    ast::{
        AssignmentExpression, BinaryExpression, BlockStatement, CallExpression, Expression,
        ExpressionStatement, FunctionStatement, GroupExpression, IfStatement, LiteralExpression,
        ReturnStatement, Statement, UnaryExpression, VariableExpression, VariableStatement,
        WhileStatement,
    },
    error::{Error, ErrorKind},
    token::{Token, TokenKind},
};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_index: 0,
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();

        while self.index_in_bound() && !self.current_token_is_eof() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.current_token().kind {
            TokenKind::Fun => self.parse_function_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::For => self.parse_for_statement(),
            TokenKind::While => self.parse_while_statement(),
            TokenKind::OpenBrace => self.parse_block_statement(),
            TokenKind::Var => self.parse_var_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_function_statement(&mut self) -> Result<Statement, Error> {
        self.consume_token(TokenKind::Fun)?;
        let identifier = self.consume_token(TokenKind::Identifier)?;
        self.consume_token(TokenKind::OpenParen)?;
        let mut parameters = Vec::new();
        if !self.current_token_matches(&[TokenKind::CloseParen]) {
            loop {
                parameters.push(self.consume_token(TokenKind::Identifier)?);
                if self.current_token_matches(&[TokenKind::Comma]) {
                    self.consume_token(TokenKind::Comma)?;
                } else {
                    break;
                }
            }
        }
        self.consume_token(TokenKind::CloseParen)?;
        let block = self.parse_block_statement()?;

        Ok(Statement::Function(FunctionStatement::new(
            identifier, parameters, block,
        )))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, Error> {
        self.consume_token(TokenKind::If)?;
        let condition = self.parse_expression()?;
        let then_block = self.parse_block_statement()?;
        if self.current_token_matches(&[TokenKind::Else]) {
            self.consume_token(TokenKind::Else)?;
            if self.current_token_matches(&[TokenKind::If]) {
                let else_block = self.parse_if_statement()?;
                Ok(Statement::If(IfStatement::new(
                    condition,
                    then_block,
                    Some(else_block),
                )))
            } else {
                let else_block = self.parse_block_statement()?;
                Ok(Statement::If(IfStatement::new(
                    condition,
                    then_block,
                    Some(else_block),
                )))
            }
        } else {
            Ok(Statement::If(IfStatement::new(condition, then_block, None)))
        }
    }

    fn parse_for_statement(&mut self) -> Result<Statement, Error> {
        self.consume_token(TokenKind::For)?;
        let variable_initialization = self.parse_var_statement()?;
        self.consume_token(TokenKind::Comma)?;
        let condition = self.parse_expression()?;
        self.consume_token(TokenKind::Comma)?;
        let step_expression = self.parse_expression()?;
        let do_block = self.parse_block_statement()?;
        let while_statement = Statement::While(WhileStatement::new(
            condition,
            Statement::Block(BlockStatement::new(vec![
                do_block,
                Statement::Expression(ExpressionStatement::new(step_expression)),
            ])),
        ));

        Ok(Statement::Block(BlockStatement::new(vec![
            variable_initialization,
            while_statement,
        ])))
    }

    fn parse_while_statement(&mut self) -> Result<Statement, Error> {
        self.consume_token(TokenKind::While)?;
        let condition = self.parse_expression()?;
        let do_block = self.parse_block_statement()?;

        Ok(Statement::While(WhileStatement::new(condition, do_block)))
    }

    fn parse_block_statement(&mut self) -> Result<Statement, Error> {
        self.consume_token(TokenKind::OpenBrace)?;
        let mut statements = Vec::new();
        while !self.current_token_matches(&[TokenKind::CloseBrace]) && !self.current_token_is_eof()
        {
            statements.push(self.parse_statement()?);
        }
        self.consume_token(TokenKind::CloseBrace)?;

        Ok(Statement::Block(BlockStatement::new(statements)))
    }

    fn parse_var_statement(&mut self) -> Result<Statement, Error> {
        self.consume_token(TokenKind::Var)?;
        let identifier = self.consume_token(TokenKind::Identifier)?;
        let mut initializer = None;
        if self.current_token_matches(&[TokenKind::Assign]) {
            self.consume_token(TokenKind::Assign)?;
            initializer = Some(self.parse_expression()?);
        }

        Ok(Statement::Variable(VariableStatement::new(
            identifier,
            initializer,
        )))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        self.consume_token(TokenKind::Return)?;
        let expression = self.parse_expression()?;
        Ok(Statement::Return(ReturnStatement::new(expression)))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let expression = self.parse_expression()?;
        Ok(Statement::Expression(ExpressionStatement::new(expression)))
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> Result<Expression, Error> {
        let assign_token = self.current_token();
        let expression = self.parse_binary_expression()?;

        if self.current_token_matches(&[TokenKind::Assign]) {
            self.consume_token(TokenKind::Assign)?;
            let initializer = self.parse_assignment_expression()?;
            if let Expression::Variable(expression) = expression {
                return Ok(Expression::Assignment(AssignmentExpression::new(
                    expression.identifier,
                    initializer,
                )));
            } else {
                return Err(Error::new(
                    ErrorKind::ParserError,
                    "Invalid assignment target".to_string(),
                    Some(assign_token.position),
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
            let right = self.parse_call_expression()?;
            Ok(Expression::Unary(UnaryExpression::new(operator, right)))
        } else {
            self.parse_call_expression()
        }
    }

    fn parse_call_expression(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_primary_expression()?;

        while self.current_token_matches(&[TokenKind::OpenParen]) {
            self.consume_token(TokenKind::OpenParen)?;
            let mut arguments = Vec::new();

            if !self.current_token_matches(&[TokenKind::CloseParen]) {
                loop {
                    arguments.push(self.parse_expression()?);
                    if self.current_token_matches(&[TokenKind::Comma]) {
                        self.consume_token(TokenKind::Comma)?;
                    } else {
                        break;
                    }
                }
            }

            self.consume_token(TokenKind::CloseParen)?;
            expression = Expression::Call(CallExpression::new(expression, arguments));
        }

        Ok(expression)
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
                self.next_token(),
            )))
        } else if self.current_token_matches(&[TokenKind::Identifier]) {
            Ok(Expression::Variable(VariableExpression::new(
                self.next_token(),
            )))
        } else if self.current_token_matches(&[TokenKind::OpenParen]) {
            self.consume_token(TokenKind::OpenParen)?;
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

    fn index_in_bound(&self) -> bool {
        self.current_index < self.tokens.len()
    }

    fn current_token_is_eof(&self) -> bool {
        self.current_token().kind == TokenKind::EOF
    }

    fn current_token(&self) -> Token {
        self.tokens[self.current_index].clone()
    }

    fn advance_current_index(&mut self) {
        self.current_index += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = self.current_token();
        self.advance_current_index();
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
                "Unexpected token; expected `{}`, got `{}`",
                kind,
                self.current_token().lexeme,
            )))
        }
    }

    fn generate_error(&self, message: String) -> Error {
        Error::new(
            ErrorKind::ParserError,
            message,
            Some(self.current_token().position),
        )
    }
}
