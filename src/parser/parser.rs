use crate::{
    ast::ast::{Expression as Exp, Program},
    parser::error::Error,
    parser::tokens::TokenType::*,
};

use super::{scanner::Scanner, tokens::TokenType};

pub fn parse(src: &str) -> Result<Program, Error> {
    let mut scan = Scanner::new(src);
    let mut expr = Vec::new();

    macro_rules! expect {
        ($(|)? $( $pattern:pat_param )|+ $( if let $guard: expr )? $(,)?) => {
            match scan.next()?.token {
                $( $pattern )|+ $( if $guard )? => Ok(()),
                x => {
                    let pattern = stringify!($( $pattern )|+ $( if $guard )?).into();
                    Err(Error::UnexpectedToken(pattern, format!("{:?}", x)))
                },
            }
        };
    };

    fn parse_expr() -> Result<Exp, Error> {
        Ok(Exp::Number(5))
    }

    while scan.peek()?.token != EOF {
        expr.push(parse_expr()?);
        expect!(EOF);
    }
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
