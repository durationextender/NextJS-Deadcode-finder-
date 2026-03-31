use crate::errors::LexerError;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    line: u32,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Export,
    Import,
    From,
    Default,
    As,
    Type,
    Star,
    LeftBrace,
    RightBrace,
    Comma,
    Eos,
    String(String),
    Identifier(String),
    Function,
    Const,
    Class,
    Let,
    Require,
    LeftParen,
    RightParen,
    Equals,
    SemiColon,
}

impl Token {
    pub fn new(kind: TokenKind, line: u32) -> Self {
        Token { kind, line }
    }
}

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    line: u32,
    current: usize,
}

impl Lexer {
    pub fn new(source: Vec<char>) -> Self {
        Lexer {
            source,
            line: 1,
            current: 0,
        }
    }
    fn white_space(&mut self) {
        while self.peek().is_whitespace() {
            self.advance();
        }
    }
    fn advance(&mut self) -> char {
        if let Some(&c) = self.source.get(self.current) {
            if c == '\n' {
                self.line += 1;
            }
            self.current += 1;
            c
        } else {
            '\0'
        }
    }
    fn peek(&self) -> char {
        let current_char = self.source.get(self.current).cloned();
        current_char.unwrap_or('\0')
    }
    fn next_token(&mut self) -> Result<Token, LexerError> {
        self.white_space();
        if self.current >= self.source.len() {
            return Ok(Token::new(TokenKind::Eos, self.line));
        }
        let c = self.advance();
        match c {
            '{' => Ok(Token::new(TokenKind::LeftBrace, self.line)),
            '}' => Ok(Token::new(TokenKind::RightBrace, self.line)),
            '(' => Ok(Token::new(TokenKind::LeftParen, self.line)),
            ')' => Ok(Token::new(TokenKind::RightParen, self.line)),
            '*' => Ok(Token::new(TokenKind::Star, self.line)),
            ',' => Ok(Token::new(TokenKind::Comma, self.line)),
            ';' => Ok(Token::new(TokenKind::SemiColon, self.line)),
            '=' => Ok(Token::new(TokenKind::Equals, self.line)),
            '"' | '\'' => {
                let mut contents = String::new();
                while self.peek() != c && self.current < self.source.len() {
                    contents.push(self.advance());
                }
                if self.current >= self.source.len() {
                    return Err(LexerError::UnsupportedToken('\0', self.line as usize));
                }
                self.advance();
                Ok(Token::new(TokenKind::String(contents), self.line))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut contents = String::new();
                contents.push(c);
                while self.peek().is_alphanumeric() || self.peek() == '_' {
                    contents.push(self.advance());
                }
                let kind = match contents.as_str() {
                    "as" => TokenKind::As,
                    "import" => TokenKind::Import,
                    "export" => TokenKind::Export,
                    "from" => TokenKind::From,
                    "default" => TokenKind::Default,
                    "type" => TokenKind::Type,
                    "function" => TokenKind::Function,
                    "const" => TokenKind::Const,
                    "class" => TokenKind::Class,
                    "let" => TokenKind::Let,
                    "require" => TokenKind::Require,
                    _ => TokenKind::Identifier(contents),
                };

                Ok(Token::new(kind, self.line))
            }

            _ => self.next_token(),
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eos = token.kind == TokenKind::Eos;

            tokens.push(token);

            if is_eos {
                break;
            }
        }
        Ok(tokens)
    }
}
