use std::fmt::Display;

use super::ast::{Expression, Function, Operator, Program, Sequence, Size, Type, Value};

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|func| write!(f, "{}", func))
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self
            .args
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty))
            .collect::<Vec<String>>()
            .join(", ");
        let body = apply_indent(format!("{}", self.body), 2);
        write!(
            f,
            "fn {}({}) -> {} {{\n{}}}",
            self.name, args, self.ret, body
        )
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.0.iter().peekable();
        while let Some(expr) = it.next() {
            if it.peek().is_none() {
                write!(f, "{}", &expr)?;
            } else {
                write!(f, "{};\n", expr)?;
            }
        }
        Ok(())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Reference(s) => write!(f, "{}", s),
            Self::Unary { op, rhs } => write!(f, "({}{})", op, rhs),
            Self::Binary { lhs, op, rhs } => match op {
                Operator::Assign => {
                    write!(f, "{} {} {}", lhs, op, rhs)
                }
                _ => write!(f, "({} {} {})", lhs, op, rhs),
            },
            Self::Tuple(v) => {
                // If it's one element, then we need to add a comma to make it a tuple
                if v.len() == 1 {
                    write!(f, "({},)", v[0])
                } else {
                    write!(f, "(")?;
                    let mut it = v.iter().peekable();
                    while let Some(expr) = it.next() {
                        if it.peek().is_none() {
                            write!(f, "{}", expr)?;
                        } else {
                            write!(f, "{}, ", expr)?;
                        }
                    }
                    write!(f, ")")
                }
            }
            Self::Array(v) => {
                write!(f, "[")?;
                let mut it = v.iter().peekable();
                while let Some(expr) = it.next() {
                    if it.peek().is_none() {
                        write!(f, "{}", expr)?;
                    } else {
                        write!(f, "{}, ", expr)?;
                    }
                }
                write!(f, "]")
            }
            Self::Let {
                name,
                ty,
                value,
                mutable,
            } => {
                let var = if *mutable { "var" } else { "let" };
                match ty {
                    Some(ty) => write!(f, "{} {}: {} = {}", var, name, ty, value),
                    None => write!(f, "{} {} = {}", var, name, value),
                }
            }
            Self::If { cond, then, else_ } => {
                let then = apply_indent(format!("{}", then), 2);
                let else_ = else_.as_ref().map(|e| apply_indent(format!("{}", e), 2));
                match else_ {
                    Some(e) => write!(f, "if {} {{\n{}}} else {{\n{}}}", cond, then, e),
                    None => write!(f, "if {} {{\n{}}}", cond, then),
                }
            }
            Self::Call { func, args } => {
                write!(f, "{}(", func)?;
                let mut it = args.iter().peekable();
                while let Some(expr) = it.next() {
                    if it.peek().is_none() {
                        write!(f, "{}", expr)?;
                    } else {
                        write!(f, "{}, ", expr)?;
                    }
                }
                write!(f, ")")
            }
            Self::While { cond, body } => {
                let body = apply_indent(format!("{}", body), 2);
                write!(f, "while {} {{\n{}}}", cond, body)
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(x) => write!(f, "{}", x),
            Value::Bool(x) => write!(f, "{}", x),
            Value::Char(x) => write!(f, "'{}'", x.escape_default()),
            Value::String(x) => write!(f, "\"{}\"", x.escape_default()),
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
            Operator::Lt => write!(f, "<"),
            Operator::Gte => write!(f, ">="),
            Operator::Lte => write!(f, "<="),
            Operator::Eq => write!(f, "=="),
            Operator::Neq => write!(f, "!="),
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),
            Operator::Not => write!(f, "!"),
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
