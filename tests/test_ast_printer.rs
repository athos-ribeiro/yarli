use yarli::lexer::{Literal, Token, TokenType};
use yarli::parser::{Expr, AstPrinter};

#[test]
fn ast_printer_test() {
    let expected = "(* (- 123) (group 45.67))";
    let star_token = Token {token_type: TokenType::STAR, lexeme: String::from("*"), literal: None, line: 1 };
    let minus_token = Token {token_type: TokenType::MINUS, lexeme: String::from("-"), literal: None, line: 1 };
    let value_1: Literal = Some(Box::new(123));
    let value_2: Literal = Some(Box::new(45.67));
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: &minus_token,
            right: Box::new(Expr::Literal { value: &value_1 })
        }),
        operator: &star_token,
        right: Box::new(Expr::Grouping { expression: Box::new(Expr::Literal {value: &value_2 })})
    };

    let result = AstPrinter.print(&expression);
    assert_eq!(&result, expected);
}
