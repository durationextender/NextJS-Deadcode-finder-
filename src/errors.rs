use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    UnsupportedToken(char, usize),
    UnexpectedEOF,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnsupportedToken(c, line) => {
                write!(
                    f,
                    "Lexer Error: Token '{}' is not supported at line {}",
                    c, line
                )
            }
            LexerError::UnexpectedEOF => write!(f, "Lexer Error: Unexpected end of file"),
        }
    }
}

impl Error for LexerError {}
