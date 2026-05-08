use crate::parser::Expr;
use crate::lexer;

pub struct Interpreter {
}

enum Value<'a> {
    Literal(&'a lexer::Literal)
}

impl Interpreter {
    fn evaluate(expr: Expr) -> Value {
        let eval: Value = match expr {
            Expr::Literal { value } => {
                Value::Literal(value)
            },
            Expr::Grouping { expression } => {
                return Self::evaluate(*expression);
            }
            Expr::Unary { operator, right } => {
                todo!()
            }
            Expr::Binary { left, operator, right } => {
                todo!()
            }
            Expr::Ternary { condition, expression_true, expression_false } => {
                todo!()
            }
        };
        return eval;
    }
}
