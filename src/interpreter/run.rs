use super::env::Env;
use super::{error::Error, value::Value};
use crate::ast::ast::*;
use crate::ast::ast::{self, Expression as Expr};
use anyhow::Context;
use anyhow::Error as AnyError;

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
        let value = run_func(&main, Env::from_funcs(funcs))?;
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

fn run_func(curr: &Function, mut env: Env) -> Result<Value, AnyError> {
    run_exprs(&curr.body.0, &mut env)
}

fn run_exprs(exprs: &Vec<Expr>, env: &mut Env) -> Result<Value, AnyError> {
    let mut val = Value::Signed(0);
    for expr in exprs {
        val = run_expr(&expr, env)?;
    }
    Ok(val)
}

fn run_expr(expr: &Expression, env: &mut Env) -> Result<Value, AnyError> {
    let val = match expr {
        Expression::Unary { op, rhs } => {
            let val = run_expr(rhs, env)?;
            val.unary(op)?
        }
        // Remember that assign needs to be handled differently
        Expression::Binary {
            lhs,
            op: Operator::Assign,
            rhs,
        } => {
            if let Expression::Reference(name) = lhs.as_ref() {
                let val = run_expr(rhs, env)?;
                env.update(name, val)?;
                Value::Tuple(vec![])
            } else {
                Err(Error::InvalidAssignment(lhs.to_string()))?
            }
        }

        Expression::Binary { lhs, op, rhs } => {
            let lhs = run_expr(lhs, env)?;
            let rhs = run_expr(rhs, env)?;
            lhs.binary(op, &rhs)?
        }
        Expression::Value(val) => match val {
            ast::Value::Number(x) => Value::Signed(*x),
            ast::Value::Bool(x) => Value::Bool(*x),
            ast::Value::Char(x) => Value::Char(*x),
            ast::Value::String(x) => Value::Array(x.chars().into_iter().map(Value::Char).collect()),
        },
        Expression::Tuple(_) => todo!(),
        Expression::Array(_) => todo!(),
        Expression::Reference(x) => env.get(x)?,
        Expression::Let {
            name,
            value,
            ty,
            mutable,
        } => {
            let val = run_expr(value, env)?;
            if let Some(ty) = ty {
                ty.context(val.type_of())?;
            }
            env.insert(name, val, *mutable);
            Value::Tuple(vec![])
        }
        Expression::If { cond, then, else_ } => {
            let cond = run_expr(cond, env)?;
            if let Value::Bool(cond) = cond {
                if cond {
                    run_exprs(&then.0, env)?
                } else if let Some(else_) = else_ {
                    run_exprs(&else_.0, env)?
                } else {
                    Value::Tuple(vec![])
                }
            } else {
                Err(Error::UnexpectedType(Type::Bool, cond.type_of()))?
            }
        }
        Expression::Call { func, args } => {
            let func = run_expr(func, env)?;
            let args = args
                .into_iter()
                .map(|arg| run_expr(arg, env))
                .collect::<Result<Vec<_>, _>>()?;
            if let Value::Function(func) = &func && func.args.len() == args.len(){
                let mut env = env.call();
                // Create a new environment for the function, and insert the arguments into it
                for ((name, _), val) in func.args.iter().zip(args) {
                    env.insert(name, val, false);
                }
                run_func(func, env)?
            } else {
                let arg_ty = args.iter().map(|arg| arg.type_of()).collect::<Vec<_>>();
                Err(Error::UnexpectedType(
                    Type::Function {
                        args: arg_ty,
                        ret: Box::new(Type::unit()),
                    },
                    func.type_of(),
                ))?
            }
        }
        Expression::While { cond, body } => {
            loop {
                let cond = run_expr(cond, env)?;
                if let Value::Bool(cond) = cond {
                    if cond {
                        run_exprs(&body.0, env)?;
                    } else {
                        break;
                    }
                } else {
                    Err(Error::UnexpectedType(Type::Bool, cond.type_of()))?
                }
            }
            Value::Tuple(vec![])
        }
    };
    Ok(val)
}

impl Value {
    fn unary(&self, op: &Operator) -> Result<Value, AnyError> {
        match op {
            Operator::Not => {
                let Value::Bool(x) = self else {
                    Err(Error::InvalidUnary(*op, self.type_of()))?
                };
                Ok(Value::Bool(!x))
            }
            Operator::Sub => {
                let Value::Signed(x) = self else {
                    Err(Error::InvalidUnary(*op, self.type_of()))?
                };
                Ok(Value::Signed(-x))
            }
            _ => Err(Error::InvalidUnary(*op, self.type_of()))?,
        }
    }

    fn binary(&self, op: &Operator, rhs: &Value) -> Result<Value, AnyError> {
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
                _ => Err(Error::InvalidBinary(*op, self.type_of(), rhs.type_of()))?,
            },
            (Value::Bool(x), Value::Bool(y)) => match op {
                Operator::And => Value::Bool(*x && *y),
                Operator::Or => Value::Bool(*x || *y),
                Operator::Eq => Value::Bool(x == y),
                Operator::Neq => Value::Bool(x != y),
                _ => Err(Error::InvalidBinary(*op, self.type_of(), rhs.type_of()))?,
            },
            (Value::Char(x), Value::Char(y)) => match op {
                Operator::Eq => Value::Bool(x == y),
                Operator::Neq => Value::Bool(x != y),
                _ => Err(Error::InvalidBinary(*op, self.type_of(), rhs.type_of()))?,
            },
            _ => Err(Error::InvalidBinary(*op, self.type_of(), rhs.type_of()))?,
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

    fn context(&self, actual: Type) -> Result<(), Error> {
        if self == &actual {
            Ok(())
        } else {
            Err(Error::UnexpectedType(self.clone(), actual))
        }
    }
}
