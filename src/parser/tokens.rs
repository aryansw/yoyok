pub struct Token {
    token: TokenType,
    pos: usize,
}

pub enum TokenType {
    Number(i64),
    Ident(String),
    Keyword(Keyword),
    Delim(char),
    EOF,
}

impl TokenType {
    pub fn with_pos(self, pos: usize) -> Token {
        Token { token: self, pos }
    }
}

pub enum Keyword {
    Let,
    Var,
}
