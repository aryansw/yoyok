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
    for func in funcs.iter() {
        let ty = func.get_type();
        // Ensure that the function names are unique
        if let Some(ty1) = env.remove(func.name.as_str()) {
            return Err(Error::Redeclaration(ty, ty1, func.name.clone()).into());
        }
        env.insert(func.name.clone(), ty);
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
            infer_function(f, &mut env).context(format!("In function '{}'", name))
        })
        .collect::<Result<Vec<Function<Type>>, AnyError>>()?;
    Ok(Program(new_funcs))
}

fn infer_function(func: Function<()>, env: &mut TypeEnv) -> Result<Function<Type>, AnyError> {
    let mut new_env = env.clone();
    for (name, ty) in &func.args {
        new_env.insert(name.into(), ty.clone());
    }
    let new_body = infer_seq(func.body, &mut new_env)?;
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

fn infer_seq(seq: Sequence<()>, env: &mut TypeEnv) -> Result<Sequence<Type>, AnyError> {
    let new_exprs = infer_exprs(seq.0, env)?;
    Ok(Sequence(new_exprs))
}

fn infer_exprs(
    exprs: Vec<Expression<()>>,
    env: &mut TypeEnv,
) -> Result<Vec<Expression<Type>>, AnyError> {
    exprs
        .into_iter()
        .map(|e| infer_expr(e, env))
        .collect::<Result<Vec<Expression<Type>>, AnyError>>()
}

fn infer_expr(expr: Expression<()>, env: &mut TypeEnv) -> Result<Expression<Type>, AnyError> {
    let mut ty = Type::unit();
    let expr = match expr.expr {
        Expr::Unary { op, rhs } => {
            let rhs = infer_expr(*rhs, env)?;
            ty = infer_op(op, &rhs)?;
            Expr::Unary {
                op,
                rhs: Box::new(rhs),
            }
        }
        Expr::Binary {
            lhs,
            op: Operator::Assign,
            rhs,
        } => {
            let lhs = infer_expr(*lhs, env)?;
            let rhs = infer_expr(*rhs, env)?;
            ty = rhs.ty.clone();
            // Ensure that the types match
            lhs.ty
                .expect(&rhs.ty)
                .context("Binary operation type mismatch")?;
            // Ensure that the lhs is assignable
            if let Expr::Reference(name) = &lhs.expr {
                if let Some(ty) = env.get_mut(name) {
                    *ty = rhs.ty.clone();
                } else {
                    return Err(Error::VariableNotFound(name.clone()).into());
                }
            } else {
                return Err(anyhow!("Invalid assignment"));
            }
            Expr::Binary {
                lhs: Box::new(lhs),
                op: Operator::Assign,
                rhs: Box::new(rhs),
            }
        }
        Expr::Binary { lhs, op, rhs } => {
            let lhs = infer_expr(*lhs, env)?;
            let rhs = infer_expr(*rhs, env)?;
            ty = infer_op(op, &lhs)?;
            // Ensure that the types match
            lhs.ty
                .expect(&rhs.ty)
                .context("Binary operation type mismatch")?;
            Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        }
        Expr::Value(val) => match val {
            Value::Number(x) => {
                ty = Type::Signed(Size::ThirtyTwo);
                Expr::Value(Value::Number(x))
            }
            Value::Bool(b) => {
                ty = Type::Bool;
                Expr::Value(Value::Bool(b))
            }
            Value::Char(c) => {
                ty = Type::Char;
                Expr::Value(Value::Char(c))
            }
            Value::String(s) => {
                ty = Type::Array(Box::new(Type::Char), s.len());
                Expr::Value(Value::String(s))
            }
        },
        Expr::Tuple(exprs) => {
            let exprs = infer_exprs(exprs, env)?;
            ty = Type::Tuple(exprs.iter().map(|e| e.ty.clone()).collect());
            Expr::Tuple(exprs)
        }
        Expr::Array(exprs) => {
            let exprs = infer_exprs(exprs, env)?;
            ty = exprs
                .first()
                .map(|e| e.ty.clone())
                .ok_or_else(|| anyhow!("Empty array"))?; // TODO: Make this an unknown type
            for e in exprs.iter() {
                e.ty.expect(&ty).context("Array element type mismatch")?;
            }
            ty = Type::Array(Box::new(ty), exprs.len());
            Expr::Array(exprs)
        }
        Expr::Reference(x) => {
            let ty = env
                .get(x.as_str())
                .ok_or_else(|| Error::VariableNotFound(x.clone()))?
                .clone();
            Expr::Reference(x)
        }
        Expr::Let {
            name,
            value,
            ty: opt_ty,
            mutable,
        } => {
            let value = infer_expr(*value, env)?;
            ty = opt_ty.unwrap_or(value.ty.clone());
            ty.expect(&value.ty)
                .context(format!("Type mismatch for variable '{}'", name))?;
            env.insert(name.clone(), ty.clone());
            Expr::Let {
                name,
                value: Box::new(value),
                ty: Some(ty.clone()),
                mutable,
            }
        }
        Expr::If { cond, then, else_ } => {
            let cond = infer_expr(*cond, env)?;
            let then = infer_seq(then, env)?;
            let else_ = else_.map(|seq| infer_seq(seq, env)).transpose()?;
            cond.ty
                .expect(&Type::Bool)
                .context("If condition must be a boolean")?;
            ty = then.0.last().map_or(&Type::unit(), |e| &e.ty).clone();
            if let Some(else_) = &else_ {
                ty.expect(else_.0.last().map_or(&Type::unit(), |e| &e.ty))
                    .context("If-else branches must have the same type")?;
            }
            Expr::If {
                cond: Box::new(cond),
                then,
                else_,
            }
        }
        Expr::Call { func, args } => {
            let func = infer_expr(*func, env)?;
            let args = infer_exprs(args, env)?;
            let func_ty = func.ty.clone();

            if let Type::Function {
                args: arg_tys,
                ret: ret_ty,
            } = func_ty
            {
                for (arg, ty) in args.iter().zip(arg_tys) {
                    arg.ty.expect(&ty).context("Argument type mismatch")?;
                }
                ty = *ret_ty;
            } else {
                Err(Error::ExpectedFunction(func_ty))?;
            }
            Expr::Call {
                func: Box::new(func),
                args,
            }
        }
        Expr::While { cond, body } => {
            let cond = infer_expr(*cond, env)?;
            let body = infer_seq(body, env)?;
            cond.ty
                .expect(&Type::Bool)
                .context("While condition must be a boolean")?;
            Expr::While {
                cond: Box::new(cond),
                body,
            }
        }
    };
    Ok(Expression { expr, ty })
}

fn infer_op(op: Operator, expr: &Expression<Type>) -> Result<Type, AnyError> {
    match op {
        Operator::Add | Operator::Sub | Operator::Mul | Operator::Div => {
            expr.ty.expect(&Type::Signed(Size::ThirtyTwo))?;
            Ok(Type::Signed(Size::ThirtyTwo))
        }
        Operator::Assign => Ok(expr.ty.clone()),
        Operator::Eq | Operator::Neq | _ => Ok(Type::Bool),
        Operator::Lte => todo!(),
        Operator::Gte => todo!(),
        Operator::And => todo!(),
        Operator::Or => todo!(),
        Operator::Not => todo!(),
    }
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
