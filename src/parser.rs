#![allow(unused)]

use std::slice;

use crate::{
    expr::{
        BinaryExpression, ExpressionBox, GroupingExpression, LiteralExpression, UnaryExpression,
    },
    token::Token,
};

pub struct Parser<'a> {
    tokens: slice::Iter<'a, Token>,
}

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
                 | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
                 | "(" expression ")" ;
*/

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

    fn advance(&mut self) -> Option<&Token> {
        self.tokens.next()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.clone().next()
    }

    fn matches(&mut self, f: impl Fn(&Token) -> bool) -> Option<Token> {
        if let Some(peeked) = self.peek() {
            if f(peeked) {
                return self.advance().cloned();
            }
        }

        None
    }

    fn equality(&mut self) -> ExpressionBox {
        use Token::*;

        let mut expr = self.comparison();

        while let Some(operator) = self.matches(|x| matches!(x, BangEqual | EqualEqual)) {
            let right = self.comparison();
            expr = box BinaryExpression::new(expr, operator, right);
        }

        expr
    }

    fn comparison(&mut self) -> ExpressionBox {
        use Token::*;

        let mut expr = self.term();

        while let Some(operator) =
            self.matches(|x| matches!(x, Greater | GreaterEqual | Less | LessEqual))
        {
            let right = self.term();
            expr = box BinaryExpression::new(expr, operator, right);
        }

        expr
    }

    fn term(&mut self) -> ExpressionBox {
        use Token::*;

        let mut expr = self.factor();

        while let Some(operator) = self.matches(|x| matches!(x, Minus | Plus)) {
            let right = self.factor();
            expr = box BinaryExpression::new(expr, operator, right);
        }

        expr
    }

    fn factor(&mut self) -> ExpressionBox {
        use Token::*;

        let mut expr = self.unary();

        while let Some(operator) = self.matches(|x| matches!(x, Slash | Star)) {
            let right = self.factor();
            expr = box BinaryExpression::new(expr, operator, right);
        }

        expr
    }

    fn unary(&mut self) -> ExpressionBox {
        use Token::*;

        if let Some(operator) = self.matches(|x| matches!(x, Bang | Minus)) {
            let expression = self.unary();
            box UnaryExpression::new(operator, expression)
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> ExpressionBox {
        use Token::*;

        let x = self.advance().unwrap().clone();

        if let False | True | Nil | Number(_) | String(_) = x {
            box LiteralExpression::new(x)
        } else if let LeftParen = x {
            let expr = self.expression();

            self.matches(|x| matches!(x, RightParen));
            // if ! {
            // private Token consume(TokenType type, String message) {
            //   if (check(type)) return advance();
            //   throw error(peek(), message);
            // }
            // consume(RIGHT_PAREN, "Expect ')' after expression.");
            // panic!()
            // }

            box GroupingExpression::new(expr)
        } else {
            unreachable!()
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
