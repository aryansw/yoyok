use thiserror::Error;

use crate::ast::tree::{Expr, Type};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected Type: expected '{0}' but found '{1}' for '{2}'")]
    UnexpectedType(Type, Type, Expr<()>),
}
