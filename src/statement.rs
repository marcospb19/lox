use crate::Expression;

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
}
