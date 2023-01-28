use anyhow::Context;

use super::{
    constants::{is_comment, is_delim, is_keyword, is_operator},
    error::{Error, Parse},
    tokens::{
        Token,
        TokenType::{self, *},
    },
};

pub struct Scanner<'a> {
    src: &'a str,
    pos: usize,
    next: Option<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
            next: None,
        }
    }

    pub fn peek(&mut self) -> Parse<Token> {
        if self.next.is_none() {
            self.next = Some(self.next()?);
        }
        Ok(self.next.clone().unwrap())
    }

    pub fn next(&mut self) -> Parse<Token> {
        if let Some(tok) = self.next.take() {
            Ok(tok)
        } else {
            Ok(self.next_tok()?)
        }
    }

    fn next_tok(&mut self) -> Parse<Token> {
        // Consume whitespace
        self.consume_whitespace();
        // Save current position
        let pos = self.pos;
        match self.peek_char() {
            Some(c) if c.is_numeric() => Ok(Number(self.next_num()?)),
            Some(c) if c.is_alphabetic() => {
                let word = self.next_word()?;
                if is_keyword(&word) {
                    Ok(Keyword(word.into()))
                } else {
                    Ok(Name(word))
                }
            }
            Some(c) if is_operator(c) => Ok(Op(self.next_op()?)),
            Some(c) if is_delim(c) => {
                self.next_char();
                Ok(Delim(c))
            }
            Some(x) if is_comment(x) => {
                self.consume_line();
                self.next_tok().map(|tok| tok.token)
            }
            Some(x) => Err(Error::UnrecognizedToken(x)),
            None => Ok(EOF),
        }
        .map(|tok| tok.with_pos(pos))
    }

    fn next_op(&mut self) -> Parse<Vec<char>> {
        let mut op = vec![];
        while let Some(c) = self.peek_char() && is_operator(c) {
            op.push(c);
            self.next_char();
        }
        Ok(op)
    }

    fn next_word(&mut self) -> Parse<String> {
        let mut word = String::new();
        while let Some(c) = self.peek_char() && (c.is_alphanumeric() || c == '_') {
            word.push(c);
            self.next_char();
        }
        Ok(word)
    }

    fn next_num(&mut self) -> Result<u64, Error> {
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

    fn consume_line(&mut self) {
        while self.pos < self.src.len() && !self.src[self.pos..].starts_with('\n') {
            self.pos += 1;
        }
    }

    fn consume_whitespace(&mut self) {
        while self.pos < self.src.len() && self.src[self.pos..].starts_with(char::is_whitespace) {
            self.pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use crate::parser::scanner::TokenType::*;
    use crate::parser::{error::Error, tokens::Keyword::*};

    use super::Scanner;

    #[test]
    fn test_scanner() -> Result<(), Error> {
        let mut scan = Scanner::new("let x = 5;");
        assert_eq!(scan.next()?.token, Keyword(Let));
        assert_eq!(scan.next()?.token, Name("x".into()));
        assert_eq!(scan.next()?.token, Op(vec!['=']));
        assert_eq!(scan.next()?.token, Number(5));
        assert_eq!(scan.next()?.token, Delim(';'));
        assert_eq!(scan.next()?.token, EOF);
        assert_eq!(scan.next()?.token, EOF);
        Ok(())
    }
}
