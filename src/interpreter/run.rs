use std::collections::HashMap;

use anyhow::Context;

use super::{error::Error, value::Value};
use crate::ast::ast::Expression as Expr;
use crate::ast::ast::*;
use anyhow::Error as AnyError;

type Env = HashMap<String, Value>;

pub fn run_program(prgm: Program) -> Result<(), AnyError> {
    let funcs = prgm.0;
    let main: Function = funcs
        .iter()
        .find(|func| func.name == "main")
        .cloned()
        .context("No main() function found")?;
    main.ret
        .expect(Type::Signed(Size::ThirtyTwo))
        .context("main() function should return i32")?;
    if main.args.len() != 0 {
        Err(Error::ArgumentCountMismatch(0, main.args))?
    } else {
        let value = run_func(main, &funcs)?;
        if let Value::Signed(val) = value {
            if val == 0 {
                Ok(())
            } else {
                Err(Error::NonZeroExitCode(val))?
            }
        } else {
            Err(Error::UnexpectedType(
                Type::Signed(Size::ThirtyTwo),
                value.type_of(),
            ))?
        }
    }
}

fn run_func(curr: Function, funcs: &Vec<Function>) -> Result<Value, AnyError> {
    run_exprs(curr.body.0, &funcs, HashMap::new())
}

fn run_exprs(
    exprs: Vec<Expr>,
    funcs: &Vec<Function>,
    mut env: HashMap<String, Value>,
) -> Result<Value, AnyError> {
    for expr in exprs {
        run_expr(expr)?;
    }
    Ok(Value::Signed(0))
}

fn run_expr(expr: Expression) -> Result<Value, AnyError> {
    Ok(Value::Signed(0))
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
