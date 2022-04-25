use pm::make_expressions;

make_expressions!();

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
