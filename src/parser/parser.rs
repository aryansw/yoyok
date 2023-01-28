use std::f32::consts::E;

use colored::Colorize;
use log::{debug, trace};

use crate::{
    ast::ast::{Expression as Exp, Operator},
    parser::error::Error,
    parser::tokens::Keyword::*,
    parser::tokens::TokenType::*,
};

use super::{scanner::Scanner, tokens::TokenType};

// Peek at the next token, and return an error if it doesn't match the pattern
macro_rules! expect {
    ($scan:ident, $(|)? $( $pattern:pat_param )|+ $( if let $guard: expr )? $(,)?) => {
        {
            let tok = $scan.next()?;
            match &tok.token {
                $( $pattern )|+ $( if $guard )? => {
                    Ok(())
                },
                x => {
                    let pattern = stringify!($( $pattern )|+ $( if $guard )?).into();
                    Err(Error::UnexpectedToken(pattern, tok))
                },
            }
        }
    };
}

// Minimum precedence of the next operator
fn parse_expr(scan: &mut Scanner, min: u8) -> Result<Exp, Error> {
    let tok = scan.next()?;

    let mut expr = match tok.token {
        Number(n) => Exp::Number(n),
        Name(x) => Exp::Reference(x),
        Keyword(If) => {
            expect!(scan, Delim('('))?; // (
            let cond = Box::new(parse_expr(scan, 0)?);
            expect!(scan, Delim(')'))?; // )
            expect!(scan, Delim('{'))?; // {
            let then = Box::new(parse_seq(scan)?);
            expect!(scan, Delim('}'))?; // }
            let else_ = if scan.peek()?.token == Keyword(Else) {
                scan.next()?; // else
                expect!(scan, Delim('{'))?; // {
                let else_ = Box::new(parse_seq(scan)?);
                expect!(scan, Delim('}'))?; // }
                Some(else_)
            } else {
                None
            };
            Exp::If { cond, then, else_ }
        }
        Keyword(key) => {
            let name = scan.next()?.name()?;
            expect!(scan, Op(x) if let ['='] == x[..])?;
            let value = Box::new(parse_expr(scan, 0)?);
            match key {
                Let => Exp::Let { name, value },
                Var => Exp::Var { name, value },
                _ => unreachable!(),
            }
        }
        Delim('(') => {
            let expr = parse_expr(scan, 0)?;
            expect!(scan, Delim(')'))?;
            expr
        }
        _ => Err(Error::UnexpectedToken("".into(), tok))?,
    };

    // Operator Parsing (with Precedence Climbing)
    loop {
        let op = match scan.peek()?.token {
            Op(x) => {
                scan.next()?;
                Operator::from(&x)
            }
            _ => break,
        };
        let rhs = parse_expr(scan, op.prec() + op.assoc())?;
        expr = Exp::Binary {
            lhs: Box::new(expr),
            op,
            rhs: Box::new(rhs),
        };
    }
    Ok(expr)
}

// As long as we see a semicolon, there's a sequence of expressions
pub fn parse_seq(scan: &mut Scanner) -> Result<Exp, Error> {
    let mut exprs = vec![parse_expr(scan, 0)?];
    while matches!(scan.peek()?.token, Delim(';')) {
        scan.next()?;
        exprs.push(parse_expr(scan, 0)?);
    }
    Ok(Exp::Sequence(exprs))
}

pub fn parse(src: &str) -> Result<Exp, Error> {
    let mut scan = &mut Scanner::new(src);
    let mut expr = parse_seq(scan)?;
    let prgm = expr;
    debug!(
        "{}\n{}",
        "AST:".bright_yellow(),
        format!("{}", prgm).bright_cyan()
    );
    Ok(prgm)
}
