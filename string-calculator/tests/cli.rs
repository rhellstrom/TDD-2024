use assert_cmd::Command;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const NAME: &str = "string-calculator";

fn run_stdin(input_file: &str, expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin(NAME)?
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::str::ends_with(expected));
    Ok(())
}

#[test]
fn welcome_instructions() -> TestResult {
    let input = "tests/inputs/empty.txt";
    let expected = "tests/expected/welcome.txt";
    run_stdin(input, expected)
}

#[test]
fn one_line1() -> TestResult {
    let welcome = "tests/inputs/one_line.txt";
    let expected = "tests/expected/one_line.txt";
    run_stdin(welcome, expected)
}

#[test]
fn one_line_complex() -> TestResult {
    let input = "tests/inputs/one_line_complex.txt";
    let expected = "tests/expected/one_line_complex.txt";
    run_stdin(input, expected)
}

#[test]
fn multiple_newline_separated() -> TestResult {
    let input = "tests/inputs/multiple_lines.txt";
    let expected = "tests/expected/multiple_lines.txt";
    run_stdin(input, expected)
}