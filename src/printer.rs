use std::fmt::{Display, Formatter, Result};

use crate::expr::*;

impl Display for BinaryExpression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Self {
            left,
            operator,
            right,
        } = self;
        write!(f, "({operator} {left} {right})")
    }
}

impl Display for GroupingExpression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Self { expression } = self;
        write!(f, "(group {expression})")
    }
}

impl Display for LiteralExpression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Self { value } = self;
        write!(f, "{value}")
    }
}

impl Display for UnaryExpression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Self {
            operator,
            expression,
        } = self;
        write!(f, "({operator} {expression})")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_ast_pretty_printing() {
        let expression = BinaryExpression::new(
            box UnaryExpression::new(
                TokenType::Minus,
                box LiteralExpression::new(TokenType::Number(123.0)),
            ),
            TokenType::Star,
            box GroupingExpression::new(box LiteralExpression::new(TokenType::Number(45.67))),
        );
        let expression = expression.to_string();

        assert_eq!(&expression, "(* (- 123) (group 45.67))");
    }
}
