use std::fmt;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn fmt::Display>>,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.as_ref()
            .or(Some(Box::new("null") as Box<dyn fmt::Display>).as_ref())
            .unwrap(),
        )
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }
        let t = Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: None,
            line: 1,
        };
        self.tokens.push(t);
        &self.tokens
    }

    fn scan_token(&mut self) {
        match self.advance() {
            Some('(') => self.add_token(TokenType::LEFT_PAREN, None),
            Some(')') => self.add_token(TokenType::RIGHT_PAREN, None),
            Some('{') => self.add_token(TokenType::LEFT_BRACE, None),
            Some('}') => self.add_token(TokenType::RIGHT_BRACE, None),
            Some(',') => self.add_token(TokenType::COMMA, None),
            Some('.') => self.add_token(TokenType::DOT, None),
            Some('-') => self.add_token(TokenType::MINUS, None),
            Some('+') => self.add_token(TokenType::PLUS, None),
            Some(';') => self.add_token(TokenType::SEMICOLON, None),
            Some('*') => self.add_token(TokenType::STAR, None),
            Some(_) => (),
            None => (),
        };
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Box<dyn fmt::Display>>) {
        // TODO: we need to account for utf8 data here. the slice below is quite error prone
        let text = String::from(&self.source[self.start..self.current]);
        let line = 0;
        self.tokens.push(Token {token_type, lexeme: text, literal, line});
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum TokenType {
    // single character tokens
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    // one or two character tokens
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,
    // literals
    IDENTIFIER, STRING, NUMBER,
    // keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR, PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

