use thiserror::Error;

use super::tokens::Token;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing number: '{0}'")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Unrecognized token: '{0}'")]
    UnrecognizedToken(char, usize),
    #[error("Expected {0}, Received: '{1}'")]
    UnexpectedToken(String, Token),
    #[error("Unknown operator: '{0}'")]
    InvalidOperator(String),
    #[error("Unknown size: '{0}'")]
    InvalidSize(u8),
    #[error("Unknown Type: '{0}'")]
    InvalidType(Token),
    #[error("Unterminated string '{0}'")]
    UnterminatedString(usize),
    #[error("Unterminated char '{0}'")]
    UnterminatedChar(usize),
}

pub type Parse<T> = Result<T, Error>;
