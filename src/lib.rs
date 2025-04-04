use std::error::Error;
use std::fmt::{self, Display};

mod scanner;
mod token;

use crate::scanner::scan;
use crate::token::{Token, TokenContext};

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

// Pipeline for our interpreter
pub fn interpret(source_code: &str) -> Result<FellowValue, FellowError> {
    let tokens = scan(source_code)?;
    match tokens.into_iter().last() {
        Some(v) => Ok(parse_token(v)),
        None => Err(FellowError::InterpreterError),
    }
}

fn parse_token(token_context: TokenContext) -> FellowValue {
    match token_context.token {
        Token::True => FellowValue::Boolean(true),
        Token::False => FellowValue::Boolean(false),
        Token::String(s) => FellowValue::String(s.clone()),
        Token::Number(i) => FellowValue::Int(i.clone().parse().unwrap()),
        _ => FellowValue::Nil,
    }
}
