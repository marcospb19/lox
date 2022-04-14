use std::slice;

use crate::{
    expression::{
        BinaryExpression, ExpressionBox, GroupingExpression, LiteralExpression, UnaryExpression,
    },
    token::{Token, Token::*},
};

pub struct Parser<'a> {
    tokens: slice::Iter<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens.iter(),
        }
    }

    pub fn parse(mut self) -> ExpressionBox {
        self.expression()
    }

    fn expression(&mut self) -> ExpressionBox {
        self.equality()
    }

    /// Advance one token if matched by given slice.
    fn matches(&mut self, slice: &[Token]) -> Option<Token> {
        let peek = || self.tokens.clone().next();

        if let Some(peeked) = peek() {
            if slice.contains(peeked) {
                // Advance
                return self.tokens.next().cloned();
            }
        }

        None
    }

    // Helper function to build binary expression parser steps in this form:
    // step  →   next_step ( TOKENS next_step )* ;
    //
    // Form is satisfied for the following grammar rules:
    // equality       → comparison ( ( "!=" | "=="             ) comparison )* ;
    // comparison     → term       ( ( ">" | ">=" | "<" | "<=" ) term       )* ;
    // term           → factor     ( ( "-" | "+"               ) factor     )* ;
    // factor         → unary      ( ( "/" | "*"               ) unary      )* ;
    fn binary_expression_parser_step<F>(&mut self, next_step: F, tokens: &[Token]) -> ExpressionBox
    where
        F: Fn(&mut Self) -> ExpressionBox,
    {
        let mut expr = next_step(self);

        while let Some(operator) = self.matches(tokens) {
            let right = next_step(self);
            expr = box BinaryExpression::new(expr, operator, right);
        }

        expr
    }

    fn equality(&mut self) -> ExpressionBox {
        self.binary_expression_parser_step(Self::comparison, &[BangEqual, EqualEqual])
    }

    fn comparison(&mut self) -> ExpressionBox {
        self.binary_expression_parser_step(Self::term, &[Greater, GreaterEqual, Less, LessEqual])
    }

    fn term(&mut self) -> ExpressionBox {
        self.binary_expression_parser_step(Self::factor, &[Minus, Plus])
    }

    fn factor(&mut self) -> ExpressionBox {
        self.binary_expression_parser_step(Self::unary, &[Slash, Star])
    }

    fn unary(&mut self) -> ExpressionBox {
        if let Some(operator) = self.matches(&[Bang, Minus]) {
            let expression = self.unary();
            box UnaryExpression::new(operator, expression)
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> ExpressionBox {
        let token = self
            .tokens
            .next()
            .unwrap_or_else(|| panic!("unexpected EOF"));

        if token.is_literal() {
            box LiteralExpression::new(token.clone())
        } else if token == &Token::LeftParen {
            let expr = self.expression();
            self.matches(&[RightParen]);
            box GroupingExpression::new(expr)
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

        let ast = Parser::new(&tokens).parse();
        assert_eq!("(== (< (- 1 (group (* 2 3))) 4) false)", ast.to_string());
    }
}
