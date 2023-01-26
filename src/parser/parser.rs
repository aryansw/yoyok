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
            let tok = $scan.peek()?;
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

fn parse_expr(scan: &mut Scanner) -> Result<Exp, Error> {
    let tok = scan.next()?;
    let expr = match tok.token {
        Number(n) => Exp::Number(n),
        Name(x) => Exp::Reference(x),
        Keyword(key) => {
            let name = scan.next()?.name()?;
            expect!(scan, Op(x) if let ['='] == x[..])?;
            scan.next()?;
            let value = Box::new(parse_expr(scan)?);
            match key {
                Let => Exp::Let { name, value },
                Var => Exp::Var { name, value },
            }
        }
        _ => Err(Error::UnexpectedToken("".into(), tok))?,
    };
    if let Op(_) = scan.peek()?.token {
        let op = scan.next()?.to_op()?;
        let rhs = Box::new(parse_expr(scan)?);
        Ok(Exp::Binary {
            lhs: Box::new(expr),
            op,
            rhs,
        })
    } else {
        Ok(expr)
    }
}

pub fn parse(src: &str) -> Result<Program, Error> {
    let mut scan = &mut Scanner::new(src);
    let mut expr = Vec::new();

    while scan.peek()?.token != EOF {
        // Parse an expression (cannot use a function here 😔 cause borrow-checker stuff)
        expr.push(parse_expr(&mut scan)?);
        // Ensure that after an expression, there is either an EOF or a semicolon
        expect!(scan, EOF | Delim(';'))?;
        scan.next()?;
    }

    println!("{:?}", expr);
    Ok(expr)
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
