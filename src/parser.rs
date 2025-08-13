use crate::{FellowError, FellowValue};
use crate::token::{Token, TokenContext};

pub trait Expr {

}

pub fn parse(tokens: Vec<TokenContext>) -> Result<Box<dyn Expr>, FellowError> {
    match tokens
        .into_iter()
        .filter(|t| !t.token.is_whitespace())
        .next_back()
    {
        Some(v) => Ok(parse_token(v)),
        None => Err(FellowError::InterpreterError),
    }
}

fn parse_token(token_context: TokenContext) -> FellowValue {
    match token_context.token {
        Token::True => FellowValue::Boolean(true),
        Token::False => FellowValue::Boolean(false),
        Token::String(s) => FellowValue::String(s),
        Token::Integer(i) => FellowValue::Int(i),
        _ => FellowValue::Nil,
    }
}
