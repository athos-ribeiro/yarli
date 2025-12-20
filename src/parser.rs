use std::fmt;
use crate::lexer::Token;

pub trait Expr {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T;
}

pub struct Binary {
    left: Box<dyn Expr>,
    operator: Token,
    right: Box<dyn Expr>,
}

impl Expr for Binary {}

impl Binary {
    fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }

    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

pub struct Grouping {
    expression: Box<dyn Expr>,
}

impl Expr for Grouping {}

impl Grouping {
    fn new(expression: Box<dyn Expr>) -> Self {
        Grouping {
            expression,
        }
    }

    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}

pub struct Literal {
    value: Option<Box<dyn fmt::Display>>,
}

impl Expr for Literal {}

impl Literal {
    fn new(value: Option<Box<dyn fmt::Display>>) -> Self {
        Literal {
            value,
        }
    }
}

pub struct Unary {
    operator: Token,
    right: Box<dyn Expr>,
}

impl Expr for Unary {}

impl Unary {
    fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Unary {
            operator,
            right,
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &Binary) -> T;
    fn visit_grouping_expr(&self, expr: &Grouping) -> T;
    //fn visit_literal_expr(&self, expr: &Literal) -> T;
    //fn visit_unary_expr(&self, expr: &Unary) -> T;
}

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: Vec<Box<dyn Expr>>) -> String {
        panic!()
    }

    fn print(&self, expr: &impl Expr) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        panic!()
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        panic!()
    }
}
