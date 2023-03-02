use super::{object::Object, token::Token};

#[derive(Debug)]
pub(crate) enum Expression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Parenthesized(ParenthesizedExpression),
    Literal(LiteralExpression),
}

#[derive(Debug)]
pub(crate) struct BinaryExpression {
    pub(crate) left: Box<Expression>,
    pub(crate) operator: Token,
    pub(crate) right: Box<Expression>,
}

impl BinaryExpression {
    pub(crate) fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub(crate) struct UnaryExpression {
    pub(crate) operator: Token,
    pub(crate) right: Box<Expression>,
}

impl UnaryExpression {
    pub(crate) fn new(operator: Token, right: Expression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParenthesizedExpression {
    pub(crate) child: Box<Expression>,
}

impl ParenthesizedExpression {
    pub(crate) fn new(child: Expression) -> Self {
        Self {
            child: Box::new(child),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LiteralExpression {
    pub(crate) value: Object,
}

impl LiteralExpression {
    pub(crate) fn new(value: Object) -> Self {
        Self { value }
    }
}
