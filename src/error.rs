use crate::parser::error::Error as ParserError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read file")]
    ReadFileError(#[from] std::io::Error),
    #[error("Error while parsing program")]
    ParserError(#[from] ParserError),
}
