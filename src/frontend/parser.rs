use crate::common::{
    ast::{
        BinaryExpression, Expression, LiteralExpression, ParenthesizedExpression, UnaryExpression,
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

    pub(crate) fn parse(&mut self) -> Result<Expression, Error> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_binary_expression()
    }

    fn parse_binary_expression(&mut self) -> Result<Expression, Error> {
        self.parse_equality_expression()
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
            TokenKind::Identifier,
        ]) {
            Ok(Expression::Literal(LiteralExpression::new(
                self.next_token().literal,
            )))
        } else if self.current_token_matches(&[TokenKind::OpenParen]) {
            self.advance_current_position();
            let child = self.parse_expression()?;
            self.consume_token(TokenKind::CloseParen)?;
            Ok(Expression::Parenthesized(ParenthesizedExpression::new(
                child,
            )))
        } else {
            Err(self.generate_error(format!(
                "Unexpected token `{:?}`",
                self.current_token().kind
            )))
        }
    }

    // fn current_token_is_eof(&self) -> bool {
    //     self.current_token().kind == TokenKind::EOF
    // }

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
                "Unexpected token`{:?}`, expected `{:?}`",
                self.current_token().kind,
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
