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
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>,
    },
    Number(i64),
    Reference(String),
    VarDec {
        name: String,
        expr: Box<Expression>,
    },
    Constant {
        name: String,
        expr: Box<Expression>,
    },
}

pub type Program = Vec<Expression>;
