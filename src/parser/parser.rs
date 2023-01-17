use crate::parser::error::Error;

pub fn parse(source: &str) -> Result<(), Error> {
    println!("Tokens: {:?}", source);
    Ok(())
}
