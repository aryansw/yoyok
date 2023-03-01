use thiserror::Error;

use crate::ast::ast::{Operator, Type};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected Type: expected '{0}' but found '{1}'")]
    UnexpectedType(Type, Type),
    #[error("Non-zero exit code: {0}")]
    NonZeroExitCode(i64),
    #[error("Argument count mismatch: expected {0} but found {1:?}")]
    ArgumentCountMismatch(usize, Vec<(String, Type)>),
    #[error("Invalid Operation: '{0}' on '{1}'")]
    InvalidUnary(Operator, Type),
    #[error("Invalid Operation: '{0}' on '{1}' and '{2}'")]
    InvalidBinary(Operator, Type, Type),
    #[error("Undefined variable: '{0}'")]
    UndefinedVariable(String),
    #[error("Immutable variable: '{0}'")]
    ImmutableVariable(String),
    #[error("Invalid Assignment: '{0}' is not assignable")]
    InvalidAssignment(String),
}
