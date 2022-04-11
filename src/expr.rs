use crate::token::{Token, TokenType};

pub trait Expression {}

pub struct BinaryExpression {
    left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>
}

impl BinaryExpression {
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Self {
        Self { left, operator, right }
    }
}

pub struct GroupingExpression {
    expression: Box<dyn Expression>
}

impl GroupingExpression {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

pub struct LiteralExpression {
    value: TokenType
}

impl LiteralExpression {
    pub fn new(value: TokenType) -> Self {
        Self { value }
    }
}

pub struct UnaryExpression {
    operator: Token, expression: Box<dyn Expression>
}

impl UnaryExpression {
    pub fn new(operator: Token, expression: Box<dyn Expression>) -> Self {
        Self { operator, expression }
    }
}

