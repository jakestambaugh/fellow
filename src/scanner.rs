use crate::{FellowError, ScanError, Token, TokenContext};

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

    fn lexeme(&self) -> String {
        self.source[self.lexeme_start..self.current_grapheme].concat()
    }

    fn is_at_end(&self) -> bool {
        self.current_grapheme >= self.source.len()
    }

    fn next(&mut self) -> &str {
        let c = self.source[self.current_grapheme];
        self.current_grapheme += 1;
        c
    }

    fn next_matches(&mut self, expected: &str) -> bool {
        if self.is_at_end() || self.source[self.current_grapheme] != expected {
            false
        } else {
            self.current_grapheme += 1;
            true
        }
    }

    fn peek(&self) -> &str {
        if self.is_at_end() {
            "\0"
        } else {
            self.source[self.current_grapheme]
        }
    }

    // Advances the scanner and emits the next token
    //
    // I originally tried to follow the pattern in Crafting Interpreters where the lexer skips
    // certain characters like newlines and comments without tokenizing them. However, this meant
    // that the scan_token function was becoming very "mulit-modal" in its return structure.
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
            "(" => Ok(self.contextualize(Token::LeftParen)),
            ")" => Ok(self.contextualize(Token::RightParen)),
            "{" => Ok(self.contextualize(Token::LeftBrace)),
            "}" => Ok(self.contextualize(Token::RightBrace)),
            "," => Ok(self.contextualize(Token::Comma)),
            "." => Ok(self.contextualize(Token::Dot)),
            "-" => Ok(self.contextualize(Token::Minus)),
            "+" => Ok(self.contextualize(Token::Plus)),
            ";" => Ok(self.contextualize(Token::Semicolon)),
            "*" => Ok(self.contextualize(Token::Star)),
            "\\" => Ok(self.contextualize(Token::ForwardSlash)),
            "!" => {
                if self.next_matches("=") {
                    Ok(self.contextualize(Token::BangEqual))
                } else {
                    Ok(self.contextualize(Token::Bang))
                }
            }
            "=" => {
                if self.next_matches("=") {
                    Ok(self.contextualize(Token::EqualEqual))
                } else {
                    Ok(self.contextualize(Token::Equal))
                }
            }
            "<" => {
                if self.next_matches("=") {
                    Ok(self.contextualize(Token::LessEqual))
                } else {
                    Ok(self.contextualize(Token::Less))
                }
            }
            ">" => {
                if self.next_matches("=") {
                    Ok(self.contextualize(Token::GreaterEqual))
                } else {
                    Ok(self.contextualize(Token::Greater))
                }
            }
            "/" => {
                if self.next_matches("/") {
                    self.comment()
                } else {
                    Ok(self.contextualize(Token::Slash))
                }
            }
            ":" => {
                if self.next_matches(":") {
                    Ok(self.contextualize(Token::ColonColon))
                } else {
                    Ok(self.contextualize(Token::Colon))
                }
            }
            " " => Ok(self.contextualize(Token::Space)),
            "\r" => Ok(self.contextualize(Token::CarriageReturn)),
            "\t" => Ok(self.contextualize(Token::Tab)),
            "\n" => {
                self.current_line += 1;
                Ok(self.contextualize(Token::NewLine))
            }
            "\"" => self.string(),
            _ => Err(self.error()),
        }
    }

    // This might be a dumb name for this method, but the idea is to wrap a
    // Token in the source code context that it was found in. For instance, the
    // line number, start position, and original lexeme
    fn contextualize(&self, token: Token) -> TokenContext {
        TokenContext::new(
            token,
            self.lexeme(),
            self.current_line,
            self.lexeme_start,
            self.current_grapheme,
        )
    }

    // Longer lexemes

    fn string(&mut self) -> Result<TokenContext, FellowError> {
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                self.current_line += 1;
            }
            self.next();
        }
        if self.is_at_end() {
            Err(FellowError::ScanError(ScanError {
                message: format!("Unterminated string {}", self.lexeme()),
                line: self.current_line,
                position: self.current_grapheme,
            }))
        } else {
            // Consume the final "
            self.next();
            // The +1 and -1 trim the actual "" characters since we only want the contents of the
            // string.
            Ok(self.contextualize(Token::String(
                self.source[self.lexeme_start + 1..self.current_grapheme - 1].concat(),
            )))
        }
    }

    fn comment(&mut self) -> Result<TokenContext, FellowError> {
        while self.peek() != "\n" && !self.is_at_end() {
            let a = self.peek();
            if a == "\n" {
                eprintln!("something has gone wrong, there is a newline in the comment");
            }
            self.next();
        }
        if self.peek() == "\n" {
            self.current_line += 1;
            self.next();
        }
        // The +2 is to skip the // characters since we only want the text of the comment.
        // The -1 is to cut off the newline that follows the comment
        Ok(self.contextualize(Token::Comment(
            self.source[self.lexeme_start + 2..self.current_grapheme - 1].concat(),
        )))
    }

    fn error(&self) -> FellowError {
        FellowError::ScanError(ScanError {
            message: format!("Failed to scan at {}", self.current_grapheme),
            line: self.current_line,
            // TODO: This should be an offset from the start of the line. I'd also like to
            // calcualte the length of the error lexeme, but that might be impossible we haven't
            // finished lexing yet.
            position: self.current_grapheme,
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

    tokens.push(state.contextualize(Token::EndOfFile));
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Runs the scan function and unpacks the Tokens from the TokenContext.
    // This helper calls unwrap() so it should only be used when the test
    // is expected to scan properlyThis helper calls unwrap() so it should only be used when the
    // test is expected to scan properly.
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
        // I threw some spaces in here because my font makes ligatures that can make the tokens a
        // bit confusing when stacked together.
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
}
