use super::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub token: TokenType,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(u64),
    Op(Vec<char>),
    Name(String),
    Keyword(Keyword),
    Delim(char),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Var,
    If,
    Else,
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
}
