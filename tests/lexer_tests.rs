use std::path::Path;
use yarli::Lox;

#[test]
fn lexer_smoke_test() {
    let mut lox = Lox { had_error: false };
    let testdata_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata");
    let test_path = testdata_dir.join("lexer_input_1.data");
    lox.run_file(test_path.into_os_string().into_string().unwrap());
    let test2_path = testdata_dir.join("lexer_input_2.data");
    lox.run_file(test2_path.into_os_string().into_string().unwrap());
}
