use std::fmt;
use crate::lexer::Token;

pub enum Expr<'a> {
    Binary { left: &'a Expr<'a>, operator: Token, right: &'a Expr<'a> },
    Grouping { expression: &'a Expr<'a> },
    Literal { value: Option<Box<dyn fmt::Display>> },
    Unary { operator: Token, right: &'a Expr<'a> }
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
                self.parenthesize(&operator.lexeme, vec![left, right]),
            Expr::Grouping { expression } =>
                self.parenthesize(&String::from("group"), vec![expression]),
            Expr::Literal { value } => {
                if value.is_none() {
                    return String::from("nil")
                }
                value.as_ref().unwrap().to_string()
            }
            Expr::Unary { operator, right } =>
                self.parenthesize(&operator.lexeme, vec![right]),
        }
    }
}
