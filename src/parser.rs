use crate::token::{Token, TokenContext};
use crate::{FellowError, FellowValue};

// Marker trait until we know what Expr needs
pub enum Expr {
    ValueExpr(FellowValue),
}

pub fn parse(tokens: Vec<TokenContext>) -> Result<Expr, FellowError> {
    match tokens
        .into_iter()
        .filter(|t| !t.token.is_whitespace())
        .next_back()
    {
        Some(v) => Ok(parse_token(v)),
        None => Err(FellowError::InterpreterError),
    }
}

fn parse_token(token_context: TokenContext) -> Expr {
    Expr::ValueExpr(match token_context.token {
        Token::True => FellowValue::Boolean(true),
        Token::False => FellowValue::Boolean(false),
        Token::String(s) => FellowValue::String(s),
        Token::Number(i) => FellowValue::Int(i),
        _ => FellowValue::Nil,
    })
}
