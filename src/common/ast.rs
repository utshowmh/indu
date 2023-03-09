use super::{object::Object, token::Token};

#[derive(Debug)]
pub(crate) enum Statement {
    Block(BlockStatement),
    Variable(VariableStatement),
    Print(PrintStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug)]
pub(crate) struct BlockStatement {
    pub(crate) statements: Box<Vec<Statement>>,
}

impl BlockStatement {
    pub(crate) fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements: Box::new(statements),
        }
    }
}

#[derive(Debug)]
pub(crate) struct VariableStatement {
    pub(crate) identifier: Token,
    pub(crate) initializer: Option<Expression>,
}

impl VariableStatement {
    pub(crate) fn new(identifier: Token, initializer: Option<Expression>) -> Self {
        Self {
            identifier,
            initializer,
        }
    }
}

#[derive(Debug)]
pub(crate) struct PrintStatement {
    pub(crate) expression: Expression,
}

impl PrintStatement {
    pub(crate) fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub(crate) struct ExpressionStatement {
    pub(crate) expression: Expression,
}

impl ExpressionStatement {
    pub(crate) fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug)]
pub(crate) enum Expression {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Group(GroupExpression),
    Literal(LiteralExpression),
    Variable(VariableExpression),
}

#[derive(Debug)]
pub(crate) struct AssignmentExpression {
    pub(crate) identifier: Token,
    pub(crate) initializer: Box<Expression>,
}

impl AssignmentExpression {
    pub(crate) fn new(identifier: Token, initializer: Expression) -> Self {
        Self {
            identifier,
            initializer: Box::new(initializer),
        }
    }
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
pub(crate) struct GroupExpression {
    pub(crate) child: Box<Expression>,
}

impl GroupExpression {
    pub(crate) fn new(child: Expression) -> Self {
        Self {
            child: Box::new(child),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LiteralExpression {
    pub(crate) value: Option<Object>,
}

impl LiteralExpression {
    pub(crate) fn new(value: Option<Object>) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub(crate) struct VariableExpression {
    pub(crate) identifier: Token,
}

impl VariableExpression {
    pub(crate) fn new(identifier: Token) -> Self {
        Self { identifier }
    }
}
