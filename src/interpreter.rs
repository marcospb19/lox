use std::{collections::HashMap, ops::Not};

use crate::{
    expression::{BinaryExpression, Expression, LiteralExpression, UnaryExpression},
    statement::Statement,
    token::Token,
};

pub fn interpret_program(
    statements: Vec<Statement>,
    environment: &mut Environment,
) -> Result<(), RuntimeError> {
    for statement in statements {
        let expression = statement.evaluate(environment)?;
        println!("evaluated: {expression:?}");
    }

    Ok(())
}

#[derive(Default, Debug)]
pub struct Environment {
    variables: HashMap<String, Option<Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    fn define(&mut self, identifier: String) {
        self.variables.insert(identifier, None);
    }

    fn set_value(&mut self, identifier: String, value: Value) {
        self.variables.insert(identifier, Some(value));
    }

    fn get_value(&self, identifier: &str) -> Option<Option<Value>> {
        self.variables.get(identifier).cloned()
    }
}

pub trait Interpret {
    fn evaluate(&self, environment: &mut Environment) -> Result<Value, RuntimeError>;
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
    fn evaluate(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        match self {
            Self::Expression(inner) => inner.evaluate(environment),
            Self::Print(inner) => {
                print!("print ");
                inner.evaluate(environment)
            }
            Statement::VariableDeclaration(identifier, initial_value_expression) => {
                match initial_value_expression {
                    Some(expression) => {
                        let value = expression.evaluate(environment)?;
                        environment.set_value(identifier.clone(), value);
                    }
                    None => {
                        environment.define(identifier.clone());
                    }
                };

                Ok(Value::Nil)
            }
        }
    }
}

impl Interpret for Expression {
    fn evaluate(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let self_variant: &dyn Interpret = match self {
            Self::Literal(inner) => inner,
            Self::Binary(inner) => inner.as_ref(),
            Self::Grouping(inner) => inner.as_ref(),
            Self::Unary(inner) => inner.as_ref(),
            Self::VariableReference(identifier) => {
                return match environment.get_value(identifier) {
                    Some(Some(value)) => Ok(value),
                    Some(None) => Err(RuntimeError::UninitializedVariable(identifier.clone())),
                    None => Err(RuntimeError::UndefinedVariable(identifier.clone())),
                };
            }
        };

        Interpret::evaluate(self_variant, environment)
    }
}

impl Interpret for LiteralExpression {
    fn evaluate(&self, _environment: &mut Environment) -> Result<Value, RuntimeError> {
        let value = match &self.value {
            Token::String(inner) => Value::String(inner.to_owned()),
            Token::Number(inner) => Value::Number(*inner),
            Token::Bool(inner) => Value::Bool(*inner),
            Token::Nil => Value::Nil,
            _ => unreachable!(),
        };

        Ok(value)
    }
}

impl Interpret for BinaryExpression {
    fn evaluate(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let Self {
            left,
            operator,
            right,
        } = self;

        let (lhs, rhs) = (left.evaluate(environment)?, right.evaluate(environment)?);

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
    fn evaluate(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        let Self {
            operator,
            expression,
        } = self;

        let value = expression.evaluate(environment)?;

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
    #[error("Expected number after binary operator '{0}', found {1:?} instead.")]
    BinaryExpressionExpectedNumberAtRight(Token, Value),
    #[error("Expected number before binary operator '{0}', found {1:?} instead.")]
    BinaryExpressionExpectedNumberAtLeft(Token, Value),
    #[error("Operator '{0}' should be surrounded by numbers, found {1:?} and {2:?} instead.")]
    BinaryExpressionExpectedNumberBothSides(Token, Value, Value),
    #[error("variable '{0}' is not defined")]
    UndefinedVariable(String),
    #[error("variable '{0}' is defined but uninitialized")]
    UninitializedVariable(String),
}
