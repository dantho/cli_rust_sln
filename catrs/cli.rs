use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echors")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    // Try again with flag, but still no args
    let mut cmd = Command::cargo_bin("echors")?;
    cmd
        .arg("-n")
        .assert()
            .failure()
            .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn dies_too_many_flags() -> TestResult {
    let mut cmd = Command::cargo_bin("echors")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    // Try again with flag, but still no args
    let mut cmd = Command::cargo_bin("echors")?;
    cmd
        .arg("-n")
        .arg("-n")
        .assert()
            .failure()
            .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echors")?;
    cmd.arg("Hello").assert().success();
    let mut cmd = Command::cargo_bin("echors")?;
    cmd
        .arg("Hello")
        .arg("world")
        .assert()
            .stdout(predicate::str::contains("Hello"))
            .stdout(predicate::str::contains("world\r\n"));
        let mut cmd = Command::cargo_bin("echors")?;
        cmd
            .arg("-n")
            .arg("Hello")
            .arg("world")
            .assert()
                .stdout(predicate::str::contains("Hello"))
                .stdout(predicate::str::contains("world"))
                .stdout(predicate::str::contains("\n").not());
    let mut cmd = Command::cargo_bin("echors")?;
    cmd
        .arg("-n")
        .arg("Hello")
        .arg("world")
        .assert()
            .stdout(predicate::str::contains("Hello"))
            .stdout(predicate::str::contains("world"))
            .stdout(predicate::str::contains("\n").not());
    let mut cmd = Command::cargo_bin("echors")?;
    cmd
        .arg("--omit_newline")
        .arg("Hello")
        .arg("world")
        .assert()
            .stdout(predicate::str::contains("Hello"))
            .stdout(predicate::str::contains("world"))
            .stdout(predicate::str::contains("\n").not());
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&vec!["Hello there"], "tests\\expected\\hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&vec!["Hello", "there"], "tests\\expected\\hello2.txt")
}

#[test]
fn hello1_omit_newline() -> TestResult {
    run(&vec!["-n", "Hello there"], "tests\\expected\\hello1.n.txt")
}

#[test]
fn hello2_omit_newline() -> TestResult {
    run(&vec!["Hello", "-n", "there"], "tests\\expected\\hello2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echors")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}