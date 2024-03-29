use crate::common::{
    ast::{
        AssignmentExpression, BinaryExpression, BlockStatement, CallExpression, ElseStatement,
        Expression, ExpressionStatement, FunctionStatement, GroupExpression, IfStatement,
        LiteralExpression, PrintStatement, Program, ReturnStatement, Statement, UnaryExpression,
        VariableExpression, VariableStatement, WhileStatement,
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

    pub(crate) fn parse(&mut self) -> Result<Program, Error> {
        let mut statements = Vec::new();

        while self.index_in_bound() && !self.current_token_is_eof() {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.current_token().kind {
            TokenKind::Fun => Ok(Statement::Function(self.parse_function_statement()?)),
            TokenKind::If => Ok(Statement::If(self.parse_if_statement()?)),
            TokenKind::For => Ok(Statement::Block(self.parse_for_statement()?)),
            TokenKind::While => Ok(Statement::While(self.parse_while_statement()?)),
            TokenKind::OpenBrace => Ok(Statement::Block(self.parse_block_statement()?)),
            TokenKind::Var => Ok(Statement::Variable(self.parse_var_statement()?)),
            TokenKind::Return => Ok(Statement::Return(self.parse_return_statement()?)),
            TokenKind::Print => Ok(Statement::Print(self.parse_print_statement()?)),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?)),
        }
    }

    fn parse_function_statement(&mut self) -> Result<FunctionStatement, Error> {
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

        Ok(FunctionStatement::new(identifier, parameters, block))
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement, Error> {
        self.consume_token(TokenKind::If)?;
        let condition = self.parse_expression()?;
        let then_branch = self.parse_block_statement()?;
        if self.current_token_matches(&[TokenKind::Else]) {
            self.consume_token(TokenKind::Else)?;
            if self.current_token_matches(&[TokenKind::If]) {
                let else_branch = self.parse_if_statement()?;
                Ok(IfStatement::new(
                    condition,
                    then_branch,
                    Some(ElseStatement::If(else_branch)),
                ))
            } else {
                let else_branch = self.parse_block_statement()?;
                Ok(IfStatement::new(
                    condition,
                    then_branch,
                    Some(ElseStatement::Block(else_branch)),
                ))
            }
        } else {
            Ok(IfStatement::new(condition, then_branch, None))
        }
    }

    fn parse_for_statement(&mut self) -> Result<BlockStatement, Error> {
        self.consume_token(TokenKind::For)?;
        let variable_initialization = self.parse_var_statement()?;
        self.consume_token(TokenKind::Comma)?;
        let condition = self.parse_comparison_expression()?;
        self.consume_token(TokenKind::Comma)?;
        let step_expression = self.parse_assignment_expression()?;
        let do_block = self.parse_block_statement()?;
        let while_statement = Statement::While(WhileStatement::new(
            condition,
            BlockStatement::new(vec![
                Statement::Block(do_block),
                Statement::Expression(ExpressionStatement::new(step_expression)),
            ]),
        ));

        Ok(BlockStatement::new(vec![
            Statement::Variable(variable_initialization),
            while_statement,
        ]))
    }

    fn parse_while_statement(&mut self) -> Result<WhileStatement, Error> {
        self.consume_token(TokenKind::While)?;
        let condition = self.parse_expression()?;
        let do_block = self.parse_block_statement()?;

        Ok(WhileStatement::new(condition, do_block))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, Error> {
        self.consume_token(TokenKind::OpenBrace)?;
        let mut statements = Vec::new();
        while !self.current_token_matches(&[TokenKind::CloseBrace]) && !self.current_token_is_eof()
        {
            statements.push(self.parse_statement()?);
        }
        self.consume_token(TokenKind::CloseBrace)?;

        Ok(BlockStatement::new(statements))
    }

    fn parse_var_statement(&mut self) -> Result<VariableStatement, Error> {
        self.consume_token(TokenKind::Var)?;
        let identifier = self.consume_token(TokenKind::Identifier)?;
        self.consume_token(TokenKind::Assign)?;
        let initializer = self.parse_expression()?;

        Ok(VariableStatement::new(identifier, initializer))
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, Error> {
        self.consume_token(TokenKind::Return)?;
        let expression = self.parse_expression()?;
        Ok(ReturnStatement::new(expression))
    }

    fn parse_print_statement(&mut self) -> Result<PrintStatement, Error> {
        self.consume_token(TokenKind::Print)?;
        let expression = self.parse_expression()?;
        Ok(PrintStatement::new(expression))
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement, Error> {
        let expression = self.parse_expression()?;
        Ok(ExpressionStatement::new(expression))
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
                    ErrorKind::Parser,
                    format!(
                        "Invalid assignment target. Can not assign to '{}'.",
                        assign_token.lexeme
                    ),
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
        let mut left = self.parse_multiplicative_expression()?;
        while self.current_token_matches(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.next_token();
            let right = self.parse_multiplicative_expression()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn parse_multiplicative_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_unary_expression()?;
        while self.current_token_matches(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.next_token();
            let right = self.parse_unary_expression()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, Error> {
        if self.current_token_matches(&[TokenKind::Plus, TokenKind::Minus, TokenKind::Not]) {
            let operator = self.next_token();
            let right = self.parse_unary_expression()?;
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
                "Unexpected token. Expected 'literal', found '{}'.",
                self.current_token().lexeme
            )))
        }
    }

    fn index_in_bound(&self) -> bool {
        self.current_index < self.tokens.len()
    }

    fn current_token_is_eof(&self) -> bool {
        self.current_token().kind == TokenKind::Eof
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
                "Unexpected token. Expected '{}', found '{}'.",
                kind,
                self.current_token().kind,
            )))
        }
    }

    fn generate_error(&self, message: String) -> Error {
        Error::new(
            ErrorKind::Parser,
            message,
            Some(self.current_token().position),
        )
    }
}
