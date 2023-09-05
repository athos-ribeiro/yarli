use std::{env, fs, io, process};
use std::io::Write;

// TODO: should this go in a new Lox struct? This would imply moving some functions into Lox scope
// and converting them to methods.
static mut HAD_ERROR: bool = false;

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
            Ok(_) => {
                run(command);
                unsafe {
                    HAD_ERROR = false;
                }
            }
            Err(e) => eprintln!("{}", e),
        };
    }
}

fn run_file(path: String) {
    match fs::read_to_string(path) {
        Ok(program) => {
            run(program);
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

fn run(source: String) {
    let scanner = Scanner { source };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(line: usize, message: String) {
    report(line, String::from(""), message);
}

fn report(line: usize, location: String, message: String) {
    eprintln!("[line {line}] Error{location}: {message}");
    unsafe {
        HAD_ERROR = true;
    }
}
