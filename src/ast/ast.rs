use std::fmt::Display;

use crate::parser::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Program<T: TypeBound>(pub Vec<Function<T>>);

#[derive(Debug, Clone, PartialEq)]
pub struct Function<T: TypeBound> {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub ret: Type,
    pub body: Sequence<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sequence<T: TypeBound>(pub Vec<Expression<T>>);

impl From<Expr<()>> for Expression<()> {
    fn from(expr: Expr<()>) -> Self {
        Expression { expr, ty: () }
    }
}

// TODO: It might be worth making this generic over the type of the expression,
// so types can initially be optional, and then be inferred later.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression<T: TypeBound> {
    pub expr: Expr<T>,
    pub ty: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<T: TypeBound> {
    Unary {
        op: Operator,
        rhs: Box<Expression<T>>,
    },
    Binary {
        lhs: Box<Expression<T>>,
        op: Operator,
        rhs: Box<Expression<T>>,
    },
    Value(Value),
    Tuple(Vec<Expression<T>>),
    Array(Vec<Expression<T>>),
    Reference(String),
    Let {
        name: String,
        value: Box<Expression<T>>,
        ty: Option<Type>,
        mutable: bool,
    },
    If {
        cond: Box<Expression<T>>,
        then: Sequence<T>,
        else_: Option<Sequence<T>>,
    },
    Call {
        func: Box<Expression<T>>,
        args: Vec<Expression<T>>,
    },
    While {
        cond: Box<Expression<T>>,
        body: Sequence<T>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(i64),
    Bool(bool),
    Char(char),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Signed(Size),
    Bool,
    Char,
    Tuple(Vec<Type>),
    Array(Box<Type>, usize),
    Function { args: Vec<Type>, ret: Box<Type> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    ThirtyTwo,
}

impl Type {
    pub fn unit() -> Self {
        Type::Tuple(vec![])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Gt,
    Lt,
    Lte,
    Gte,
    Eq,
    Neq,
    And,
    Or,
    Not,
}

impl Operator {
    pub fn from(op: &[char]) -> Result<Self, Error> {
        Ok(match op {
            ['+'] => Operator::Add,
            ['-'] => Operator::Sub,
            ['*'] => Operator::Mul,
            ['/'] => Operator::Div,
            ['='] => Operator::Assign,
            ['>'] => Operator::Gt,
            ['<'] => Operator::Lt,
            ['<', '='] => Operator::Lte,
            ['>', '='] => Operator::Gte,
            ['=', '='] => Operator::Eq,
            ['!', '='] => Operator::Neq,
            ['&', '&'] => Operator::And,
            ['|', '|'] => Operator::Or,
            ['!'] => Operator::Not,
            _ => return Err(Error::InvalidOperator(op.iter().collect())),
        })
    }

    fn is_binary(&self) -> bool {
        match self {
            Self::Not => false,
            _ => true,
        }
    }

    fn is_unary(&self) -> bool {
        match self {
            Self::Not | Self::Sub => true,
            _ => false,
        }
    }

    pub fn expect_binary(&self) -> Result<(), Error> {
        if self.is_binary() {
            Ok(())
        } else {
            Err(Error::InvalidOperator(format!("{:?}", self)))
        }
    }

    pub fn expect_unary(&self) -> Result<(), Error> {
        if self.is_unary() {
            Ok(())
        } else {
            Err(Error::InvalidOperator(format!("{:?}", self)))
        }
    }

    pub fn prec(&self) -> u8 {
        match self {
            Self::Not => 0,
            Self::Add | Self::Sub => 1,
            Self::Mul | Self::Div => 2,
            x if x.is_comparison() => 3,
            x if x.is_logical() => 4,
            Self::Assign | _ => 5,
        }
    }

    pub fn assoc(&self) -> u8 {
        match self {
            x if x.is_arith() => 1,
            x if x.is_comparison() => 1,
            x if x.is_logical() => 1,
            Self::Assign | _ => 0,
        }
    }

    pub fn is_arith(&self) -> bool {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div => true,
            _ => false,
        }
    }

    pub fn is_comparison(&self) -> bool {
        match self {
            Self::Gt | Self::Lt | Self::Lte | Self::Gte | Self::Eq | Self::Neq => true,
            _ => false,
        }
    }

    pub fn is_logical(&self) -> bool {
        match self {
            Self::And | Self::Or | Self::Not => true,
            _ => false,
        }
    }
}

impl TryInto<Size> for u8 {
    type Error = Error;

    fn try_into(self) -> Result<Size, Self::Error> {
        Ok(match self {
            32 => Size::ThirtyTwo,
            _ => return Err(Error::InvalidSize(self)),
        })
    }
}

impl Into<Value> for u64 {
    fn into(self) -> Value {
        Value::Number(self as i64)
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::Bool(self)
    }
}

// These are the types that programs can take up. They can either have no type assigned to them, or (after type inference) they need to have a type assigned to them.
pub trait TypeBound
where
    Self: Sized + Clone + PartialEq + std::fmt::Debug,
{
}

impl TypeBound for () {}
impl TypeBound for Type {}
