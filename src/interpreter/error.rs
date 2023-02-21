use proptest::test_runner::Reason;
use thiserror::Error;

use crate::ast::ast::Type;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected Type: expected '{0}' but found '{1}'")]
    UnexpectedType(Type, Type),
    #[error("Unknown function: '{0}'")]
    UnknownFunction(String),
    #[error("Unknown variable: '{0}'")]
    UnknownVariable(String),
    #[error("Argument count mismatch: expected {0} but found {1}")]
    ArgumentCountMismatch(usize, usize),
}