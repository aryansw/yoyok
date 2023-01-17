use super::{
    error::Parse,
    tokens::{
        Token,
        TokenType::{self, *},
    },
};

struct Scanner<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Scanner<'a> {
    fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    fn next(&mut self) -> Parse<Token> {
        // Save current position
        let pos = self.pos;
        Ok(self.next_tok()?.with_pos(pos))
    }

    fn next_tok(&mut self) -> Parse<TokenType> {
        let src = self.src;
        let pos = &mut self.pos;
        let len = src.len();
        // EOF check
        if *pos >= len {
            return Ok(EOF);
        }
        // Consume whitespaces
        while src[*pos..].starts_with(char::is_whitespace) {
            *pos += 1;
            if *pos >= len {
                return Ok(EOF);
            }
        }
        // Check for number
        let mut end = *pos;
        while end != len && src[end..].starts_with(char::is_numeric) {
            
        }
        Ok(EOF)
    }

    fn consume_whitespace(&mut self) {}
}
