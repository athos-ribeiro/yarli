use std::{fmt, str::FromStr};
use crate::Lox;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<dyn fmt::Display>>,
    pub line: usize,
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

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    runner: &'a mut Lox,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, runner: &'a mut Lox) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            runner,
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
            Some('!') => {
                if self.match_next('=') {
                    self.add_token(TokenType::BANG_EQUAL, None);
                } else {
                    self.add_token(TokenType::BANG, None);
                }
            }
            Some('=') => {
                if self.match_next('=') {
                    self.add_token(TokenType::EQUAL_EQUAL, None);
                } else {
                    self.add_token(TokenType::EQUAL, None);
                }
            }
            Some('<') => {
                if self.match_next('=') {
                    self.add_token(TokenType::LESS_EQUAL, None);
                } else {
                    self.add_token(TokenType::LESS, None);
                }
            }
            Some('>') => {
                if self.match_next('=') {
                    self.add_token(TokenType::GREATER_EQUAL, None);
                } else {
                    self.add_token(TokenType::GREATER, None);
                }
            }
            Some('/') => {
                if self.match_next('/') {
                    // ignore the whole line
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_next('*') {
                    self.block_comment();
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }
            Some(' ') | Some ('\r') | Some('\t') => (),
            Some('\n') => self.line += 1,
            Some('"') => self.string(),
            Some('0'..='9') => self.number(),
            Some('A'..='Z') | Some('a'..='z') | Some('_') => self.identifier(),
            Some(entry) => {
                self.runner.error(self.line, String::from(format!("Unexpected character '{entry}'")));
            }
            None => (),
        };
    }

    fn block_comment(&mut self) {
        loop {
            if self.is_at_end() {
                self.runner.error(self.line, String::from(format!("Unfinished block comment")));
                break;
            }
            if self.peek() == Some('*') && self.peek_next() == Some('/') {
                self.current += 2;
                break;
            }
            if self.peek() == Some ('\n') {
                self.line += 1;
            }
            self.advance();
        }
    }

    fn identifier(&mut self) {
        while let Some('0'..='9') | Some('A'..='Z') | Some('a'..='z') | Some('_') = self.peek() {
            self.advance();
        }
        // TODO: we need to account for utf8 data here. the slice below is quite error prone
        let text = &self.source[self.start..self.current];
        let identifier_type = match text {
            "and" => TokenType::AND,
            "class" => TokenType::CLASS,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "for" => TokenType::FOR,
            "fun" => TokenType::FUN,
            "if" => TokenType::IF,
            "nil" => TokenType::NIL,
            "or" => TokenType::OR,
            "print" => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "super" => TokenType::SUPER,
            "this" => TokenType::THIS,
            "true" => TokenType::TRUE,
            "var" => TokenType::VAR,
            "while" => TokenType::WHILE,
            _ => TokenType::IDENTIFIER,
        };
        self.add_token(identifier_type, None);
    }

    fn number(&mut self) {
        while let Some('0'..='9') = self.peek() {
            self.advance();
        }
        if self.peek() == Some('.') {
            if let Some('0'..='9') = self.peek_next() {
                // Condume the first '.'
                self.advance();
                // and keep parsing the digits after it.
                while let Some('0'..='9') = self.peek() {
                    self.advance();
                }
            }
        }
        // TODO: we need to account for utf8 data here. the slice below is quite error prone
        let value: f64 = f64::from_str(&self.source[self.start..self.current]).unwrap();
        self.add_token(TokenType::NUMBER, Some(Box::new(value)));
    }

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.runner.error(self.line, String::from(format!("Unterminated string.")));
            return ();
        }

        // now get the closing '"'
        self.advance();
        // trim the surrounding quotes
        // TODO: we need to account for utf8 data here. the slice below is quite error prone
        let value = String::from(&self.source[(self.start + 1)..(self.current - 1)]);
        self.add_token(TokenType::STRING, Some(Box::new(value)));
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return Some('\0');
        }
        self.source.chars().nth(self.current + 1)
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(self.current) == Some(expected) {
            self.current += 1;
            return true;
        }
        false
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.source.chars().nth(self.current);
        self.current += 1;
        next
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
pub enum TokenType {
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

