use crate::token::Token;

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpression),
    Binary(Box<BinaryExpression>),
    Grouping(Box<Expression>),
    Unary(Box<UnaryExpression>),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

impl BinaryExpression {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub value: Token,
}

impl LiteralExpression {
    pub fn new(value: Token) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub expression: Expression,
}

impl UnaryExpression {
    pub fn new(operator: Token, expression: Expression) -> Self {
        Self {
            operator,
            expression,
        }
    }
}
