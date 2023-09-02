use std::{env, fs, io, process};
use std::io::Write;

fn main() {
    match env::args().len() {
        1 => run_prompt(),
        2 => run_file(env::args().nth(1).unwrap()),
        _ => {
            eprintln!("Usage: {} [script]", env::args().nth(0).unwrap());
            process::exit(64);
        },
    };
}

fn run_prompt() {
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
            Ok(_) => run(command),
            Err(e) => println!("{}", e),
        };
    }
}

fn run_file(path: String) {
    match fs::read_to_string(path) {
        Ok(program) => {
            run(program);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}

fn run(source: String) {
    let scanner = Scanner { source };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

struct Scanner {
    source: String,
}

impl Scanner {
    fn scan_tokens(&self) -> Vec<Token> {
        let _ = &self.source;
        Vec::new()
    }
}

#[derive(Debug)]
struct Token;
