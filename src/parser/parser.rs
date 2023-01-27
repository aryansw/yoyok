use std::f32::consts::E;

use crate::{
    ast::ast::{Expression as Exp, Operator, Program},
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
        Keyword(key) => {
            let name = scan.next()?.name()?;
            expect!(scan, Op(x) if let ['='] == x[..])?;
            let value = Box::new(parse_expr(scan, 0)?);
            match key {
                Let => Exp::Let { name, value },
                Var => Exp::Var { name, value },
            }
        }
        Delim('(') => {
            let expr = parse_expr(scan, 0)?;
            expect!(scan, Delim(')'))?;
            expr
        }
        _ => Err(Error::UnexpectedToken("".into(), tok))?,
    };
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

pub fn parse(src: &str) -> Result<Program, Error> {
    let mut scan = &mut Scanner::new(src);
    let mut expr = Vec::new();

    while scan.peek()?.token != EOF {
        expr.push(parse_expr(&mut scan, 0)?);
        expect!(scan, EOF | Delim(';'))?;
    }

    println!("{:?}", expr);
    Ok(Program(expr))
}

fn debug_print(src: &str) -> Result<(), Error> {
    let mut scan = Scanner::new(src);
    println!("Program:\n\n{}\n\n", src);
    let mut tok = scan.next()?;
    println!("Tokens:\n");
    while !matches!(tok.token, TokenType::EOF) {
        println!("{:?}", tok.token);
        tok = scan.peek()?;
        scan.next()?;
    }
    Ok(())
}
