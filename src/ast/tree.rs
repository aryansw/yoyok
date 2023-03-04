use crate::{
    parser::error::Error,
    semantics::types::{Size, Type},
};

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
    TupleIndex(usize),
    ArrayIndex,
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
        !matches!(self, Self::Not | Self::TupleIndex(_))
    }

    fn is_unary(&self) -> bool {
        matches!(self, Self::Not | Self::Sub | Self::TupleIndex(_))
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
            _ => 5,
        }
    }

    pub fn assoc(&self) -> u8 {
        match self {
            x if x.is_arith() => 1,
            x if x.is_comparison() => 1,
            x if x.is_logical() => 1,
            _ => 0,
        }
    }

    pub fn is_arith(&self) -> bool {
        matches!(self, Self::Add | Self::Sub | Self::Mul | Self::Div)
    }

    pub fn is_comparison(&self) -> bool {
        matches!(
            self,
            Self::Gt | Self::Lt | Self::Lte | Self::Gte | Self::Eq | Self::Neq
        )
    }

    pub fn is_logical(&self) -> bool {
        matches!(self, Self::And | Self::Or | Self::Not)
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

impl From<u64> for Value {
    fn from(val: u64) -> Self {
        Value::Number(val as i64)
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Value::Bool(val)
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
