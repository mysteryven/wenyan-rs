use std::fs;

use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(input_file: &str, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("wyr")?
        .arg(input_file)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
pub fn test_basic_type() -> TestResult {
    run(
        "tests/inputs/basic-type.wy",
        "tests/expected/basic-type.out.wy",
    )
}

#[test]
pub fn test_binary_statement() -> TestResult {
    run(
        "tests/inputs/binary-statement.wy",
        "tests/expected/binary-statement.out.wy",
    )
}

#[test]
pub fn test_unary_statement() -> TestResult {
    run(
        "tests/inputs/unary-statement.wy",
        "tests/expected/unary-statement.out.wy",
    )
}
