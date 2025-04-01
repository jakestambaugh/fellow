use crate::{
    FellowError, ScanError,
    token::{Token, TokenContext},
};

use unicode_segmentation::UnicodeSegmentation;

/// The state of the scan is held in a struct. Helper functions can operate
/// on the state to figure out which tokens to emit
pub struct ScanState<'a> {
    source: Vec<&'a str>,
    lexeme_start: usize,
    current_grapheme: usize,
    current_line: usize,
}

impl<'a> ScanState<'a> {
    fn new(source_code: &'a str) -> Self {
        Self {
            source: source_code.graphemes(true).collect(),
            lexeme_start: 0,
            current_grapheme: 0,
            current_line: 0,
        }
    }

    fn mark_lexeme_start(&mut self) {
        self.lexeme_start = self.current_grapheme;
    }

    fn is_at_end(&self) -> bool {
        self.current_grapheme >= self.source.len()
    }

    // Advances the scanner and emits the next token
    fn scan_token(&mut self) -> Result<TokenContext, FellowError> {
        let c = self.source[self.current_grapheme];
        self.current_grapheme += 1;
        match c {
            "(" => self.contextualize(Token::LeftParen),
            ")" => self.contextualize(Token::RightParen),
            "{" => self.contextualize(Token::LeftBrace),
            "}" => self.contextualize(Token::RightBrace),
            "," => self.contextualize(Token::Comma),
            "." => self.contextualize(Token::Dot),
            "-" => self.contextualize(Token::Minus),
            "+" => self.contextualize(Token::Plus),
            ";" => self.contextualize(Token::Semicolon),
            "*" => self.contextualize(Token::Star),
            _ => self.error(),
        }
    }

    // This might be a dumb name for this method, but the idea is to wrap a
    // Token in the source code context that it was found in. For instance, the
    // line number, start position, and original lexeme
    fn contextualize(&self, token: Token) -> Result<TokenContext, FellowError> {
        Ok(TokenContext::new(
            token,
            self.source[self.lexeme_start..self.current_grapheme].concat(),
            self.current_line,
            self.lexeme_start,
            self.current_grapheme,
        ))
    }

    fn error(&self) -> Result<TokenContext, FellowError> {
        Err(FellowError::ScanError(ScanError {
            _message: format!("Failed to scan at {}", self.current_grapheme),
            _line: self.current_line,
            // TODO: This should be an offset from the start of the line. I'd also like to
            // calcualte the length of the error lexeme, but that might be impossible we haven't
            // finished lexing yet.
            _position: self.current_grapheme,
        }))
    }
}

// Take ownership of the source code and turn it into tokens
pub fn scan(source_code: &str) -> Result<Vec<TokenContext>, FellowError> {
    let mut state = ScanState::new(source_code);
    let mut tokens = Vec::new();

    while !state.is_at_end() {
        state.mark_lexeme_start();
        let token = state.scan_token()?;
        tokens.push(token)
    }

    tokens.push(state.contextualize(Token::EndOfFile)?);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_all_single_character_tokens() {
        let source = "(){},.-+;*";
        let tokens: Vec<Token> = scan(source)
            .unwrap()
            .into_iter()
            .map(|tc| tc.token)
            .collect();
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBrace,
                Token::RightBrace,
                Token::Comma,
                Token::Dot,
                Token::Minus,
                Token::Plus,
                Token::Semicolon,
                Token::Star,
                Token::EndOfFile
            ]
        )
    }
}
