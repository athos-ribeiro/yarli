use yarli::lexer::{Token, TokenType};
use yarli::parser::{Expr, AstPrinter};
fn main() {
    let expression = Expr::Binary {
        left: &Expr::Unary {
            operator: Token {token_type: TokenType::MINUS, lexeme: String::from("-"), literal: None, line: 1 },
            right: &Expr::Literal { value: Some(Box::new(123)) }
        },
        operator: Token {token_type: TokenType::STAR, lexeme: String::from("*"), literal: None, line: 1 },
        right: &Expr::Grouping { expression: &Expr::Literal {value: Some(Box::new(45.67))} }
    };

    let expected = AstPrinter.print(&expression);
    assert!(&expected == "(* (- 123) (group 45.67))");
    println!("{}", expected);
}
