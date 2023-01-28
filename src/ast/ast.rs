#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Gt,
}

#[derive(Debug)]
pub enum Expression {
    Binary {
        lhs: Box<Expression>,
        op: Operator,
        rhs: Box<Expression>,
    },
    Number(i64),
    Reference(String),
    Var {
        name: String,
        value: Box<Expression>,
    },
    Let {
        name: String,
        value: Box<Expression>,
    },
    If {
        cond: Box<Expression>,
        then: Box<Expression>,
        else_: Option<Box<Expression>>,
    },
    Sequence(Vec<Expression>),
}

impl Operator {
    pub fn from(op: &[char]) -> Self {
        match op {
            ['+'] => Operator::Add,
            ['-'] => Operator::Sub,
            ['*'] => Operator::Mul,
            ['/'] => Operator::Div,
            ['='] => Operator::Assign,
            ['>'] => Operator::Gt,
            _ => panic!("Invalid operator"),
        }
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
