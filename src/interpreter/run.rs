use std::collections::HashMap;

use anyhow::Context;

use super::{error::Error, value::Value};
use crate::ast::ast::*;
use crate::ast::ast::{self, Expression as Expr};
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

fn run_exprs(exprs: Vec<Expr>, funcs: &Vec<Function>, mut env: Env) -> Result<Value, AnyError> {
    let mut val = Value::Signed(1);
    for expr in exprs {
        val = run_expr(expr, funcs, &mut env)?;
    }
    Ok(val)
}

fn run_expr(expr: Expression, funcs: &Vec<Function>, env: &mut Env) -> Result<Value, AnyError> {
    match expr {
        Expression::Unary { op, rhs } => {
            let val = run_expr(*rhs, funcs, env)?;
            val.unary(op)
        }
        // Remember that assign needs to be handled differently
        Expression::Binary { lhs, op, rhs } => {
            let lhs = run_expr(*lhs, funcs, env)?;
            let rhs = run_expr(*rhs, funcs, env)?;
            lhs.binary(op, &rhs)
        }
        Expression::Value(val) => todo!(),
        Expression::Tuple(_) => todo!(),
        Expression::Array(_) => todo!(),
        Expression::Reference(_) => todo!(),
        Expression::Let {
            name,
            value,
            ty,
            mutable,
        } => todo!(),
        Expression::If { cond, then, else_ } => todo!(),
        Expression::Call { func, args } => todo!(),
        Expression::While { cond, body } => todo!(),
    }
}

impl Value {
    fn unary(&self, op: Operator) -> Result<Value, AnyError> {
        match op {
            Operator::Not => {
                let Value::Bool(x) = self else {
                    Err(Error::InvalidUnary(op, self.type_of()))?
                };
                Ok(Value::Bool(!x))
            }
            Operator::Sub => {
                let Value::Signed(x) = self else {
                    Err(Error::InvalidUnary(op, self.type_of()))?
                };
                Ok(Value::Signed(-x))
            }
            _ => Err(Error::InvalidUnary(op, self.type_of()))?,
        }
    }

    fn binary(&self, op: Operator, rhs: &Value) -> Result<Value, AnyError> {
        let value = match (self, rhs) {
            (Value::Signed(x), Value::Signed(y)) => match op {
                Operator::Add => Value::Signed(x + y),
                Operator::Sub => Value::Signed(x - y),
                Operator::Mul => Value::Signed(x * y),
                Operator::Div => Value::Signed(x / y),
                Operator::Gt => Value::Bool(x > y),
                Operator::Lt => Value::Bool(x < y),
                Operator::Gte => Value::Bool(x >= y),
                Operator::Lte => Value::Bool(x <= y),
                Operator::Eq => Value::Bool(x == y),
                Operator::Neq => Value::Bool(x != y),
                _ => Err(Error::InvalidBinary(op, self.type_of(), rhs.type_of()))?,
            },
            (Value::Bool(x), Value::Bool(y)) => match op {
                Operator::And => Value::Bool(*x && *y),
                Operator::Or => Value::Bool(*x || *y),
                Operator::Eq => Value::Bool(x == y),
                Operator::Neq => Value::Bool(x != y),
                _ => Err(Error::InvalidBinary(op, self.type_of(), rhs.type_of()))?,
            },
            _ => Err(Error::InvalidBinary(op, self.type_of(), rhs.type_of()))?,
        };
        Ok(value)
    }
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
