use std::{fs, io, process};
use std::io::Write;
use crate::lexer::{Token, TokenType, Scanner};
use crate::parser::{Parser, AstPrinter};

pub mod lexer;
pub mod parser;
pub mod interpreter;

// Set this to pub to allow integration testing
pub static mut HAD_ERROR: bool = false;

pub struct Lox {}

impl Lox {
    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut command = String::new();
            match io::stdin().read_line(&mut command) {
                Ok(0) => {
                    // erase "> " with backspaces
                    print!("\u{8}\u{8}");
                    io::stdout().flush().unwrap();
                    println!("(CTRL+D) QUIT");
                    break;
                }
                Ok(_) => {
                    self.run(command);
                    unsafe {
                        HAD_ERROR = false;
                    }
                }
                Err(e) => eprintln!("{}", e),
            };
        }
    }

    pub fn run_file(&mut self, path: String) {
        match fs::read_to_string(path) {
            Ok(program) => {
                self.run(program);
                unsafe {
                    if HAD_ERROR {
                        process::exit(65);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source, self);
        let tokens = scanner.scan_tokens();

        let parser = Parser::new(tokens);
        let expression = parser.parse();
        // Stop if there was a syntax error
        unsafe {
            if HAD_ERROR {
                return
            }
        }
        match expression {
            Some(expr) => println!("{}", AstPrinter{}.print(&expr)),
            // HAD_ERROR should be set and we should not get here
            None => (),
        }
    }

    fn error(line: usize, message: String) {
        Self::report(line, String::from(""), message);
    }

    fn report(line: usize, location: String, message: String) {
        eprintln!("[line {line}] Error{location}: {message}");
        unsafe {
            HAD_ERROR = true;
        }
    }

    fn parsing_error(token: &Token, message: String) {
        if token.token_type == TokenType::EOF {
            Self::report(token.line, String::from(" at end"), message);
        } else {
            Self::report(token.line, format!("at '{}'", token.lexeme), message);
        }
    }
}
