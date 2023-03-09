use super::{object::Object, token::Token};

#[derive(Debug, Clone)]
pub(crate) enum Statement {
    If(IfStatement),
    While(WhileStatement),
    Block(BlockStatement),
    Variable(VariableStatement),
    Print(PrintStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug, Clone)]
pub(crate) struct WhileStatement {
    pub(crate) condition: Expression,
    pub(crate) do_block: Box<Statement>,
}

impl WhileStatement {
    pub(crate) fn new(condition: Expression, do_block: Statement) -> Self {
        Self {
            condition,
            do_block: Box::new(do_block),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IfStatement {
    pub(crate) condition: Expression,
    pub(crate) then_block: Box<Statement>,
    pub(crate) else_block: Box<Option<Statement>>,
}

impl IfStatement {
    pub(crate) fn new(
        condition: Expression,
        then_block: Statement,
        else_block: Option<Statement>,
    ) -> Self {
        Self {
            condition,
            then_block: Box::new(then_block),
            else_block: Box::new(else_block),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub(crate) struct PrintStatement {
    pub(crate) expression: Expression,
}

impl PrintStatement {
    pub(crate) fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExpressionStatement {
    pub(crate) expression: Expression,
}

impl ExpressionStatement {
    pub(crate) fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Expression {
    Assignment(AssignmentExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Group(GroupExpression),
    Literal(LiteralExpression),
    Variable(VariableExpression),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub(crate) struct LiteralExpression {
    pub(crate) value: Option<Object>,
}

impl LiteralExpression {
    pub(crate) fn new(value: Option<Object>) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VariableExpression {
    pub(crate) identifier: Token,
}

impl VariableExpression {
    pub(crate) fn new(identifier: Token) -> Self {
        Self { identifier }
    }
}
