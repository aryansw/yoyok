use super::env::Env;
use super::{error::Error, value::Value};
use crate::ast::tree::*;
use crate::ast::tree::{self};
use anyhow::Context;
use anyhow::Error as AnyError;

// Currently, the interpreter completely ignores the type information (Type Erasure),
// but we can use the type information to check the types of the values at runtime.
pub fn run_program<T: TypeBound>(prgm: Program<T>) -> Result<(), AnyError> {
    let funcs = prgm.0;
    let main: Function<T> = funcs
        .iter()
        .find(|func| func.name == "main")
        .cloned()
        .context("No main() function found")?;
    main.ret
        .expect(Type::Signed(Size::ThirtyTwo))
        .context("main() function should return i32")?;
    if !main.args.is_empty() {
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

fn run_func<T: TypeBound>(curr: &Function<T>, mut env: Env<T>) -> Result<Value<T>, AnyError> {
    run_exprs(&curr.body.0, &mut env)
}

fn run_exprs<T: TypeBound>(
    exprs: &Vec<Expression<T>>,
    env: &mut Env<T>,
) -> Result<Value<T>, AnyError> {
    let mut val = Value::Signed(0);
    for expr in exprs {
        val = run_expr(expr, env).context(format!("On expression: {}", expr))?;
    }
    Ok(val)
}

fn run_expr<T: TypeBound>(expr: &Expression<T>, env: &mut Env<T>) -> Result<Value<T>, AnyError> {
    let val = match &expr.expr {
        Expr::Unary { op, rhs } => {
            let val = run_expr(rhs, env)?;
            val.unary(op)?
        }
        // Remember that assign needs to be handled differently
        Expr::Binary {
            lhs,
            op: Operator::Assign,
            rhs,
        } => {
            if let Expr::Reference(name) = &lhs.as_ref().expr {
                let val = run_expr(rhs, env)?;
                env.update(name, val)?;
                Value::Tuple(vec![])
            } else {
                Err(Error::InvalidAssignment(lhs.to_string()))?
            }
        }
        Expr::Binary { lhs, op, rhs } => {
            let lhs = run_expr(lhs, env)?;
            let rhs = run_expr(rhs, env)?;
            lhs.binary(op, &rhs)?
        }
        Expr::Value(val) => match val {
            tree::Value::Number(x) => Value::Signed(*x),
            tree::Value::Bool(x) => Value::Bool(*x),
            tree::Value::Char(x) => Value::Char(*x),
            tree::Value::String(x) => Value::Array(x.chars().map(Value::Char).collect()),
        },
        Expr::Tuple(_) => todo!(),
        Expr::Array(_) => todo!(),
        Expr::Reference(x) => env.get(x)?,
        Expr::Let {
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
        Expr::If { cond, then, else_ } => {
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
        Expr::Call { func, args } => {
            let func = run_expr(func, env)?;
            let args = args
                .iter()
                .map(|arg| run_expr(arg, env))
                .collect::<Result<Vec<_>, _>>()?;
            if let Value::Function(func) = &func && func.args.len() == args.len(){
                let mut env = env.call();
                // Create a new environment for the function, and insert the arguments into it
                for ((name, _), val) in func.args.iter().zip(args) {
                    env.insert(name, val, false);
                }
                run_func(func, env).context(format!("On call to function '{}'", func.name))?
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
        Expr::While { cond, body } => {
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

impl<T: TypeBound> Value<T> {
    fn unary(&self, op: &Operator) -> Result<Value<T>, AnyError> {
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

    fn binary(&self, op: &Operator, rhs: &Value<T>) -> Result<Value<T>, AnyError> {
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
