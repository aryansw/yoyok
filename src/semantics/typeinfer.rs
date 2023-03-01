use crate::ast::tree::*;
use anyhow::{anyhow, Error as AnyError};

use super::types::Type;

pub fn infer_program(prgm: Program<()>) -> Result<Program<Type>, AnyError> {
    let funcs = prgm.0;
    let new_funcs: Vec<Function<Type>> = funcs
        .iter()
        .map(infer_function)
        .collect::<Result<Vec<Function<Type>>, AnyError>>()?;
    Err(anyhow!("Not implemented"))
}

pub fn infer_function(func: &Function<()>) -> Result<Function<Type>, AnyError> {
    Err(anyhow!("Not implemented"))
}
