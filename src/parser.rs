use std::slice;

use crate::{
    expression::{BinaryExpression, Expression, LiteralExpression, UnaryExpression},
    token::{Token, Token::*},
};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens_iter: slice::Iter<'a, Token>,
    errors: Vec<ParseError>,
}

#[derive(Debug)]
pub enum ParseError {
    UnclosedGrouping,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens_iter: tokens.iter(),
            errors: vec![],
        }
    }

    pub fn parse(mut self) -> Result<Expression, Vec<ParseError>> {
        self.expression().ok_or(self.errors)
    }

    fn advance(&mut self) -> Option<&Token> {
        self.tokens_iter.next()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens_iter.clone().next()
    }

    /// Advance one token if matched by given slice.
    fn matches(&mut self, slice: &[Token]) -> Option<Token> {
        if let Some(peeked) = self.peek() {
            if slice.contains(peeked) {
                return self.advance().cloned();
            }
        }

        None
    }

    #[allow(unused)]
    fn synchronize(&mut self) {
        loop {
            let skipped = self.advance();

            // If has reached the end of the tokens_iter, or an semicolon
            if let None | Some(Token::Semicolon) = skipped {
                return;
            }

            if let Some(
                Token::Class
                | Token::Fun
                | Token::Var
                | Token::For
                | Token::If
                | Token::While
                | Token::Print
                | Token::Return,
            ) = self.peek()
            {
                return;
            }

            self.advance();
        }
    }

    fn expression(&mut self) -> Option<Expression> {
        self.equality()
    }

    // Helper function to build binary expression parser steps in this form:
    // step  →   next_step ( TOKENS next_step )* ;
    //
    // Form is satisfied for the following grammar rules:
    // equality       → comparison ( ( "!=" | "=="             ) comparison )* ;
    // comparison     → term       ( ( ">" | ">=" | "<" | "<=" ) term       )* ;
    // term           → factor     ( ( "-" | "+"               ) factor     )* ;
    // factor         → unary      ( ( "/" | "*"               ) unary      )* ;
    fn binary_expression_parser_step<F>(
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
        self.binary_expression_parser_step(Self::comparison, &[BangEqual, EqualEqual])
    }

    fn comparison(&mut self) -> Option<Expression> {
        self.binary_expression_parser_step(Self::term, &[Greater, GreaterEqual, Less, LessEqual])
    }

    fn term(&mut self) -> Option<Expression> {
        self.binary_expression_parser_step(Self::factor, &[Minus, Plus])
    }

    fn factor(&mut self) -> Option<Expression> {
        self.binary_expression_parser_step(Self::unary, &[Slash, Star])
    }

    fn unary(&mut self) -> Option<Expression> {
        if let Some(operator) = self.matches(&[Bang, Minus]) {
            let expression = self.unary()?;
            Some(Expression::Unary(box UnaryExpression::new(
                operator, expression,
            )))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Option<Expression> {
        let token = self
            .tokens_iter
            .next()
            .unwrap_or_else(|| panic!("unexpected EOF"));

        if token.is_literal() {
            Some(Expression::Literal(LiteralExpression::new(token.clone())))
        } else if token == &Token::LeftParen {
            // Eat next expression
            let expr = self.expression()?;

            // We expect the next token to be a closing parenthesis
            // If it's not, enter recovery mode that jumps to the next statement.
            match self.matches(&[RightParen]) {
                Some(_) => Some(Expression::Grouping(box expr)),
                None => {
                    self.errors.push(ParseError::UnclosedGrouping);
                    None
                }
            }
        } else {
            unreachable!("token {token:?} was unexpected");
        }
    }
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

        let ast = Parser::new(&tokens).parse().unwrap();
        assert_eq!("(== (< (- 1 (group (* 2 3))) 4) false)", ast.to_string());
    }
}
