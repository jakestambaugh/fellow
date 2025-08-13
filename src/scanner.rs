use std::string::FromUtf8Error;

use crate::{FellowError, ScanError, Token, TokenContext};

/// The state of the scan is held in a struct. Helper functions can operate
/// on the state to figure out which tokens to emit
pub struct ScanState<'a> {
    source: &'a [u8],
    lexeme_start: usize,
    current_byte: usize,
    current_line: usize,
}

impl<'a> ScanState<'a> {
    // I think that this project could become an experiment in Unicode source code interpretation.
    // There are many guidances from the Unicode Consortium about the proper way to do this, and I
    // think it would be fun to try to understand as much of it as possible.
    // https://www.unicode.org/reports/tr55/#Specifications
    fn new(source_code: &'a str) -> Self {
        Self {
            source: source_code.as_bytes(),
            lexeme_start: 0,
            current_byte: 0,
            current_line: 0,
        }
    }

    fn mark_lexeme_start(&mut self) {
        self.lexeme_start = self.current_byte;
    }

    fn lexeme(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.source[self.lexeme_start..self.current_byte].to_vec())
    }

    fn is_at_end(&self) -> bool {
        self.current_byte >= self.source.len()
    }

    fn next(&mut self) -> char {
        let c = self.source[self.current_byte];
        self.current_byte += 1;
        c as char
    }

    fn next_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current_byte] != expected as u8 {
            false
        } else {
            self.current_byte += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current_byte] as char
        }
    }

    // Advances the scanner and emits the next token
    //
    // I originally tried to follow the pattern in Crafting Interpreters where the lexer skips
    // certain characters like newlines and comments without tokenizing them. However, this meant
    // that the scan_token function was becoming very "multi-modal" in its return structure.
    // It could return:
    // * A successfully parsed Token
    // * An error, most likely due to malformed input
    // * A "successful" None value in cases where the next set of characters represented a
    // non-token.
    //
    // Instead of this pattern, I decided to just create more types of Tokens and tokenize every
    // character in the input, including whitespace. In the end, this will give me more flexibility
    // down the road to make whitespace-sensitive grammar.
    fn scan_token(&mut self) -> Result<TokenContext, FellowError> {
        let c = self.next();
        match c {
            '(' => self.contextualize(Token::LeftParen),
            ')' => self.contextualize(Token::RightParen),
            '{' => self.contextualize(Token::LeftBrace),
            '}' => self.contextualize(Token::RightBrace),
            ',' => self.contextualize(Token::Comma),
            '.' => self.contextualize(Token::Dot),
            '-' => self.contextualize(Token::Minus),
            '+' => self.contextualize(Token::Plus),
            ';' => self.contextualize(Token::Semicolon),
            '*' => self.contextualize(Token::Star),
            '\\' => self.contextualize(Token::ForwardSlash),
            '!' => {
                if self.next_matches('=') {
                    self.contextualize(Token::BangEqual)
                } else {
                    self.contextualize(Token::Bang)
                }
            }
            '=' => {
                if self.next_matches('=') {
                    self.contextualize(Token::EqualEqual)
                } else {
                    self.contextualize(Token::Equal)
                }
            }
            '<' => {
                if self.next_matches('=') {
                    self.contextualize(Token::LessEqual)
                } else {
                    self.contextualize(Token::Less)
                }
            }
            '>' => {
                if self.next_matches('=') {
                    self.contextualize(Token::GreaterEqual)
                } else {
                    self.contextualize(Token::Greater)
                }
            }
            '/' => {
                if self.next_matches('/') {
                    self.comment()
                } else {
                    self.contextualize(Token::Slash)
                }
            }
            ':' => {
                if self.next_matches(':') {
                    self.contextualize(Token::ColonColon)
                } else {
                    self.contextualize(Token::Colon)
                }
            }
            ' ' => self.contextualize(Token::Space),
            '\r' => self.contextualize(Token::CarriageReturn),
            '\t' => self.contextualize(Token::Tab),
            '\n' => {
                self.current_line += 1;
                self.contextualize(Token::NewLine)
            }
            '\"' => self.string(),
            '0'..='9' => self.integer_or_float(),
            // https://www.unicode.org/reports/tr31/
            // I think it would be cool to use the unicode-ident table to define what counts as an
            // identifier. The unicode-ident crate doesn't take bytes, it takes chars. Not sure
            // if I should just index into c[0] and pass it in, or try to understand which
            // bytes bytes count as non XID_start chars.
            'A'..='Z' | 'a'..='z' | '_' => self.keyword_or_identifier(),
            _ => Err(self.error()),
        }
    }

    // This might be a dumb name for this method, but the idea is to wrap a
    // Token in the source code context that it was found in. For instance, the
    // line number, start position, and original lexeme
    fn contextualize(&self, token: Token) -> Result<TokenContext, FellowError> {
        Ok(TokenContext::new(
            token,
            self.lexeme()?,
            self.current_line,
            self.lexeme_start,
            self.current_byte,
        ))
    }

    // Longer lexemes

    fn string(&mut self) -> Result<TokenContext, FellowError> {
        while self.peek() != '\"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.current_line += 1;
            }
            self.next();
        }
        if self.is_at_end() {
            Err(FellowError::ScanError(ScanError {
                message: format!("Unterminated string {}", self.lexeme().unwrap()), // TODO: deal
                // with this
                // unwrap too
                line: self.current_line,
                position: self.current_byte,
            }))
        } else {
            // Consume the final "
            self.next();
            // The +1 and -1 trim the actual "" characters since we only want the contents of the
            // string.
            let lexeme: String = String::from_utf8(
                self.source[self.lexeme_start + 1..self.current_byte - 1].to_vec(),
            )?;
            self.contextualize(Token::String(lexeme))
        }
    }

    fn integer_or_float(&mut self) -> Result<TokenContext, FellowError> {
        while self.peek().is_ascii_digit() && !self.is_at_end() {
            self.next();
        }
        // TODO: I need to check for the case where the dot ends the number. I could make that implicitly .0, but that isn't obvious.
        // Doing this properly would require an extra lookahead to see if the next character is a digit.
        // Consuming a character with next() would comsume the . without generating a PERIOD token.
        if self.peek() == '.' {
            while self.peek().is_ascii_digit() && !self.is_at_end() {
                self.next();
            }
            match self.lexeme()?.parse() {
                Ok(value) => self.contextualize(Token::Float(value)),
                Err(e) => Err(FellowError::ScanError(ScanError {
                    message: e.to_string(),
                    line: self.current_line,
                    position: self.current_byte,
                })),
            }
        } else {
            match self.lexeme()?.parse() {
                Ok(value) => self.contextualize(Token::Integer(value)),
                Err(e) => Err(FellowError::ScanError(ScanError {
                    message: e.to_string(),
                    line: self.current_line,
                    position: self.current_byte,
                })),
            }
        }
    }

    fn comment(&mut self) -> Result<TokenContext, FellowError> {
        while self.peek() != '\n' && !self.is_at_end() {
            self.next();
        }
        if self.peek() == '\n' {
            self.current_line += 1;
            self.next();
        }
        // The +2 is to skip the // characters since we only want the text of the comment.
        // The -1 is to cut off the newline that follows the comment
        self.contextualize(Token::Comment(String::from_utf8(
            self.source[self.lexeme_start + 2..self.current_byte - 1].to_vec(),
        )?))
    }

    fn keyword_or_identifier(&mut self) -> Result<TokenContext, FellowError> {
        while (self.peek().is_alphanumeric() || self.peek() == '_') && !self.is_at_end() {
            self.next();
        }
        let lexeme = self.lexeme()?;
        let token = match lexeme.as_str() {
            "and" => Token::And,
            "class" => Token::Class,
            "else" => Token::Else,
            "false" => Token::False,
            "fun" => Token::Fun,
            "for" => Token::For,
            "if" => Token::If,
            "nil" => Token::Nil,
            "or" => Token::Or,
            "print" => Token::Print,
            "return" => Token::Return,
            "super" => Token::Super,
            "this" => Token::This,
            "true" => Token::True,
            "var" => Token::Var,
            "while" => Token::While,
            _ => Token::Identifier(lexeme),
        };
        self.contextualize(token)
    }

    fn error(&self) -> FellowError {
        FellowError::ScanError(ScanError {
            message: format!("Failed to scan at {}", self.current_byte),
            line: self.current_line,
            // TODO: This should be an offset from the start of the line. I'd also like to
            // calculate the length of the error lexeme, but that might be impossible since we haven't
            // finished lexing yet.
            position: self.current_byte,
        })
    }
}

