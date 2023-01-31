use crate::parser::error::Error as ParserError;
use proptest::test_runner::Reason;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to read file")]
    ReadFileError(#[from] std::io::Error),
    #[error("Error while parsing program")]
    ParserError(#[from] ParserError),
    #[error("Error with logging")]
    LogError,
}

impl Into<Reason> for Error {
    fn into(self) -> Reason {
        self.to_string().into()
    }
}
