use std::{fs, io, process};
use std::io::Write;

mod lexer;

pub struct Lox {
    pub had_error: bool,
}

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
                    self.had_error = false;
                }
                Err(e) => eprintln!("{}", e),
            };
        }
    }

    pub fn run_file(&self, path: String) {
        match fs::read_to_string(path) {
            Ok(program) => {
                self.run(program);
                if self.had_error {
                    process::exit(65);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };
    }

    fn run(&self, source: String) {
        let mut scanner = lexer::Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token);
        }
    }

    fn error(&mut self, line: usize, message: String) {
        self.report(line, String::from(""), message);
    }

    fn report(&mut self, line: usize, location: String, message: String) {
        eprintln!("[line {line}] Error{location}: {message}");
        self.had_error = true;
    }
}
