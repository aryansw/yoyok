use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing number: '{0}'")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Unrecognized token: '{0}'")]
    UnrecognizedToken(char),
}

pub type Parse<T> = Result<T, Error>;
