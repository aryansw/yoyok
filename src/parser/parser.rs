use crate::parser::error::Error;

use super::{scanner::Scanner, tokens::TokenType};

pub fn parse(src: &str) -> Result<(), Error> {
    let mut scan = Scanner::new(src);
    println!("Program:\n\n{}\n\n", src);
    let mut tok = scan.next()?;
    println!("Tokens:\n");
    Ok(())
}