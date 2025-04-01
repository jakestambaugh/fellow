#[derive(Debug, PartialEq)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEquAL,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(String),
    Boolean(String),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EndOfFile,
}

// The value and position of the token from the source code
pub struct TokenContext {
    pub token: Token,
    lexeme: String,
    // The line the token was found on
    line: usize,
    // The start and end grapheme in the line that the token was at
    start: usize,
    end: usize,
}

impl TokenContext {
    pub fn new(token: Token, lexeme: String, line: usize, start: usize, end: usize) -> Self {
        Self {
            token,
            lexeme,
            line,
            start,
            end,
        }
    }
}
