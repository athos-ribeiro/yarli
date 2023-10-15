use std::fmt;
use crate::lexer::Token;

pub trait Expr {}

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
