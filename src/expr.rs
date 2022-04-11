use std::fmt;

use crate::token::TokenType;

pub trait Expression: fmt::Display {}

impl Expression for BinaryExpression {}
impl Expression for GroupingExpression {}
impl Expression for LiteralExpression {}
impl Expression for UnaryExpression {}

pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub operator: TokenType,
    pub right: Box<dyn Expression>,
}

impl BinaryExpression {
    pub fn new(left: Box<dyn Expression>, operator: TokenType, right: Box<dyn Expression>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

pub struct GroupingExpression {
    pub expression: Box<dyn Expression>,
}

impl GroupingExpression {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

pub struct LiteralExpression {
    pub value: TokenType,
}

impl LiteralExpression {
    pub fn new(value: TokenType) -> Self {
        Self { value }
    }
}

pub struct UnaryExpression {
    pub operator: TokenType,
    pub expression: Box<dyn Expression>,
}

impl UnaryExpression {
    pub fn new(operator: TokenType, expression: Box<dyn Expression>) -> Self {
        Self {
            operator,
            expression,
        }
    }
}
