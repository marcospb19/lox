#![cfg_attr(not(test), allow(unused))]

use std::fmt;

use crate::token::Token;

pub trait Expression: fmt::Display {}

impl Expression for BinaryExpression {}
impl Expression for GroupingExpression {}
impl Expression for LiteralExpression {}
impl Expression for UnaryExpression {}

pub type ExpressionBox = Box<dyn Expression>;

pub struct BinaryExpression {
    pub left: ExpressionBox,
    pub operator: Token,
    pub right: ExpressionBox,
}

impl BinaryExpression {
    pub fn new(left: ExpressionBox, operator: Token, right: ExpressionBox) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

pub struct GroupingExpression {
    pub expression: ExpressionBox,
}

impl GroupingExpression {
    pub fn new(expression: ExpressionBox) -> Self {
        Self { expression }
    }
}

pub struct LiteralExpression {
    pub value: Token,
}

impl LiteralExpression {
    pub fn new(value: Token) -> Self {
        Self { value }
    }
}

pub struct UnaryExpression {
    pub operator: Token,
    pub expression: ExpressionBox,
}

impl UnaryExpression {
    pub fn new(operator: Token, expression: ExpressionBox) -> Self {
        Self {
            operator,
            expression,
        }
    }
}
