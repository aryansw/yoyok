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
    Unary {
        op: Operator,
        rhs: Box<Expression>,
    },
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
    While {
        cond: Box<Expression>,
        body: Sequence,
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
pub enum Type {
    Signed(Size),
    Unsigned(Size),
    Float(Size),
    Bool,
    Char,
    Tuple(Vec<Type>),
    Array(Box<Type>, usize),
    Function { args: Vec<Type>, ret: Box<Type> },
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

#[derive(Debug, Clone, PartialEq)]
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
