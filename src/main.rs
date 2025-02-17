use std::error::Error;
use std::fmt::{self, Display};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser, value_name = "SCRIPT")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.path);

    match read_source_code(&args.path) {
        Ok(contents) => match interpret(contents) {
            Ok(value) => println!("{}", value),
            Err(err) => eprintln!("Error {:?}", err),
        },
        Err(err) => eprintln!("Failed to read source code from {:?}", err),
    }
}

#[derive(Debug)]
enum FellowError {
    CannotReadFile,
    InterpreterError,
}

impl Error for FellowError {}

impl fmt::Display for FellowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fellow error occurred")
    }
}

fn read_source_code(path: &PathBuf) -> Result<String, FellowError> {
    match std::fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(_err) => Err(FellowError::CannotReadFile),
    }
}

enum FellowValue {
    Int(i64),
    String(String),
}

impl Display for FellowValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{:?}", i),
            Self::String(s) => write!(f, "{}", s),
        }
    }
}

// Pipeline for our interpreter
fn interpret(source_code: String) -> Result<FellowValue, FellowError> {
    let tokens = source_code.split_whitespace();
    match tokens.last() {
        Some(v) => Ok(parse_token(v)),
        None => Err(FellowError::InterpreterError),
    }
}

fn parse_token(token: &str) -> FellowValue {
    if let Ok(i) = token.parse::<i64>() {
        FellowValue::Int(i)
    } else {
        FellowValue::String(token.to_string())
    }
}
