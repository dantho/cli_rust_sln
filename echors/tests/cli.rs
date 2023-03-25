use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    // Try again with flag, but still no args
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd
        .arg("-n")
        .assert()
            .failure()
            .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd.arg("Hello").assert().success();
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd
        .arg("Hello")
        .arg("world")
        .assert()
            .stdout(predicate::str::contains("Hello"))
            .stdout(predicate::str::contains("world\n"));
    let mut cmd = Command::cargo_bin("echors").unwrap();
    cmd
        .arg("-n")
        .arg("Hello")
        .arg("world")
        .assert()
            .stdout(predicate::str::contains("Hello"))
            .stdout(predicate::str::contains("world"))
            .stdout(predicate::str::contains("\n").not());
}