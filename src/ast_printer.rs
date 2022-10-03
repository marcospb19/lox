use std::fmt::{Display, Formatter, Result};

use crate::{
    expression::{BinaryExpression, Expression, LiteralExpression, UnaryExpression},
    statement::Statement,
};

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Expression(inner) => write!(f, "(expression {inner})"),
            Self::Print(inner) => write!(f, "(print {inner})"),
            Self::VariableDeclaration(identifier, initial_value) => {
                write!(f, "(var {identifier} (")?;
                if let Some(initial_value) = initial_value {
                    write!(f, "{initial_value}")?;
                }
                write!(f, "))")
            }
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let self_variant: &dyn Display = match self {
            Self::Grouping(inner) => return write!(f, "(group {inner})"),
            Self::VariableReference(identifier) => return write!(f, "(value_of {identifier})"),
            Self::Literal(inner) => inner,
            Self::Binary(inner) => inner,
            Self::Unary(inner) => inner,
        };
        self_variant.fmt(f)
    }
}

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
    use crate::token::Token;

    #[test]
    fn test_ast_pretty_printing() {
        let expression = BinaryExpression::new(
            Expression::Unary(box UnaryExpression::new(
                Token::Minus,
                Expression::Literal(LiteralExpression::new(Token::Number(123.0))),
            )),
            Token::Star,
            Expression::Grouping(box Expression::Literal(LiteralExpression::new(
                Token::Number(45.67),
            ))),
        );
        let expression = expression.to_string();

        assert_eq!(&expression, "(* (- 123) (group 45.67))");
    }
}
