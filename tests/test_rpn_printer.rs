use yarli::lexer::{Token, TokenType};
use yarli::parser::{Expr, RpnPrinter};

#[test]
fn rpn_printer_test() {
    let expected = "1 2 + 4 3 - *";
    let expression = Expr::Binary {
        left: &Expr::Grouping {
            expression: &Expr::Binary {
                left: &Expr::Literal {value: Some(Box::new(1))},
                operator: Token {token_type: TokenType::PLUS, lexeme: String::from("+"), literal: None, line: 1 },
                right: &Expr::Literal {value: Some(Box::new(2))}
            }
        },
        operator: Token {token_type: TokenType::STAR, lexeme: String::from("*"), literal: None, line: 1 },
        right: &Expr::Grouping {
            expression: &Expr::Binary {
                left: &Expr::Literal {value: Some(Box::new(4))},
                operator: Token {token_type: TokenType::MINUS, lexeme: String::from("-"), literal: None, line: 1 },
                right: &Expr::Literal {value: Some(Box::new(3))}
            }
        }
    };

    let result = RpnPrinter.print(&expression);
    assert_eq!(&result, expected);
}
