#![allow(unused)]
// Cool features
#![feature(let_chains)]
#![feature(assert_matches)]
#![feature(result_option_inspect)]

use std::fs;

use crate::error::Error;
use ::log::{debug, info};
use anyhow::Context;
use clap::Parser;
use colored::Colorize;
use parser::parser::parse;
use proptest::{
    prelude,
    test_runner::{Config, TestCaseError, TestRunner},
};

// Modules
mod ast;
mod error;
mod log;
mod parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path for source file to run (or run a random program if not specified)
    file: Option<String>,
    /// Logging messages
    #[clap(short, long, default_value = "false")]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    log::init(args.verbose).map_err(|_| Error::LogError)?;
    if let Some(src) = args.file {
        let source = fs::read_to_string(&src).context("Failed to read file")?;
        run_str(&source).context("Failed to compile program".bright_red())?;
    } else {
        info!("Running a random program");
        let mut runner = TestRunner::new(Config {
            cases: 1,
            ..Default::default()
        });
        let ast_strat = ast::proptest::arb_seq();
        let result = runner.run(&ast_strat, |ast| {
            run_str(format!("{}", ast).as_str()).map_err(|err| TestCaseError::Fail(err.into()))
        })?;
    };
    Ok(())
}

// Run program from string
fn run_str(source: &str) -> Result<(), Error> {
    let _ast = parse(source)?;
    Ok(())
}
