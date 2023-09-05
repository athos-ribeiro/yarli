use std::{env, fs, io, process};
use std::io::Write;

fn main() {
    let mut lox = Lox { had_error: false };
    match env::args().len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(env::args().nth(1).unwrap()),
        _ => {
            eprintln!("Usage: {} [script]", env::args().nth(0).unwrap());
            process::exit(64);
        },
    };
}

#[derive(Debug)]
struct Token;

struct Scanner {
    source: String,
}

impl Scanner {
    fn scan_tokens(&self) -> Vec<Token> {
        let _ = &self.source;
        Vec::new()
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn run_prompt(&mut self) {
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

    fn run_file(&self, path: String) {
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
        let scanner = Scanner { source };
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
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