// Take ownership of the source code and turn it into tokens
pub fn scan(source_code: &str) -> Result<Vec<TokenContext>, FellowError> {
    let mut state = ScanState::new(source_code);
    let mut tokens = Vec::new();

    while !state.is_at_end() {
        state.mark_lexeme_start();
        let token = state.scan_token()?;
        tokens.push(token);
    }

    // safe to unwrap because we are at the end of the source code
    // and the EndOfFile token is always valid.
    tokens.push(state.contextualize(Token::EndOfFile).unwrap());
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Runs the scan function and unpacks the Tokens from the TokenContext.
    // This helper calls unwrap() so it should only be used when the test
    // is expected to scan properly.
    fn scan_to_tokens(source: &str) -> Vec<Token> {
        scan(source)
            .unwrap()
            .into_iter()
            .map(|tc| tc.token)
            .collect()
    }

    #[test]
    fn scans_all_single_character_tokens() {
        let source = "(){},.-+;*";
        let tokens = scan_to_tokens(source);
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

    #[test]
    fn scans_multiline_strings() {
        let source = r#" "Hello
There" "#;
        let tokens = scan_to_tokens(source);
        assert_eq!(
            tokens,
            vec![
                Token::Space,
                Token::String("Hello\nThere".to_string()),
                Token::Space,
                Token::EndOfFile
            ]
        )
    }

    #[test]
    fn scans_strings() {
        let source = r#" "Hello" "#;
        let tokens = scan_to_tokens(source);
        assert_eq!(
            tokens,
            vec![
                Token::Space,
                Token::String("Hello".to_string()),
                Token::Space,
                Token::EndOfFile
            ]
        )
    }

    #[test]
    fn scans_comments() {
        let source = r#"// This is a comment
// So is this
"Not this"
"#;
        let tokens = scan_to_tokens(source);
        assert_eq!(
            tokens,
            vec![
                Token::Comment(" This is a comment".to_string()),
                Token::Comment(" So is this".to_string()),
                Token::String("Not this".to_string()),
                Token::NewLine,
                Token::EndOfFile
            ]
        )
    }

    #[test]
    fn scans_two_char_tokens() {
        // I threw some spaces in here because my font has ligatures. It can make the tokens a
        // bit visually confusing when stacked together.
        let source = "=!<>: != <= >= == ::";
        let tokens = scan_to_tokens(source);
        assert_eq!(
            tokens,
            vec![
                Token::Equal,
                Token::Bang,
                Token::Less,
                Token::Greater,
                Token::Colon,
                Token::Space,
                Token::BangEqual,
                Token::Space,
                Token::LessEqual,
                Token::Space,
                Token::GreaterEqual,
                Token::Space,
                Token::EqualEqual,
                Token::Space,
                Token::ColonColon,
                Token::EndOfFile
            ]
        )
    }

    #[test]
    fn scans_keywords() {
        let source =
            "and class else false fun for if nil or print return super this true var while";
        let tokens: Vec<Token> = scan_to_tokens(source)
            .into_iter()
            .filter(|t| t != &Token::Space)
            .collect();
        assert_eq!(
            tokens,
            vec![
                Token::And,
                Token::Class,
                Token::Else,
                Token::False,
                Token::Fun,
                Token::For,
                Token::If,
                Token::Nil,
                Token::Or,
                Token::Print,
                Token::Return,
                Token::Super,
                Token::This,
                Token::True,
                Token::Var,
                Token::While,
                Token::EndOfFile
            ]
        )
    }

    #[test]
    fn scans_identifiers() {
        let source = "myIdentifier anotherIdentifier _underscore123";
        let tokens = scan_to_tokens(source);
        assert_eq!(
            tokens,
            vec![
                Token::Identifier("myIdentifier".to_string()),
                Token::Space,
                Token::Identifier("anotherIdentifier".to_string()),
                Token::Space,
                Token::Identifier("_underscore123".to_string()),
                Token::EndOfFile
            ]
        )
    }
}
