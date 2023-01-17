pub struct Token {
    pub token: TokenType,
    pos: usize,
}

#[derive(Debug)]
pub enum TokenType {
    Number(i64),
    Ident(String),
    Keyword(Keyword),
    Delim(char),
    EOF,
}

#[derive(Debug)]
pub enum Keyword {
    Let,
    Var,
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
            _ => panic!("Invalid keyword"),
        }
    }
}
