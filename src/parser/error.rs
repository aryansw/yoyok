use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing number: '{0}'")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Unrecognized token: '{0}'")]
    UnrecognizedToken(char),
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    #[error("Expected {0}, Received token: '{0}'")]
    UnexpectedToken(String, String),
}

pub type Parse<T> = Result<T, Error>;
