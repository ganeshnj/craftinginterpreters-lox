use crate::token::{BinaryOperator, Expr, Literal, Token, UnaryOperator};
use crate::token_type::TokenType;

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
 */
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub(crate) fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();

            let bin_op = Self::binary_operator(operator.token_type);
            expr = Expr::Binary(Box::from(expr), bin_op, Box::from(right));
        }

        expr
    }

    fn binary_operator(token_type: TokenType) -> BinaryOperator {
        match token_type {
            TokenType::BangEqual => BinaryOperator::NotEqual,
            TokenType::EqualEqual => BinaryOperator::EqualEqual,
            TokenType::Less => BinaryOperator::Less,
            TokenType::LessEqual => BinaryOperator::LessEqual,
            TokenType::Greater => BinaryOperator::Greater,
            TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
            TokenType::Plus => BinaryOperator::Plus,
            TokenType::Minus => BinaryOperator::Minus,
            TokenType::Star => BinaryOperator::Star,
            TokenType::Slash => BinaryOperator::Slash,
            _ => panic!("Invalid operator"),
        }
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            let bin_op = Self::binary_operator(operator.token_type);
            expr = Expr::Binary(Box::from(expr), bin_op, Box::from(right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let expr = self.factor();
        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            let bin_op = Self::binary_operator(operator.token_type);
            return Expr::Binary(Box::from(expr), bin_op, Box::from(right));
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let expr = self.unary();
        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            let bin_op = Self::binary_operator(operator.token_type);
            return Expr::Binary(Box::from(expr), bin_op, Box::from(right));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            let unary_op = Self::unary_operator(operator.token_type);
            return Expr::Unary(unary_op, Box::new(right));
        }
        self.primary()
    }

    fn unary_operator(token_type: TokenType) -> UnaryOperator {
        match token_type {
            TokenType::Bang => UnaryOperator::Bang,
            TokenType::Minus => UnaryOperator::Minus,
            _ => panic!("Invalid operator"),
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::FALSE]) {
            return Expr::Literal(Literal::False);
        }
        if self.match_token(vec![TokenType::TRUE]) {
            return Expr::Literal(Literal::True);
        }
        if self.match_token(vec![TokenType::NIL]) {
            return Expr::Literal(Literal::Nil);
        }
        if self.match_token(vec![TokenType::Number]) {
            return Expr::Literal(Literal::Number(self.previous().literal.parse().unwrap()));
        }
        if self.match_token(vec![TokenType::String]) {
            return Expr::Literal(Literal::String(self.previous().literal));
        }
        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            let _ = self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(Box::from(expr));
        }
        panic!("Expect expression.");
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(Error::ParseError)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS | TokenType::FUN | TokenType::VAR | TokenType::FOR | TokenType::IF | TokenType::WHILE | TokenType::PRINT | TokenType::RETURN => {
                    return;
                }
                _ => {
                    _ = self.advance()
                }
            }
        }
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false
        }
        return self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

pub enum Error {
    ParseError,
}