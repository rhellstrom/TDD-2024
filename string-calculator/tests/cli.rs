use assert_cmd::Command;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

fn run_stdin(input_file: &str, expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin("string-calculator")?
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn welcome_instructions() -> TestResult {
    let welcome = "tests/inputs/empty.txt";
    let output = "tests/expected/welcome.txt";
    run_stdin(welcome, output)
}