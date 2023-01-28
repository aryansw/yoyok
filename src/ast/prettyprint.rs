use std::fmt::Display;

use super::ast::{Expression, Operator};

impl Expression {
    fn display(&self, idt: i32) -> String {
        match self {
            Self::Number(n) => format!("{}", n),
            Self::Reference(s) => format!("{}", s),
            Self::Binary { lhs, op, rhs } => match op {
                Operator::Assign => {
                    format!("{} {} {}", lhs.display(idt), op, rhs.display(idt))
                }
                _ => format!("({} {} {})", lhs.display(idt), op, rhs.display(idt)),
            },
            Self::Var { name, value } => format!("var {} = {}", name, value.display(idt)),
            Self::Let { name, value } => format!("let {} = {}", name, value.display(idt)),
            Self::If { cond, then, else_ } => {
                let idt = idt + 2;
                let then = apply_indent(then.display(idt), 2);
                let else_ = else_.as_ref().map(|e| apply_indent(e.display(idt), 2));
                match else_ {
                    Some(e) => format!("if {} {{\n{}}}\nelse {{\n{}}}", cond.display(idt), then, e),
                    None => format!("if {} {{\n{}}}", cond.display(idt), then),
                }
            }
            Self::Sequence(seq) => {
                let mut s = String::new();
                let mut it = seq.iter().peekable();
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
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display(0))
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
