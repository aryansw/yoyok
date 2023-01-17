#![allow(unused)]
use crate::error::Error;
use anyhow::Context;
use clap::Parser;
use parser::parser::parse;

mod error;
mod parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path for source file to run
    file: String,
}

fn main() {
    let args = Args::parse();
    let source = std::fs::read_to_string(&args.file)
        .context("Failed to read file")
        .unwrap();
    run_str(&source)
        .context("Failed to compile program")
        .unwrap();
}

// Run program from string
fn run_str(source: &str) -> Result<(), Error> {
    parse(source)?;
    Ok(())
}
