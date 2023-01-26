#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
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
}

pub type Program = Vec<Expression>;

impl Operator {
    pub fn from(op: &[char]) -> Self {
        match op {
            ['+'] => Operator::Add,
            ['-'] => Operator::Sub,
            ['*'] => Operator::Mul,
            ['/'] => Operator::Div,
            ['='] => Operator::Assign,
            _ => panic!("Invalid operator"),
        }
    }
}
