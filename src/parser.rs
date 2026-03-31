use crate::lexers::Token;
use crate::lexers::TokenKind;

#[derive(Debug)]
pub struct ImportStatement {
    pub names: Vec<String>,
    pub source: String,
    is_default: bool,
}

#[derive(Debug)]
pub struct ExportStatement {
    pub names: Vec<String>,
    source: Option<String>,
    pub is_default: bool,
}

pub struct Parser {
    source: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(lexer_tokens: Vec<Token>) -> Self {
        Parser {
            source: lexer_tokens,
            current: 0,
        }
    }
    fn peek(&self) -> Option<&Token> {
        self.source.get(self.current)
    }
    fn advance(&mut self) -> &Token {
        self.source
            .get(self.current)
            .inspect(|t| {
                self.current += 1;
            })
            .expect("Parser Error: Attempted to advance past End of Stream. Check Lexer EOS logic.")
    }
    pub fn parse(&mut self) -> (Vec<ImportStatement>, Vec<ExportStatement>) {
        let mut imports = Vec::new();
        let mut exports = Vec::new();
        loop {
            let token = self.peek().unwrap();
            if token.kind == TokenKind::Eos {
                break;
            }
            match token.kind {
                TokenKind::Import => {
                    imports.push(self.parse_import());
                }
                TokenKind::Export => {
                    exports.push(self.parse_export());
                }
                _ => {
                    self.advance();
                }
            }
        }
        (imports, exports)
    }
    fn parse_import(&mut self) -> ImportStatement {
        let mut names: Vec<String> = vec![];
        let mut source = String::new();
        let mut is_default = false;

        self.advance();

        if let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::LeftBrace => {
                    self.advance();
                    while let Some(t) = self.peek() {
                        match &t.kind {
                            TokenKind::RightBrace => {
                                self.advance();
                                break;
                            }
                            TokenKind::Comma => {
                                self.advance();
                            }
                            TokenKind::Identifier(name) => {
                                names.push(name.clone());
                                self.advance();
                            }
                            _ => {
                                self.advance();
                            }
                        }
                    }
                }

                TokenKind::Identifier(_name) => {
                    is_default = true;
                    names.push("default".to_string());
                    self.advance();
                }

                TokenKind::Star => {
                    self.advance();

                    self.advance();
                    if let Some(TokenKind::Identifier(name)) = self.peek().map(|t| &t.kind) {
                        names.push(name.clone());
                        self.advance();
                    }
                }

                TokenKind::String(_) => {}
                _ => {}
            }
        }

        if let Some(t) = self.peek() {
            if t.kind == TokenKind::From {
                self.advance();
            }
        }

        if let Some(TokenKind::String(src)) = self.peek().map(|t| &t.kind) {
            source.push_str(src);
            self.advance();
        }

        ImportStatement {
            names,
            source,
            is_default,
        }
    }
    fn parse_export(&mut self) -> ExportStatement {
        let mut names: Vec<String> = vec![];
        let mut is_default = false;
        let mut source: Option<String> = None;

        self.advance();

        if let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Default => {
                    is_default = true;
                    self.advance();

                    names.push("default".to_string());

                    if let Some(t) = self.peek() {
                        if matches!(t.kind, TokenKind::Function | TokenKind::Class) {
                            self.advance();
                            if let Some(TokenKind::Identifier(_)) = self.peek().map(|t| &t.kind) {
                                self.advance();
                            }
                        }
                    }
                }

                TokenKind::LeftBrace => {
                    self.advance();
                    while let Some(t) = self.peek() {
                        match &t.kind {
                            TokenKind::RightBrace => {
                                self.advance();
                                break;
                            }
                            TokenKind::Comma => {
                                self.advance();
                            }
                            TokenKind::Identifier(name) => {
                                names.push(name.clone());
                                self.advance();
                            }
                            _ => {
                                self.advance();
                            }
                        }
                    }
                }

                TokenKind::Const | TokenKind::Function | TokenKind::Class | TokenKind::Let => {
                    self.advance();
                    if let Some(TokenKind::Identifier(name)) = self.peek().map(|t| &t.kind) {
                        names.push(name.clone());
                        self.advance();
                    }
                }
                _ => {}
            }
        }

        if let Some(t) = self.peek() {
            if t.kind == TokenKind::From {
                self.advance(); 
                if let Some(TokenKind::String(src)) = self.peek().map(|t| &t.kind) {
                    source = Some(src.clone());
                    self.advance();
                }
            }
        }

        ExportStatement {
            names,
            source,
            is_default,
        }
    }
}
