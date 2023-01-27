use std::fmt::Display;

use super::ast::{Expression, Operator, Program};

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.0.iter().peekable();
        while let Some(expr) = it.next() {
            if it.peek().is_none() {
                write!(f, "{}", expr);
            } else {
                writeln!(f, "{};", expr)?;
            }
        }
        Ok(())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Reference(x) => write!(f, "{}", x),
            Expression::Let { name, value } => write!(f, "let {} = {}", name, value),
            Expression::Var { name, value } => write!(f, "var {} = {}", name, value),
            Expression::Binary { op, lhs, rhs } => match op {
                Operator::Assign => write!(f, "{} {} {}", lhs, op, rhs),
                _ => write!(f, "({} {} {})", lhs, op, rhs),
            },
            Expression::If { cond, then, else_ } => match else_ {
                Some(else_) => write!(f, "if ({}) {{ {} }} else {{ {} }}", cond, then, else_),
                None => write!(f, "if ({}) {{ {} }}", cond, then),
            },
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
        }
    }
}
