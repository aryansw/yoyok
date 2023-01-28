use colored::Colorize;
use log::debug;

use crate::{
    ast::ast::{Expression as Exp, Operator, Sequence as Seq},
    parser::error::Error,
    parser::tokens::Keyword::*,
    parser::tokens::TokenType::*,
};

use super::scanner::Scanner;

// Peek at the next token, and return an error if it doesn't match the pattern
macro_rules! expect {
    ($scan:ident, $(|)? $( $pattern:pat_param )|+ $( if let $guard: expr )? $(,)?) => {
        {
            let tok = $scan.next()?;
            match &tok.token {
                $( $pattern )|+ $( if $guard )? => {
                    Ok(())
                },
                _x => {
                    let pattern = stringify!($( $pattern )|+ $( if $guard )?).into();
                    Err(Error::UnexpectedToken(pattern, tok))
                },
            }
        }
    };
}

// Minimum precedence of the next operator
fn parse_expr(scan: &mut Scanner, _min: u8) -> Result<Exp, Error> {
    let tok = scan.next()?;

    let mut expr = match tok.token {
        Number(n) => Exp::Number(n),
        Name(x) => Exp::Reference(x),
        Keyword(If) => {
            expect!(scan, Delim('('))?; // (
            let cond = Box::new(parse_expr(scan, 0)?);
            expect!(scan, Delim(')'))?; // )
            expect!(scan, Delim('{'))?; // {
            let then = parse_seq(scan)?;
            expect!(scan, Delim('}'))?; // }
            let else_ = if scan.peek()?.token == Keyword(Else) {
                scan.next()?; // else
                expect!(scan, Delim('{'))?; // {
                let else_ = parse_seq(scan)?;
                expect!(scan, Delim('}'))?; // }
                Some(else_)
            } else {
                None
            };
            Exp::If { cond, then, else_ }
        }
        Keyword(Else) => Err(Error::UnexpectedToken("".into(), tok))?,
        Keyword(ref key) => {
            let name = scan.next()?.name()?;
            expect!(scan, Op(x) if let ['='] == x[..])?;
            let value = Box::new(parse_expr(scan, 0)?);
            match key {
                Let => Exp::Let { name, value },
                Var => Exp::Var { name, value },
                _ => Err(Error::UnexpectedToken("".into(), tok))?,
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
                Operator::from(&x)?
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
pub fn parse_seq(scan: &mut Scanner) -> Result<Seq, Error> {
    let mut exprs = vec![parse_expr(scan, 0)?];
    while matches!(scan.peek()?.token, Delim(';')) {
        scan.next()?;
        exprs.push(parse_expr(scan, 0)?);
    }
    Ok(Seq(exprs))
}

pub fn parse(src: &str) -> Result<Seq, Error> {
    let scan = &mut Scanner::new(src);
    let expr = parse_seq(scan).map_err(|e| {
        match e {
            Error::UnexpectedToken(s, tok) => {
                let pos = tok.pos;
                // Figure out the line and column of the error
                let line = src[..pos].lines().count();
                let col = src[..pos].lines().last().unwrap().len();
                // Print the line with the error
                let line = src.lines().nth(line - 1).unwrap();
                let line = format!("{}", line.bright_yellow());
                // Print a caret under the error
                println!(
                    "{}\n{}\n",
                    line,
                    format!(
                        "{}{}{}",
                        " ".repeat(col),
                        "^".bright_red(),
                        format!(" {}", s).yellow()
                    )
                );
                Error::UnexpectedToken(s, tok)
            }
            _ => e,
        }
    })?;
    // Expect EOF
    expect!(scan, EOF)?;
    let prgm = expr;
    debug!(
        "{}\n{}",
        "AST:".bright_yellow(),
        format!("{}", prgm).bright_cyan()
    );
    Ok(prgm)
}
