//! This Token parser is an Ast builder
//!
//! Grammar:
//!
//! ```txt
//!   program        → statement* EOF ;
//!
//!   statement      → exprStmt
//!                    | printStmt ;
//!
//!   exprStmt       → expression ";" ;
//!   printStmt      → "print" expression ";" ;
//!
//!   expression     → equality ;
//!   equality       → comparison ( ( "!=" | "==" ) comparison )* ;
//!   comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//!   term           → factor ( ( "-" | "+" ) factor )* ;
//!   factor         → unary ( ( "/" | "*" ) unary )* ;
//!   unary          → ( "!" | "-" ) unary
//!                    | literal ;
//!   literal        → NUMBER | STRING | "true" | "false" | "nil"
//! ```

use std::slice;

use thiserror::Error;

use crate::{
    expression::{BinaryExpression, Expression, LiteralExpression, UnaryExpression},
    statement::Statement,
    token::{Token, Token::*},
    ParserErrorReporter,
};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens_iter: slice::Iter<'a, Token>,
    error_reporter: ParserErrorReporter,
}

impl<'a> Parser<'a> {
    /// Creates a new token parser.
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens_iter: tokens.iter(),
            error_reporter: ParserErrorReporter::new(),
        }
    }

    /// Tries to parse all tokens, may fail with a list of errors.
    pub fn try_parse(mut self) -> Result<Vec<Statement>, ParserErrorReporter> {
        let mut statements = vec![];

        while self.peek().is_some() {
            let parsed_statement = self.parse_statement();
            match parsed_statement {
                Some(statement) => statements.push(statement),
                None => self.synchronize_after_error(),
            }
        }

        match self.error_reporter.has_errors() {
            true => Err(self.error_reporter),
            false => Ok(statements),
        }
    }

    fn add_error(&mut self, error: ParserError) {
        self.error_reporter.add_parser_error(error);
    }

    fn advance_token(&mut self) -> Option<&Token> {
        self.tokens_iter.next()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens_iter.clone().next()
    }

    /// Advance one token if matched by given slice.
    fn matches(&mut self, slice: &[Token]) -> Option<Token> {
        if let Some(peeked) = self.peek() {
            if slice.contains(peeked) {
                return self.advance_token().cloned();
            }
        }

        None
    }

    pub fn parse_expression(&mut self) -> Option<Expression> {
        self.equality()
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.matches(&[Token::Print]).is_some() {
            self.parse_print_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_print_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression()?;

        match self.matches(&[Semicolon]) {
            Some(_) => Some(Statement::Print(expression)),
            None => {
                self.add_error(ParserError::UnterminatedStatement);
                None
            }
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression()?;

        match self.matches(&[Semicolon]) {
            Some(_) => Some(Statement::Expression(expression)),
            None => {
                self.add_error(ParserError::UnterminatedStatement);
                None
            }
        }
    }

    // Helper function to build binary expression parser steps in this form:
    // step  →   next_step ( TOKENS next_step )* ;
    //
    // Form is satisfied for the following grammar rules:
    // equality       → comparison ( ( "!=" | "=="             ) comparison )* ;
    // comparison     → term       ( ( ">" | ">=" | "<" | "<=" ) term       )* ;
    // term           → factor     ( ( "-" | "+"               ) factor     )* ;
    // factor         → unary      ( ( "/" | "*"               ) unary      )* ;
    fn parse_binary_expression_by<F>(
        &mut self,
        next_step: F,
        tokens: &[Token],
    ) -> Option<Expression>
    where
        F: Fn(&mut Self) -> Option<Expression>,
    {
        let mut expr = next_step(self)?;

        while let Some(operator) = self.matches(tokens) {
            let right = next_step(self)?;
            expr = Expression::Binary(box BinaryExpression::new(expr, operator, right));
        }

        Some(expr)
    }

    fn equality(&mut self) -> Option<Expression> {
        self.parse_binary_expression_by(Self::comparison, &[BangEqual, EqualEqual])
    }

    fn comparison(&mut self) -> Option<Expression> {
        self.parse_binary_expression_by(Self::term, &[Greater, GreaterEqual, Less, LessEqual])
    }

    fn term(&mut self) -> Option<Expression> {
        self.parse_binary_expression_by(Self::factor, &[Minus, Plus])
    }

    fn factor(&mut self) -> Option<Expression> {
        self.parse_binary_expression_by(Self::parse_unary_expression, &[Slash, Star])
    }

    fn parse_unary_expression(&mut self) -> Option<Expression> {
        if let Some(operator) = self.matches(&[Bang, Minus]) {
            let expression = self.parse_unary_expression()?;
            Some(Expression::Unary(box UnaryExpression::new(
                operator, expression,
            )))
        } else {
            self.parse_literal()
        }
    }

    fn parse_literal(&mut self) -> Option<Expression> {
        let token = match self.tokens_iter.next() {
            Some(token) => token,
            None => todo!("expected literal, found EOF"),
        };

        if token.is_literal() {
            Some(Expression::Literal(LiteralExpression::new(token.clone())))
        } else if token == &Token::LeftParen {
            // Eat next expression
            let expr = self.parse_expression()?;

            // We expect the next token to be a closing parenthesis
            // If it's not, enter recovery mode that jumps to the next statement.
            match self.matches(&[RightParen]) {
                Some(_) => Some(Expression::Grouping(box expr)),
                None => {
                    self.add_error(ParserError::UnclosedGrouping);
                    None
                }
            }
        } else {
            self.add_error(ParserError::ExpectedValidExpression(token.clone()));
            None
        }
    }

    fn synchronize_after_error(&mut self) {
        loop {
            let skipped = self.advance_token().cloned();
            let peeked = self.peek();

            // If has reached the end of the tokens_iter, or an semicolon
            if let None | Some(Token::Semicolon) = skipped {
                return;
            }
            if peeked.is_none() {
                return;
            }

            // If matches any token that are the start of an statement,
            // then consider this to be synchronized
            if peeked.map(Token::is_start_of_statement).unwrap_or(false) {
                return;
            }

            self.advance_token();
        }
    }
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unclosed delimiter, expected ')'")]
    UnclosedGrouping,
    #[error("Unterminated statement, expected ';'")]
    UnterminatedStatement,
    #[error("Expected valid expression, found {0:?}")]
    ExpectedValidExpression(Token),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scanner;

    #[test]
    fn test_parsing_expression() {
        let source_code = "1 - (2 * 3) < 4 == false";
        let scanner = Scanner::new(source_code);
        let tokens: Vec<_> = scanner.into_iter().map(|x| x.token_type).collect();

        let ast = Parser::new(&tokens).parse_expression().unwrap();
        assert_eq!("(== (< (- 1 (group (* 2 3))) 4) false)", ast.to_string());
    }
}
