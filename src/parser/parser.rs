use colored::Colorize;
use log::debug;

use crate::{
    ast::ast::{Expression as Exp, Operator, Sequence as Seq, Type},
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

fn parse_opt_type(scan: &mut Scanner) -> Result<Option<Type>, Error> {
    if let Delim(':') = scan.peek()?.token {
        expect!(scan, Delim(':'))?;
        Ok(Some(parse_type(scan)?))
    } else {
        Ok(None)
    }
}

fn parse_type(scan: &mut Scanner) -> Result<Type, Error> {
    let tok = scan.next()?;
    let ty = match tok.token {
        Name(ref x) => {
            // First check if the beginning is i, u, f
            let ty = match &x[..1] {
                "i" | "u" | "f" => {
                    let size = x[1..].parse::<u8>()?.try_into()?;
                    match &x[..1] {
                        "i" => Type::Signed(size),
                        "u" => Type::Unsigned(size),
                        "f" => Type::Float(size),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    match x.as_str() {
                        "bool" => Type::Bool,
                        "char" => Type::Char,
                        _ => Err(Error::InvalidType(tok))?
                    }
                },
            };
            ty
        }
        Delim('(') => {
            let mut types = vec![];
            loop {
                if let Delim(')') = scan.peek()?.token {
                    expect!(scan, Delim(')'))?;
                    break;
                }
                types.push(parse_type(scan)?);
                if let Delim(',') = scan.peek()?.token {
                    scan.next()?;
                }
            }
            Type::Tuple(types)
        }
        Delim('[') => {
            let ty = parse_type(scan)?;
            expect!(scan, Delim(';'))?;
            let size = scan.next()?.number()?;
            expect!(scan, Delim(']'))?;
            Type::Array(Box::new(ty), size)
        }
        _ => Err(Error::InvalidType(tok))?,
    };
    if let Op(x) = scan.peek()?.token && let ['-', '>'] = x[..] {
        scan.next()?;
        let ret = parse_type(scan)?;
        Ok(Type::Function {
            args: Box::new(ty),
            ret: Box::new(ret),
        })
    } else { 
        Ok(ty)
    }
}

// Minimum precedence of the next operator
fn parse_expr(scan: &mut Scanner, _min: u8) -> Result<Exp, Error> {
    let tok = scan.next()?;

    let mut expr = match tok.token {
        Number(n) => Exp::Number(n),
        Name(x) => Exp::Reference(x),
        Keyword(If) => {
            let cond = Box::new(parse_expr(scan, 0)?);
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
        Keyword(True) => Exp::Bool(true),
        Keyword(False) => Exp::Bool(false),
        Keyword(Else) => Err(Error::UnexpectedToken("".into(), tok))?,
        Keyword(ref key) => {
            let name = scan.next()?.name()?;
            let ty = parse_opt_type(scan)?;
            expect!(scan, Op(x) if let ['='] == x[..])?;
            let value = Box::new(parse_expr(scan, 0)?);
            let mutable = match key {
                Let => true,
                Var => false,
                _ => Err(Error::UnexpectedToken("".into(), tok))?,
            };
            Exp::Let {
                name,
                value,
                ty,
                mutable,
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
    let expr = parse_seq(scan)?;
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
