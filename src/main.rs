#![allow(unused)]
// Cool features
#![feature(let_chains)]
#![feature(assert_matches)]

use std::fs;

use crate::error::Error;
use anyhow::Context;
use clap::Parser;
use parser::parser::parse;

// Modules
mod ast;
mod error;
mod parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path for source file to run
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let source = fs::read_to_string(&args.file).context("Failed to read file")?;
    run_str(&source).context("Failed to compile program")?;
    Ok(())
}

// Run program from string
fn run_str(source: &str) -> Result<(), Error> {
    parse(source)?;
    Ok(())
}
