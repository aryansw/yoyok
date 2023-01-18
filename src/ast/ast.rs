pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
}

pub enum Expression {
    Binary {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>,
    },
    Literal {
        value: i64,
    },
    Reference {
        name: String,
    },
}
