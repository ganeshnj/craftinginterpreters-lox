use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: String,
    pub(crate) line: usize,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Token {
        Token { token_type, lexeme, literal, line }
    }

    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, BinaryOperator, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(UnaryOperator, Box<Expr>),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(Debug)]
pub enum BinaryOperator {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}
#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}