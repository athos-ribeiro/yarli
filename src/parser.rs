use std::fmt;
use crate::lexer::Token;

pub enum Expr<'a> {
    Binary { left: &'a Expr<'a>, operator: Token, right: &'a Expr<'a> },
    Grouping { expression: &'a Expr<'a> },
    Literal { value: Option<Box<dyn fmt::Display>> },
    Unary { operator: Token, right: &'a Expr<'a> }
}

impl<'a> Expr<'a> {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Self::Binary { .. } => visitor.visit_binary_expr(self),
            Self::Grouping { .. } => visitor.visit_grouping_expr(self),
            Self::Literal { .. } => visitor.visit_literal_expr(self),
            Self::Unary { .. }=> visitor.visit_unary_expr(self)
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &Expr) -> T;
    fn visit_grouping_expr(&self, expr: &Expr) -> T;
    fn visit_literal_expr(&self, expr: &Expr) -> T;
    fn visit_unary_expr(&self, expr: &Expr) -> T;
}

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut my_str = format!("({name}");
        for expr in exprs {
            my_str.push_str(" ");
            my_str.push_str(&expr.accept(self));
        }
        my_str.push_str(")");
        my_str
    }

    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Expr) -> String {
        if let Expr::Binary { left, operator, right } = expr {
            self.parenthesize(&operator.lexeme, vec![left, right])
        } else {
            panic!()
        }
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> String {
        if let Expr::Grouping { expression } = expr {
            self.parenthesize(&String::from("group"), vec![expression])
        } else {
            panic!()
        }
    }

    fn visit_literal_expr(&self, expr: &Expr) -> String {
        if let Expr::Literal { value } = expr {
            if value.is_none() {
                return String::from("nil")
            }
            value.as_ref().unwrap().to_string()
        } else {
            panic!()
        }
    }

    fn visit_unary_expr(&self, expr: &Expr) -> String {
        if let Expr::Unary { operator, right } = expr {
            self.parenthesize(&operator.lexeme, vec![right])
        } else {
            panic!()
        }
    }
}
