use std::collections::HashMap;

use crate::ast::tree::*;
use anyhow::{anyhow, Context, Error as AnyError};

use super::{
    error::Error,
    types::{Size, Type},
};

type TypeEnv = HashMap<String, Type>;

pub fn infer(prgm: Program<()>) -> Result<Program<Type>, AnyError> {
    let funcs = prgm.0;
    let mut env = TypeEnv::new();
    // Add the function types to the environment
    {
        let borrow = &funcs;
        for func in funcs.iter() {
            let ty = func.get_type();
            // Ensure that the function names are unique
            if let Some(ty1) = env.remove(func.name.as_str()) {
                return Err(Error::Redeclaration(ty, ty1, func.name.clone()).into());
            }
            env.insert(func.name.clone(), ty);
        }
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
        .into_iter()
        .map(|f| {
            let name = f.name.clone();
            infer_function(f, &env).context(format!("In function '{}'", name))
        })
        .collect::<Result<Vec<Function<Type>>, AnyError>>()?;
    Ok(Program(new_funcs))
}

fn infer_function(func: Function<()>, env: &TypeEnv) -> Result<Function<Type>, AnyError> {
    let mut new_env = env.clone();
    for (name, ty) in &func.args {
        new_env.insert(name.into(), ty.clone());
    }
    let new_body = infer_seq(func.body, &new_env)?;
    let default = Type::unit();
    let ret_ty = new_body.0.last().map_or(&default, |f| &f.ty);
    ret_ty
        .expect(&func.ret)
        .context(format!("Return type mismatch for function {}", func.name))?;
    Ok(Function {
        name: func.name,
        args: func.args,
        ret: func.ret,
        body: new_body,
    })
}

fn infer_seq(seq: Sequence<()>, env: &TypeEnv) -> Result<Sequence<Type>, AnyError> {
    let new_exprs = infer_exprs(seq.0, env)?;
    Ok(Sequence(new_exprs))
}

fn infer_exprs(
    exprs: Vec<Expression<()>>,
    env: &TypeEnv,
) -> Result<Vec<Expression<Type>>, AnyError> {
    exprs
        .into_iter()
        .map(|e| infer_expr(e, env))
        .collect::<Result<Vec<Expression<Type>>, AnyError>>()
}

fn infer_expr(expr: Expression<()>, env: &TypeEnv) -> Result<Expression<Type>, AnyError> {
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
