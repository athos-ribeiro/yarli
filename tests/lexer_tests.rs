use std::path::Path;
use std::fs;
use std::process;
use yarli::{Lox, HAD_ERROR};
use yarli::lexer::Scanner;

#[test]
fn lexer_smoke_test() {
    let testdata_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata");
    let test_path = testdata_dir.join("lexer_input_1.data");
    run_file(test_path.into_os_string().into_string().unwrap());
    let test2_path = testdata_dir.join("lexer_input_2.data");
    run_file(test2_path.into_os_string().into_string().unwrap());
}

// We duplicate the old implementation of Lox::run here to allow testing tokens without considering
// any semantics (i.e., ignore parser).
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
    let mut lox = Lox{};
    let mut scanner = Scanner::new(source, &mut lox);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}
