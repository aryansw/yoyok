use crate::ast::ast::*;
use anyhow::{anyhow, Error as AnyError};

pub fn infer_types(prgm: Program<()>) -> Result<Program<Type>, AnyError> {
    Err(anyhow!("Not implemented"))
}
