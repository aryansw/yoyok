use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing number: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub type Parse<T> = Result<T, Error>;
