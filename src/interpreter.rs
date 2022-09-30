use std::ops::Not;

use crate::{
    expression::{BinaryExpression, Expression, LiteralExpression, UnaryExpression},
    statement::Statement,
    token::Token,
};

pub fn interpret_program(statements: Vec<Statement>) -> Result<(), RuntimeError> {
    for statement in statements {
        let expression = statement.evaluate()?;
        println!("evaluated: {expression:?}");
    }

    Ok(())
}

pub trait Interpret {
    fn evaluate(&self) -> Result<Value, RuntimeError>;
}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn to_number(&self) -> f64 {
        match self {
            Self::Number(inner) => *inner,
            _ => todo!(),
        }
    }
}

impl Interpret for Statement {
    fn evaluate(&self) -> Result<Value, RuntimeError> {
        match self {
            Self::Expression(inner) | Self::Print(inner) => inner.evaluate(),
        }
    }
}

impl Interpret for Expression {
    fn evaluate(&self) -> Result<Value, RuntimeError> {
        let self_variant: &dyn Interpret = match self {
            Self::Literal(inner) => inner,
            Self::Binary(inner) => inner.as_ref(),
            Self::Grouping(inner) => inner.as_ref(),
            Self::Unary(inner) => inner.as_ref(),
        };

        Interpret::evaluate(self_variant)
    }
}

impl Interpret for LiteralExpression {
    fn evaluate(&self) -> Result<Value, RuntimeError> {
        let value = match &self.value {
            Token::String(inner) => Value::String(inner.to_owned()),
            Token::Number(inner) => Value::Number(*inner),
            Token::Bool(inner) => Value::Bool(*inner),
            Token::Nil => Value::Nil,
            // todo
            Token::Identifier(_inner) => todo!("2"),
            _ => todo!("1"),
        };

        Ok(value)
    }
}

impl Interpret for BinaryExpression {
    fn evaluate(&self) -> Result<Value, RuntimeError> {
        let Self {
            left,
            operator,
            right,
        } = self;

        let (lhs, rhs) = (left.evaluate()?, right.evaluate()?);

        let value = match operator {
            Token::Minus => {
                check_number_operands(operator, &lhs, &rhs)?;
                Value::Number(lhs.to_number() - rhs.to_number())
            }
            Token::Slash => {
                check_number_operands(operator, &lhs, &rhs)?;
                Value::Number(lhs.to_number() / rhs.to_number())
            }
            Token::Star => {
                check_number_operands(operator, &lhs, &rhs)?;
                Value::Number(lhs.to_number() * rhs.to_number())
            }
            Token::Greater => {
                check_number_operands(operator, &lhs, &rhs)?;
                Value::Bool(lhs.to_number() > rhs.to_number())
            }
            Token::GreaterEqual => {
                check_number_operands(operator, &lhs, &rhs)?;
                Value::Bool(lhs.to_number() >= rhs.to_number())
            }
            Token::Less => {
                check_number_operands(operator, &lhs, &rhs)?;
                Value::Bool(lhs.to_number() < rhs.to_number())
            }
            Token::LessEqual => {
                check_number_operands(operator, &lhs, &rhs)?;
                Value::Bool(lhs.to_number() <= rhs.to_number())
            }
            Token::BangEqual => Value::Bool(lhs.to_number() != rhs.to_number()),
            Token::EqualEqual => Value::Bool(lhs == rhs),
            Token::Plus => {
                match (lhs, rhs) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                    (Value::String(left), Value::String(right)) => Value::String(left + &right),
                    // throw new RuntimeError(expr.operator,
                    // "Operands must be two numbers or two strings.");
                    _ => todo!(),
                }
            }
            _ => todo!(),
        };

        Ok(value)
    }
}

impl Interpret for UnaryExpression {
    fn evaluate(&self) -> Result<Value, RuntimeError> {
        let Self {
            operator,
            expression,
        } = self;

        let value = expression.evaluate()?;

        let value = match operator {
            Token::Bang => Value::Bool(is_truthy(value).not()),
            Token::Minus => {
                if matches!(value, Value::Number(_)).not() {
                    return Err(RuntimeError::UnaryExpressionExpectedNumber(
                        operator.clone(),
                        value,
                    ));
                }
                Value::Number(value.to_number())
            }
            _ => unreachable!(),
        };

        Ok(value)
    }
}

fn is_truthy(value: Value) -> bool {
    matches!(value, Value::Nil | Value::Bool(false)).not()
}

//  extends RuntimeException {
//   final Token token;
//   RuntimeError(Token token, String message) {
//     super(message);
//     this.token = token;
//   }
// }

//  private void checkNumberOperands(Token operator, Object left, Object right) {
//    if (left instanceof Double && right instanceof Double) return;
//    throw new RuntimeError(operator, "Operands must be numbers.");
//  }
fn check_number_operands(
    operator: &Token,
    lhs_val: &Value,
    rhs_val: &Value,
) -> Result<(), RuntimeError> {
    let error = match (lhs_val, rhs_val) {
        (Value::Number(_), Value::Number(_)) => return Ok(()),
        (Value::Number(_), _) => {
            RuntimeError::BinaryExpressionExpectedNumberAtRight(operator.clone(), rhs_val.clone())
        }
        (_, Value::Number(_)) => {
            RuntimeError::BinaryExpressionExpectedNumberAtLeft(operator.clone(), lhs_val.clone())
        }
        _ => {
            RuntimeError::BinaryExpressionExpectedNumberBothSides(
                operator.clone(),
                lhs_val.clone(),
                rhs_val.clone(),
            )
        }
    };

    Err(error)
}

#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error("Expected number after unary operator '{0}'")]
    UnaryExpressionExpectedNumber(Token, Value),
    #[error("oi")]
    BinaryExpressionExpectedNumberAtRight(Token, Value),
    #[error("oi")]
    BinaryExpressionExpectedNumberAtLeft(Token, Value),
    #[error("oi")]
    BinaryExpressionExpectedNumberBothSides(Token, Value, Value),
}
