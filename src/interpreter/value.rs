use itertools::Itertools;

use crate::ast::ast::{Function, Size, Type};

pub enum Value {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    Bool(bool),
    Char(char),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
    Function(Function),
}

impl Value {
    pub fn type_of(&self) -> Type {
        match &self {
            Value::Signed(_) => Type::Signed(Size::SixtyFour),
            Value::Unsigned(_) => Type::Unsigned(Size::SixtyFour),
            Value::Float(_) => Type::Float(Size::SixtyFour),
            Value::Bool(_) => Type::Bool,
            Value::Char(_) => Type::Char,
            Value::Tuple(x) => Type::Tuple(x.iter().map(|x| x.type_of()).collect()),
            Value::Array(x) => Type::Array(Box::new(Type::Char), x.len()),
            Value::Function(x) => Type::Function {
                args: x.args.iter().map(|x| x.1.clone()).collect(),
                ret: Box::new(x.ret.clone()),
            },
        }
    }
}
