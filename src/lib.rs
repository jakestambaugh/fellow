use std::error::Error;
use std::fmt::{self, Display};
use std::string::FromUtf8Error;

mod scanner;
mod parser;
mod token;

use crate::scanner::scan;
use crate::token::{Token, TokenContext};
use crate::parser::{Expr, parse};

#[derive(Debug)]
pub struct ScanError {
    message: String,
    line: usize,
    position: usize,
}

#[derive(Debug)]
pub enum FellowError {
    CannotReadFile,
    InterpreterError,
    ScanError(ScanError),
}

impl Error for FellowError {}

impl From<FromUtf8Error> for FellowError {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        // TODO: fix this up, get some context
        Self::ScanError(ScanError {
            message: "".to_string(),
            line: 0,
            position: 0,
        })
    }
}

impl fmt::Display for FellowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::CannotReadFile => write!(f, "Cannot read file"),
            Self::InterpreterError => write!(f, "Fellow interpreter error"),
            Self::ScanError(err) => write!(
                f,
                "Fellow scanner error occured at line {}:{}\n\t{}",
                err.line, err.position, err.message
            ),
        }
    }
}

pub enum FellowValue {
    Int(i64),
    String(String),
    Identifier(String),
    Boolean(bool),
    Nil,
}

impl Display for FellowValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{:?}", i),
            Self::String(s) => write!(f, "{}", s),
            Self::Identifier(s) => write!(f, "{}", s),
            Self::Boolean(b) => write!(f, "{:?}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}

fn evaluate(ast: Box<dyn Expr>) -> FellowValue {
    // For now, we just return the last token's value
    // In a real interpreter, this would involve more complex evaluation logic
    if let Some(last_token) = ast.last() {
        FellowValue::String("Something".to_string())
    } else {
        FellowValue::Nil
    }
}

// Pipeline for our interpreter
pub fn interpret(source_code: &str) -> Result<FellowValue, FellowError> {
    let tokens = scan(source_code)?;
    let ast = match parse(tokens) {
        Ok(value) => Ok(value),
        Err(_) => Err(FellowError::InterpreterError),
    };
    Ok(evaluate(ast?))
}

