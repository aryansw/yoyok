use crate::parser::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Function>);

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub ret: Type,
    pub body: Sequence,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sequence(pub Vec<Expression>);

// TODO: It might be worth making this generic over the type of the expression,
// so types can initially be optional, and then be inferred later.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Binary {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
    Value(Value),
    Tuple(Vec<Expression>),
    Array(Vec<Expression>),
    Reference(String),
    Let {
        name: String,
        value: Box<Expression>,
        ty: Option<Type>,
        mutable: bool,
    },
    If {
        cond: Box<Expression>,
        then: Sequence,
        else_: Option<Sequence>,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(u64),
    Bool(bool),
    Char(char),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Gt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Signed(Size),
    Unsigned(Size),
    Float(Size),
    Bool,
    Char,
    Tuple(Vec<Type>),
    Array(Box<Type>, u64),
    Function { args: Box<Type>, ret: Box<Type> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

impl Type {
    pub fn unit() -> Self {
        Type::Tuple(vec![])
    }
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
            _ => return Err(Error::InvalidOperator(op.iter().collect())),
        })
    }

    pub fn prec(&self) -> u8 {
        match self {
            Self::Add | Self::Sub => 1,
            Self::Mul | Self::Div => 2,
            Self::Assign | Self::Gt => 3,
        }
    }

    pub fn assoc(&self) -> u8 {
        match self {
            Self::Add | Self::Sub | Self::Mul | Self::Div => 1,
            Self::Assign | Self::Gt => 0,
        }
    }
}

impl TryInto<Size> for u8 {
    type Error = Error;

    fn try_into(self) -> Result<Size, Self::Error> {
        Ok(match self {
            8 => Size::Eight,
            16 => Size::Sixteen,
            32 => Size::ThirtyTwo,
            64 => Size::SixtyFour,
            _ => return Err(Error::InvalidSize(self)),
        })
    }
}

impl Into<Value> for u64 {
    fn into(self) -> Value {
        Value::Number(self)
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::Bool(self)
    }
}
