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

fn infer_unary(op: Operator, rhs: &Expression<Type>) -> Result<Type, AnyError> {
    let ty = &rhs.ty;
    match op {
        Operator::Not => {
            ty.expect(&Type::Bool)
                .context("Unary negation on non-boolean")?;
            Ok(Type::Bool)
        }
        Operator::Sub => {
            ty.expect(&Type::Signed(Size::ThirtyTwo))
                .context("Unary negation on non-integer")?;
            Ok(Type::Signed(Size::ThirtyTwo))
        }
        Operator::TupleIndex(i) => ty.map_mut(|ty| {
            if let Type::Tuple(tys) = ty {
                if i >= tys.len() {
                    Err(Error::InvalidTupleIndex(ty.clone()))?
                } else {
                    Ok(tys[i].clone())
                }
            } else {
                Err(Error::InvalidTupleIndex(ty.clone()))?
            }
        }),
        Operator::Ref => ty.map_mut(|ty| Ok(Type::Reference(Box::new(ty.clone())))),
        Operator::Mul => ty.map_mut(|ty| {
            if let Type::Reference(ty) = ty {
                Ok(*ty.clone())
            } else {
                Err(Error::InvalidDereference(ty.clone()))?
            }
        }),
        _ => Err(anyhow!("Invalid unary operator {:?}", op)),
    }
}

fn infer_binary(
    op: Operator,
    lhs: &mut Expression<Type>,
    rhs: &Expression<Type>,
) -> Result<Type, AnyError> {
    let lhs_ty = &lhs.ty;
    let rhs_ty = &rhs.ty;
    match op {
        Operator::Assign => {
            if let Type::Mutable(ty) = lhs_ty {
                rhs_ty
                    .expect(ty)
                    .context("Invalid operand types for assignment")?;
                lhs.ty = *ty.clone();
                Ok(Type::unit())
            } else {
                Err(Error::NotMutable(lhs_ty.clone()))?
            }
        }
        Operator::ArrayIndex => lhs_ty.map_mut(|ty| {
            if let Type::Array(ty, _) = ty {
                rhs_ty
                    .expect(&Type::Signed(Size::ThirtyTwo))
                    .context("Invalid index type for array")?;
                Ok(*ty.clone())
            } else {
                Err(Error::InvalidArrayIndex(ty.clone()))?
            }
        }),
        op if op.is_arith() => {
            lhs_ty
                .expect(rhs_ty)
                .context("Invalid operand types for arithmetic operator")?;
            lhs_ty
                .expect(&Type::Signed(Size::ThirtyTwo))
                .context("Invalid operand type for arithmetic operator")?;
            Ok(Type::Signed(Size::ThirtyTwo))
        }
        op if op.is_comparison() => {
            lhs_ty
                .expect(&Type::Signed(Size::ThirtyTwo))
                .context("Invalid operand type for comparison operator")?;
            Ok(Type::Bool)
        }
        op if op.is_logical() => {
            lhs_ty
                .expect(&Type::Bool)
                .context("Invalid operand type for logical operator")?;
            Ok(Type::Bool)
        }
        _ => Err(anyhow!("Invalid binary operator {:?}", op)),
    }
}

fn infer_expr(expr: Expression<()>, env: &mut TypeEnv) -> Result<Expression<Type>, AnyError> {
    let mut ty = Type::unit();
    let expr = match expr.expr {
        Expr::Unary { op, rhs } => {
            let rhs = infer_expr(*rhs, env)?;
            ty = infer_unary(op, &rhs).context(format!("On unary expression: ({} {})", op, rhs))?;
            Expr::Unary {
                op,
                rhs: Box::new(rhs),
            }
        }
        Expr::Binary { op, lhs, rhs } => {
            let mut lhs = infer_expr(*lhs, env)?;
            let rhs = infer_expr(*rhs, env)?;
            ty = infer_binary(op, &mut lhs, &rhs)
                .context(format!("On binary expression: ({} {} {})", lhs, op, rhs))?;
            Expr::Binary {
                op,
                lhs: Box::new(lhs),
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
            ty = env
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
            let value_ty = value.ty.pop_mut().clone();
            let ty = if let Some(ty) = opt_ty {
                ty.expect(&value_ty)
                    .context(format!("Type mismatch for variable '{}'", name))?;
                ty
            } else {
                value_ty
            };
            if env.contains_key(name.as_str()) {
                Err(Error::RedeclarationVariable(name.clone()))?
            }
            // Wrap this in a mutable if it can be reassigned later
            let ty = if mutable {
                Type::Mutable(Box::new(ty))
            } else {
                ty
            };

            env.insert(name.clone(), ty.clone());
            Expr::Let {
                name,
                value: Box::new(value),
                ty: Some(ty),
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
        if self.pop_mut() == ty.pop_mut() {
            Ok(())
        } else {
            Err(Error::UnexpectedType(self.clone(), ty.clone()).into())
        }
    }

    // Unpacks mutable values
    fn pop_mut(&self) -> &Type {
        match self {
            Type::Mutable(ty) => ty,
            _ => self,
        }
    }

    // Uses just one reference
    fn pop_ref(&self) -> &Type {
        match self {
            Type::Reference(ty) => ty,
            _ => self,
        }
    }

    // Wraps the type in a mutable reference if the type is mutable
    fn map_mut(&self, f: impl FnOnce(&Type) -> Result<Type, AnyError>) -> Result<Type, AnyError> {
        match self {
            Type::Mutable(ty) => {
                let ty = f(ty)?;

                match ty {
                    // if the type is a composite type, we lose the mutability
                    // This fix prevents mutable nested values from being created
                    // To actually fix this, we need to add a new type for non-mutable values
                    Type::Array(_, _) | Type::Tuple(_) => Ok(ty),
                    // if the type is already mutable, we don't need to wrap it again
                    Type::Mutable(_) => Ok(ty),
                    _ => Ok(Type::Mutable(Box::new(ty))),
                }
            }

            _ => f(self),
        }
    }
}
