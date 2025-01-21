use assert_cmd::Command;
use predicates::prelude::*;

const PROGRAM: &str = "dataengineering-deduplication";
const DUPLICATE1: &str = "tests/inputs/sample-one.txt";
const DUPLICATE2: &str = "tests/inputs/sample-two.txt";
const DUPLICATE3: &str = "tests/inputs/sample-three.txt";

#[test]
fn test_cli() {
    let mut cmd = Command::cargo_bin(PROGRAM).unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Finds duplicate files"));
}

#[test]
fn search() {
    let mut cmd = Command::cargo_bin(PROGRAM).unwrap();
    cmd.arg("search")
        .arg("--path")
        .arg("tests/inputs")
        .arg("--pattern")
        .arg(".txt")
        .assert()
        .success()
        .stdout(predicate::str::contains(DUPLICATE1))
        .stdout(predicate::str::contains(DUPLICATE2))
        .stdout(predicate::str::contains(DUPLICATE3));
}
