use crate::{interpreter::error::Error as InterpretError, parser::error::Error as ParserError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read file")]
    ReadFile(#[from] std::io::Error),
    #[error("Error while parsing program")]
    Parse(#[from] ParserError),
    #[error("Error while interpreting program")]
    Interpreter(#[from] InterpretError),
    #[error("Error with logging")]
    Log,
}
