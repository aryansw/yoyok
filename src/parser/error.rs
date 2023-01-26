use thiserror::Error;

use super::tokens::Token;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing number: '{0}'")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Unrecognized token: '{0}'")]
    UnrecognizedToken(char),
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    #[error("Expected {0}, Received: '{1}'")]
    UnexpectedToken(String, Token),
}

pub type Parse<T> = Result<T, Error>;
