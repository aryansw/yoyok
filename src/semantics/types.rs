#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Signed(Size),
    Bool,
    Char,
    Tuple(Vec<Type>),
    Array(Box<Type>, usize),
    Function { args: Vec<Type>, ret: Box<Type> },
    Reference(Box<Type>),
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
