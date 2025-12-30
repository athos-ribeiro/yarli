use crate::lexer::{Literal, Token};

pub enum Expr<'a> {
    Binary { left: Box<Expr<'a>>, operator: &'a Token, right: Box<Expr<'a>> },
    Grouping { expression: Box<Expr<'a>> },
    Literal { value: &'a Literal },
    Unary { operator: &'a Token, right: Box<Expr<'a>> }
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
