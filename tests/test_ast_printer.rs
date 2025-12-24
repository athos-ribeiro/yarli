use yarli::lexer::{Token, TokenType};
use yarli::parser::{Expr, AstPrinter};

#[test]
fn ast_printer_test() {
    let expected = "(* (- 123) (group 45.67))";
    let expression = Expr::Binary {
        left: &Expr::Unary {
            operator: Token {token_type: TokenType::MINUS, lexeme: String::from("-"), literal: None, line: 1 },
            right: &Expr::Literal { value: Some(Box::new(123)) }
        },
        operator: Token {token_type: TokenType::STAR, lexeme: String::from("*"), literal: None, line: 1 },
        right: &Expr::Grouping { expression: &Expr::Literal {value: Some(Box::new(45.67))} }
    };

    let result = AstPrinter.print(&expression);
    assert_eq!(&result, expected);
}
