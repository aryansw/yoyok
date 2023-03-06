#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Signed(Size),
    Bool,
    Char,
    Tuple(Vec<Type>),
    Array(Box<Type>, usize),
    Function { args: Vec<Type>, ret: Box<Type> },
    // Reference needs to have a bool to indicate mutability
    // This needs to be indicated by the parser, and then the type checker.
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
