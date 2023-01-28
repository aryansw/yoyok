#![allow(unused)]
// Cool features
#![feature(let_chains)]
#![feature(assert_matches)]
#![feature(result_option_inspect)]

use std::fs;

use crate::error::Error;
use anyhow::Context;
use clap::Parser;
use colored::Colorize;
use parser::parser::parse;

// Modules
mod ast;
mod error;
mod log;
mod parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path for source file to run
    file: String,
    // Logging messages
    #[clap(short, long, default_value = "false")]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    log::init(args.verbose).map_err(|_| Error::LogError)?;
    let source = fs::read_to_string(&args.file).context("Failed to read file")?;
    run_str(&source).context("Failed to compile program".bright_red())?;
    Ok(())
}

// Run program from string
fn run_str(source: &str) -> Result<(), Error> {
    let _ast = parse(source)?;
    Ok(())
}
