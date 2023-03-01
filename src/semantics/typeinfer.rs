use crate::ast::tree::*;
use anyhow::{anyhow, Error as AnyError};

pub fn infer_types(_prgm: Program<()>) -> Result<Program<Type>, AnyError> {
    Err(anyhow!("Not implemented"))
}
