use crate::ast::tree::{Function, Size, Type, TypeBound};

#[derive(Clone, Debug, PartialEq)]
pub enum Value<T: TypeBound> {
    Signed(i64),
    Bool(bool),
    Char(char),
    Tuple(Vec<Value<T>>),
    Array(Vec<Value<T>>),
    Function(Function<T>),
}

impl<T: TypeBound> Value<T> {
    pub fn type_of(&self) -> Type {
        match &self {
            Value::Signed(_) => Type::Signed(Size::ThirtyTwo),
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
