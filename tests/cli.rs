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

#[test]
pub fn test_binary_if_statement() -> TestResult {
    run(
        "tests/inputs/binary-if-statement.wy",
        "tests/expected/binary-if-statement.out.wy",
    )
}

#[test]
pub fn test_declare_statement() -> TestResult {
    run(
        "tests/inputs/declare-statement.wy",
        "tests/expected/declare-statement.out.wy",
    )
}

#[test]
pub fn test_define_statement() -> TestResult {
    run(
        "tests/inputs/define-statement.wy",
        "tests/expected/define-statement.out.wy",
    )
}

#[test]
pub fn test_assign_statement() -> TestResult {
    run(
        "tests/inputs/assign-statement.wy",
        "tests/expected/assign-statement.out.wy",
    )
}

#[test]
pub fn test_local_define_statement() -> TestResult {
    run(
        "tests/inputs/local-define-statement.wy",
        "tests/expected/local-define-statement.out.wy",
    )
}

#[test]
pub fn test_if_statement() -> TestResult {
    run(
        "tests/inputs/if-statement.wy",
        "tests/expected/if-statement.out.wy",
    )
}
