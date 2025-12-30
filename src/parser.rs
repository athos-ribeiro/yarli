use std::cell::Cell;
use crate::lexer::{Literal, Token, TokenType};

pub enum Expr<'a> {
    Binary { left: Box<Expr<'a>>, operator: &'a Token, right: Box<Expr<'a>> },
    Grouping { expression: Box<Expr<'a>> },
    Literal { value: &'a Literal },
    Unary { operator: &'a Token, right: Box<Expr<'a>> }
}

pub struct Parser {
    // We use a Cell here for interior mutability. Since the parsing method calls are recursive and
    // calling each other, all of them would need to borrow self butably just so one method could
    // increment the current Token pointer. Let's do it with a Cell here.
    current: Cell<usize>,
    tokens: Vec<Token>
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            current: Cell::new(0),
            tokens
        }
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.match_token(vec!(TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL)) {
            let operator: &Token = self.previous();
            let right: Box<Expr> = Box::new(self.comparison());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary { left, operator, right };
        }

        return expr
    }

    fn comparison(&self) -> Expr {
        let mut expr: Expr = self.term();

        while self.match_token(vec!(TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL)) {
            let operator: &Token = self.previous();
            let right: Box<Expr> = Box::new(self.term());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary { left, operator, right };
        }

        return expr
    }

    fn term(&self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.match_token(vec!(TokenType::MINUS, TokenType::PLUS)) {
            let operator: &Token = self.previous();
            let right: Box<Expr> = Box::new(self.factor());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary { left, operator, right };
        }

        return expr
    }

    fn factor(&self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.match_token(vec!(TokenType::SLASH, TokenType::STAR)) {
            let operator: &Token = self.previous();
            let right: Box<Expr> = Box::new(self.unary());
            let left: Box<Expr> = Box::new(expr);
            expr = Expr::Binary { left, operator, right };
        }

        return expr
    }

    fn unary(&self) -> Expr {
        if self.match_token(vec!(TokenType::BANG, TokenType::MINUS)) {
            let operator: &Token = self.previous();
            let right: Box<Expr> = Box::new(self.unary());
            return Expr::Unary { operator, right };
        }
        self.primary()
    }

    fn primary(&self) -> Expr {
        // Instead of returning booleans for true and false, and None for nil, as suggested by the
        // book, we changed the lexer code to include the literal values in the boolean tokens TRUE
        // and FALSE so we can just pass a reference to those here, avoiding issues with the borrow
        // checker when passing the values for STRING and NUMBER.
        // As an consequence, we do not need to have several if clauses here as we have in the
        // book.
        if self.match_token(vec!(TokenType::FALSE, TokenType::TRUE, TokenType::NIL, TokenType::NUMBER, TokenType::STRING)) {
            return Expr::Literal { value: &self.previous().literal };
        }
        if self.match_token(vec!(TokenType::LEFT_PAREN)) {
            let expr: Expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Expr::Grouping { expression: Box::new(expr) };
        }
        // TODO: improve error handling. We should never get here, if we do, should we really
        // panic?
        panic!()
    }

    fn consume(&self, token_type: TokenType, err_msg: &str) {
        panic!()
    }

    // match is a reserved keyword. Hence, let's call this function match_token
    fn match_token(&self, token_types: Vec<TokenType>) -> bool {
        let matches: bool = token_types.into_iter().any(|t| self.check(t));
        if matches {
            self.advance();
        }
        matches
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }


    fn advance(&self) -> &Token {
        if !self.is_at_end() {
            let mut current = self.current.get();
            current += 1;
            self.current.set(current);
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current.get()]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.get() - 1]
    }
}

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut my_str = format!("({name}");
        for expr in exprs {
            my_str.push_str(" ");
            my_str.push_str(&self.print(&expr));
        }
        my_str.push_str(")");
        my_str
    }

    pub fn print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } =>
                self.parenthesize(&operator.lexeme, vec![&left, &right]),
            Expr::Grouping { expression } =>
                self.parenthesize(&String::from("group"), vec![&expression]),
            Expr::Literal { value } => {
                if value.is_none() {
                    return String::from("nil")
                }
                value.as_ref().unwrap().to_string()
            }
            Expr::Unary { operator, right } =>
                self.parenthesize(&operator.lexeme, vec![&right]),
        }
    }
}

pub struct RpnPrinter;

impl RpnPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => format!("{} {} {}", self.print(&left), self.print(&right), &operator.lexeme),
            Expr::Grouping { expression } => {
                format!("{}", self.print(&expression))
            }
            Expr::Literal { value } => {
                if value.is_none() {
                    return String::from("nil")
                }
                value.as_ref().unwrap().to_string()
            }
            Expr::Unary { operator, right } => format!("{} {}", self.print(&right), &operator.lexeme)
        }
    }
}
