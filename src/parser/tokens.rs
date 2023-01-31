use crate::ast::ast::Value;

use super::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pub pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(u64),
    Op(Vec<char>),
    Name(String),
    Keyword(Keyword),
    Delim(char),
    Literal(Literal),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Char(char),
}

impl Into<Literal> for char {
    fn into(self) -> Literal {
        Literal::Char(self)
    }
}

impl Into<Literal> for String {
    fn into(self) -> Literal {
        Literal::String(self)
    }
}

impl Into<Value> for Literal {
    fn into(self) -> Value {
        match self {
            Literal::Char(c) => Value::Char(c),
            Literal::String(s) => Value::String(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Var,
    If,
    Else,
    True,
    False,
    Func,
    While,
}

impl TokenType {
    pub fn with_pos(self, pos: usize) -> Token {
        Token { token: self, pos }
    }
}

impl Into<Keyword> for String {
    fn into(self) -> Keyword {
        match self.as_str() {
            "let" => Keyword::Let,
            "var" => Keyword::Var,
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "true" => Keyword::True,
            "false" => Keyword::False,
            "fn" => Keyword::Func,
            "while" => Keyword::While,
            _ => panic!("Invalid keyword"),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.token)
    }
}

impl Token {
    // Name or throws an error:
    pub fn name(&self) -> Result<String, Error> {
        match &self.token {
            TokenType::Name(name) => Ok(name.clone()),
            _ => Err(Error::UnexpectedToken("Name(_)".into(), self.clone())),
        }
    }

    // Number or throws an error:
    pub fn number(&self) -> Result<u64, Error> {
        match &self.token {
            TokenType::Number(n) => Ok(*n),
            _ => Err(Error::UnexpectedToken("Number(_)".into(), self.clone())),
        }
    }
}
