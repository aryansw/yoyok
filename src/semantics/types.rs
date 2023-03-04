#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Signed(Size),
    Bool,
    Char,
    Tuple(Vec<Type>),
    Array(Box<Type>, usize),
    Function { args: Vec<Type>, ret: Box<Type> },
    Reference(Box<Type>),
    // Hidden type that's used to present a mutable value
    Mutable(Box<Type>),
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
