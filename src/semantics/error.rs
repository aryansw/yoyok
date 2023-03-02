use super::types::Type;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected Type: expected '{0}' but found '{1}'")]
    UnexpectedType(Type, Type),
    #[error("Redeclaration of function '{2}' with types: '{0}' and {1}")]
    Redeclaration(Type, Type, String),
    #[error("Function '{0}' not found")]
    FunctionNotFound(String),
    #[error("Variable '{0}' not found")]
    VariableNotFound(String),
    #[error("Expected Function but found '{0}'")]
    ExpectedFunction(Type),
}
