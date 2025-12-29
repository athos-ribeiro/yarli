use yarli::lexer::{Token, TokenType};
use yarli::parser::{Expr, RpnPrinter};

#[test]
fn rpn_printer_test() {
    let expected = "1 2 + 4 3 - *";
    let plus_token = Token {token_type: TokenType::PLUS, lexeme: String::from("+"), literal: None, line: 1 };
    let star_token = Token {token_type: TokenType::STAR, lexeme: String::from("*"), literal: None, line: 1 };
    let minus_token = Token {token_type: TokenType::MINUS, lexeme: String::from("-"), literal: None, line: 1 };
    let expression = Expr::Binary {
        left: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {value: Some(Box::new(1))}),
                operator: &plus_token,
                right: Box::new(Expr::Literal {value: Some(Box::new(2))})
            })
        }),
        operator: &star_token,
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {value: Some(Box::new(4))}),
                operator: &minus_token,
                right: Box::new(Expr::Literal {value: Some(Box::new(3))})
            })
        })
    };

    let result = RpnPrinter.print(&expression);
    assert_eq!(&result, expected);
}
