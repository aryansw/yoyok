use anyhow::Context;

use super::error::Error;
use crate::ast::ast::Expression as Expr;
use crate::ast::ast::*;
use anyhow::Error as AnyError;

pub fn run_program(prgm: Program) -> Result<(), AnyError> {
    let funcs = prgm.0;
    for func in funcs {
        if func.name == "main" {
            func.ret
                .expect(Type::Signed(Size::ThirtyTwo))
                .context("main() function must have return type i32")?;
            if func.args.len() != 0 {
                Err(Error::ArgumentCountMismatch(0, func.args.len()))?;
            }
            run_func(func)?;
        }
    }
    Ok(())
}

fn run_func(func: Function) -> Result<(), AnyError> {
    run_exprs(func.body.0)
}

fn run_expr(expr: Expression) -> Result<(), AnyError> {
    Ok(())
}

fn run_exprs(exprs: Vec<Expr>) -> Result<(), AnyError> {
    for expr in exprs {
        run_expr(expr)?;
    }
    Ok(())
}

impl Type {
    fn expect(&self, expected: Type) -> Result<(), Error> {
        if self == &expected {
            Ok(())
        } else {
            Err(Error::UnexpectedType(expected, self.clone()))
        }
    }
}
