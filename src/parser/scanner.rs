use anyhow::Context;

use super::{
    constants::{is_delim, is_keyword, is_operator, DELIMS},
    error::{Error, Parse},
    tokens::{
        Token,
        TokenType::{self, *},
    },
};

pub struct Scanner<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    pub fn next(&mut self) -> Parse<Token> {
        // Save current position
        let pos = self.pos;
        Ok(self.next_tok()?.with_pos(pos))
    }

    fn next_tok(&mut self) -> Parse<TokenType> {
        // Consume whitespace
        self.consume_whitespace();
        match self.peek_char() {
            Some(c) if c.is_numeric() => Ok(Number(self.next_num()?)),
            Some(c) if c.is_alphabetic() => {
                let word = self.next_word()?;
                if is_keyword(&word) {
                    Ok(Keyword(word.into()))
                } else {
                    Ok(Ident(word))
                }
            }
            Some(c) if is_operator(c) => Ok(Ident(self.next_op()?)),
            Some(c) if is_delim(c) => {
                self.next_char();
                Ok(Delim(c))
            }
            _ => Ok(EOF),
        }
    }

    fn next_op(&mut self) -> Parse<String> {
        let mut op = String::new();
        while let Some(c) = self.peek_char() && is_operator(c) {
            op.push(c);
            self.next_char();
        }
        Ok(op)
    }

    fn next_word(&mut self) -> Parse<String> {
        let mut word = String::new();
        while let Some(c) = self.peek_char() && c.is_alphanumeric() {
            word.push(c);
            self.next_char();
        }
        Ok(word)
    }

    fn next_num(&mut self) -> Result<i64, Error> {
        let mut num = String::new();
        while let Some(c) = self.peek_char() && c.is_numeric() {
            num.push(c);
            self.next_char();
        }
        Ok(num.parse()?)
    }

    fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.src.len() {
            None
        } else {
            let c = self.src[self.pos..].chars().next()?;
            self.pos += c.len_utf8();
            Some(c)
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.pos >= self.src.len() {
            None
        } else {
            self.src[self.pos..].chars().next()
        }
    }

    fn consume_whitespace(&mut self) {
        while self.pos < self.src.len() && self.src[self.pos..].starts_with(char::is_whitespace) {
            self.pos += 1;
        }
    }
}
