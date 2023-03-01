use std::collections::HashMap;

use crate::ast::tree::*;
use anyhow::{anyhow, Context, Error as AnyError};

use super::{
    error::Error,
    types::{Size, Type},
};

type TypeEnv<'a> = HashMap<&'a str, Type>;

pub fn infer(prgm: Program<()>) -> Result<Program<Type>, AnyError> {
    let funcs = prgm.0;

    let mut env = TypeEnv::new();
    // Add the function types to the environment
    for func in &funcs {
        let ty = func.get_type();
        // Ensure that the function names are unique
        if let Some(ty1) = env.remove(func.name.as_str()) {
            return Err(Error::Redeclaration(ty, ty1, func.name.clone()).into());
        }
        env.insert(func.name.as_str(), ty);
    }
    // Ensure there's a main function
    if let Some(f) = env.get("main") {
        f.expect(&Type::Function {
            args: vec![],
            ret: Box::new(Type::Signed(Size::ThirtyTwo)),
        })
        .context("In function main()")?;
    } else {
        Err(Error::FunctionNotFound("main".into()))
            .context("You need to declare a main() function")?
    }
    // Infer the types of the functions, one by one
    let new_funcs: Vec<Function<Type>> = funcs
        .iter()
        .map(|f| infer_function(f, &env).context(format!("In function '{}'", f.name)))
        .collect::<Result<Vec<Function<Type>>, AnyError>>()?;
    Ok(Program(new_funcs))
}

fn infer_function(func: &Function<()>, env: &TypeEnv) -> Result<Function<Type>, AnyError> {
    Err(anyhow!("Not implemented"))
}

impl<T: TypeBound> Function<T> {
    fn expect(&self, ty: &Type) -> Result<(), AnyError> {
        self.get_type()
            .expect(ty)
            .context(format!("In function '{}'", self.name))
    }

    fn get_type(&self) -> Type {
        Type::Function {
            args: self.args.iter().map(|(_, ty)| ty).cloned().collect(),
            ret: Box::new(self.ret.clone()),
        }
    }
}

impl Type {
    // A strict equality check
    fn expect(&self, ty: &Type) -> Result<(), AnyError> {
        if self == ty {
            Ok(())
        } else {
            Err(Error::UnexpectedType(self.clone(), ty.clone()).into())
        }
    }
}
