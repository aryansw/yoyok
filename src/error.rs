use crate::{interpreter::error::Error as InterpretError, parser::error::Error as ParserError};
use proptest::test_runner::Reason;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read file")]
    ReadFileError(#[from] std::io::Error),
    #[error("Error while parsing program")]
    ParserError(#[from] ParserError),
    #[error("Error while interpreting program")]
    InterpreterError(#[from] InterpretError),
    #[error("Error with logging")]
    LogError,
}
