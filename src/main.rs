use std::{env, process};
use yarli::Lox;

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

