use super::{position::Position, token::Token};

pub(crate) type Program = Vec<Statement>;

#[derive(Debug, Clone)]
pub(crate) enum Statement {
    Function(FunctionStatement),
    If(IfStatement),
    While(WhileStatement),
    Block(BlockStatement),
    Variable(VariableStatement),
    Return(ReturnStatement),
    Print(PrintStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug, Clone)]
pub(crate) struct FunctionStatement {
    pub(crate) identifier: Token,
    pub(crate) parameters: Vec<Token>,
    pub(crate) block: BlockStatement,
}

impl FunctionStatement {
    pub(crate) fn new(identifier: Token, parameters: Vec<Token>, block: BlockStatement) -> Self {
        Self {
            identifier,
            parameters,
            block,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WhileStatement {
    pub(crate) condition: Expression,
    pub(crate) do_block: BlockStatement,
}

impl WhileStatement {
    pub(crate) fn new(condition: Expression, do_block: BlockStatement) -> Self {
        Self {
            condition,
            do_block,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ElseStatement {
    If(IfStatement),
    Block(BlockStatement),
}

#[derive(Debug, Clone)]
pub(crate) struct IfStatement {
    pub(crate) condition: Expression,
    pub(crate) then_branch: BlockStatement,
    pub(crate) else_branch: Box<Option<ElseStatement>>,
}

impl IfStatement {
    pub(crate) fn new(
        condition: Expression,
        then_branch: BlockStatement,
        else_branch: Option<ElseStatement>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch: Box::new(else_branch),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BlockStatement {
    pub(crate) statements: Vec<Statement>,
}

impl BlockStatement {
    pub(crate) fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VariableStatement {
    pub(crate) identifier: Token,
    pub(crate) initializer: Expression,
}

impl VariableStatement {
    pub(crate) fn new(identifier: Token, initializer: Expression) -> Self {
        Self {
            identifier,
            initializer,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ReturnStatement {
    pub(crate) expression: Expression,
}

impl ReturnStatement {
    pub(crate) fn new(expression: Expression) -> Self {
        Self { expression }
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
    Call(CallExpression),
    Literal(LiteralExpression),
    Variable(VariableExpression),
}

impl Expression {
    pub(crate) fn position(&self) -> Position {
        match self {
            Self::Assignment(expression) => expression.position(),
            Self::Binary(expression) => expression.position(),
            Self::Unary(expression) => expression.position(),
            Self::Group(expression) => expression.position(),
            Self::Call(expression) => expression.position(),
            Self::Literal(expression) => expression.position(),
            Self::Variable(expression) => expression.position(),
        }
    }
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

    pub(crate) fn position(&self) -> Position {
        self.identifier.position.clone()
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

    pub(crate) fn position(&self) -> Position {
        self.operator.position.clone()
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

    pub(crate) fn position(&self) -> Position {
        self.operator.position.clone()
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

    pub(crate) fn position(&self) -> Position {
        self.child.position()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CallExpression {
    pub(crate) callee: Box<Expression>,
    pub(crate) arguments: Vec<Expression>,
}

impl CallExpression {
    pub(crate) fn new(callee: Expression, arguments: Vec<Expression>) -> Self {
        Self {
            callee: Box::new(callee),
            arguments,
        }
    }

    pub(crate) fn position(&self) -> Position {
        self.callee.position()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LiteralExpression {
    pub(crate) value: Token,
}

impl LiteralExpression {
    pub(crate) fn new(value: Token) -> Self {
        Self { value }
    }

    pub(crate) fn position(&self) -> Position {
        self.value.position.clone()
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

    pub(crate) fn position(&self) -> Position {
        self.identifier.position.clone()
    }
}
