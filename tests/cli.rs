use std::fs;

use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(input_file: &str, expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("wyw")?
        .arg(input_file)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn test_basic_type() -> TestResult {
    run(
        "tests/inputs/basic-type.wy",
        "tests/expected/basic-type.out.txt",
    )
}

#[test]
fn test_binary_statement() -> TestResult {
    run(
        "tests/inputs/binary-statement.wy",
        "tests/expected/binary-statement.out.txt",
    )
}

#[test]
fn test_unary_statement() -> TestResult {
    run(
        "tests/inputs/unary-statement.wy",
        "tests/expected/unary-statement.out.txt",
    )
}

#[test]
fn test_binary_if_statement() -> TestResult {
    run(
        "tests/inputs/binary-if-statement.wy",
        "tests/expected/binary-if-statement.out.txt",
    )
}

#[test]
fn test_declare_statement() -> TestResult {
    run(
        "tests/inputs/declare-statement.wy",
        "tests/expected/declare-statement.out.txt",
    )
}

#[test]
fn test_define_statement() -> TestResult {
    run(
        "tests/inputs/define-statement.wy",
        "tests/expected/define-statement.out.txt",
    )
}

#[test]
fn test_assign_statement() -> TestResult {
    run(
        "tests/inputs/assign-statement.wy",
        "tests/expected/assign-statement.out.txt",
    )
}

#[test]
fn test_if_statement() -> TestResult {
    run(
        "tests/inputs/if-statement.wy",
        "tests/expected/if-statement.out.txt",
    )
}

#[test]
fn test_bool_algebra_statement() -> TestResult {
    run(
        "tests/inputs/bool-algebra-statement.wy",
        "tests/expected/bool-algebra-statement.out.txt",
    )
}

#[test]
fn test_for_statement() -> TestResult {
    run(
        "tests/inputs/for-statement.wy",
        "tests/expected/for-statement.out.txt",
    )
}

#[test]
fn test_for_enum_statement() -> TestResult {
    run(
        "tests/inputs/for-enum-statement.wy",
        "tests/expected/for-enum-statement.out.txt",
    )
}

#[test]
fn test_fun_statement() -> TestResult {
    run(
        "tests/inputs/fun-statement.wy",
        "tests/expected/fun-statement.out.txt",
    )
}

#[test]
fn test_factorial() -> TestResult {
    run(
        "tests/inputs/factorial.wy",
        "tests/expected/factorial.out.txt",
    )
}

#[test]
fn test_multiplication_table() -> TestResult {
    run(
        "tests/inputs/multiplication-table.wy",
        "tests/expected/multiplication-table.out.txt",
    )
}
