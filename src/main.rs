// Cool features
#![feature(let_chains)]
#![feature(assert_matches)]
#![feature(result_option_inspect)]
#![feature(trivial_bounds)]
#![feature(box_patterns)]
// Cargo Clippy settings
#![allow(dead_code, unused_variables)]

// Crates
use crate::error::Error;
use ::log::debug;
use ::log::info;
use anyhow::Context;
use anyhow::Error as AnyError;
use clap::Parser;
use colored::Colorize;
use interpreter::run::run_program;
use parser::parse::parse;
use proptest::test_runner::Reason;
use proptest::test_runner::{Config, TestCaseError, TestRunner};
use semantics::typeinfer::infer;
use std::fs;

// Modules
mod ast;
mod error;
mod interpreter;
mod log;
mod parser;
mod semantics;

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
    log::init(args.verbose).map_err(|_| Error::Log)?;
    if let Some(src) = args.file {
        let source = fs::read_to_string(src).context("Failed to read file")?;
        run_str(&source).context("Failed to run program".bright_red())?;
    } else {
        info!("Running a random program");
        let mut runner = TestRunner::new(Config {
            cases: 1,
            ..Default::default()
        });
        let ast_strat = ast::proptest::arb_prgm();
        runner.run(&ast_strat, |ast| {
            run_str(format!("{}", ast).as_str()).map_err(|err| TestCaseError::Fail(into(err)))
        })?;
    };
    Ok(())
}

// Run program from string
fn run_str(source: &str) -> Result<(), AnyError> {
    let ast = parse(source).context("Error while parsing")?;
    debug!(
        "{}\n{}",
        "Untyped AST:".bright_yellow(),
        format!("{}", ast).bright_cyan()
    );
    let ast = infer(ast).context("Error while inferring types")?;
    debug!(
        "\n{}\n{}",
        "Typed AST:".bright_yellow(),
        format!("{}", ast).bright_cyan()
    );
    run_program(ast).context("Error while running program")?;
    Ok(())
}

fn into(error: AnyError) -> Reason {
    error.to_string().into()
}
