use std::fmt::Display;

use super::ast::{Expression, Operator, Sequence, Size, Type, Value};

impl Expression {
    fn display(&self, idt: i32) -> String {
        match self {
            Self::Value(v) => format!("{}", v),
            Self::Reference(s) => format!("{}", s),
            Self::Binary { lhs, op, rhs } => match op {
                Operator::Assign => {
                    format!("{} {} {}", lhs.display(idt), op, rhs.display(idt))
                }
                _ => format!("({} {} {})", lhs.display(idt), op, rhs.display(idt)),
            },
            Self::Tuple(v) => {
                // If it's one element, then we need to add a comma to make it a tuple
                if v.len() == 1 {
                    return format!("({},)", v[0].display(idt));
                }
                let mut s = String::new();
                let mut it = v.iter().peekable();
                while let Some(expr) = it.next() {
                    if it.peek().is_none() {
                        s.push_str(&expr.display(idt));
                    } else {
                        s.push_str(&format!("{}, ", expr.display(idt)));
                    }
                }
                format!("({})", s)
            }
            Self::Array(v) => {
                let mut s = String::new();
                let mut it = v.iter().peekable();
                while let Some(expr) = it.next() {
                    if it.peek().is_none() {
                        s.push_str(&expr.display(idt));
                    } else {
                        s.push_str(&format!("{}, ", expr.display(idt)));
                    }
                }
                format!("[{}]", s)
            }
            Self::Let {
                name,
                ty,
                value,
                mutable,
            } => {
                let var = if *mutable { "var" } else { "let" };
                match ty {
                    Some(ty) => format!("{} {}: {} = {}", var, name, ty, value.display(idt)),
                    None => format!("{} {} = {}", var, name, value.display(idt)),
                }
            }
            Self::If { cond, then, else_ } => {
                let idt = idt + 2;
                let then = apply_indent(then.display(idt), 2);
                let else_ = else_.as_ref().map(|e| apply_indent(e.display(idt), 2));
                match else_ {
                    Some(e) => format!("if {} {{\n{}}}\nelse {{\n{}}}", cond.display(idt), then, e),
                    None => format!("if {} {{\n{}}}", cond.display(idt), then),
                }
            }
        }
    }
}

impl Sequence {
    fn display(&self, idt: i32) -> String {
        let mut s = String::new();
        let mut it = self.0.iter().peekable();
        while let Some(expr) = it.next() {
            if it.peek().is_none() {
                s.push_str(&expr.display(idt));
            } else {
                s.push_str(&format!("{};\n", expr.display(idt)));
            }
        }
        s
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display(0))
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display(0))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(x) => write!(f, "{}", x),
            Value::Bool(x) => write!(f, "{}", x),
            Value::Char(x) => write!(f, "'{}'", x),
            Value::String(x) => write!(f, "\"{}\"", x),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Assign => write!(f, "="),
            Operator::Gt => write!(f, ">"),
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Eight => write!(f, "8"),
            Size::Sixteen => write!(f, "16"),
            Size::ThirtyTwo => write!(f, "32"),
            Size::SixtyFour => write!(f, "64"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Signed(size) => write!(f, "i{}", size),
            Type::Unsigned(size) => write!(f, "u{}", size),
            Type::Float(size) => write!(f, "f{}", size),
            Type::Bool => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::Tuple(tys) => {
                let mut s = String::new();
                for ty in tys {
                    s.push_str(&format!("{}, ", ty));
                }
                s.pop();
                s.pop();
                write!(f, "({})", s)
            }
            Type::Array(ty, size) => write!(f, "[{}; {}]", ty, size),
            Type::Function { args, ret } => {
                write!(f, "{} -> {}", args, ret)
            }
        }
    }
}

fn apply_indent(str: String, idt: i32) -> String {
    // Apply same indentation to all lines
    let mut s = String::new();
    for line in str.lines() {
        s.push_str(&" ".repeat(idt as usize));
        s.push_str(line);
        s.push_str("\n");
    }
    s
}
