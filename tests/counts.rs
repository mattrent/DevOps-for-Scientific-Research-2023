use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{error::Error, process::Command};

#[test]
fn test_fail_on_missing_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("wcrs")?;
    cmd.arg("tests/fixtures/file-doesnt-exist");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn test_word_counts() -> Result<(), Box<dyn Error>> {
    let fixtures = [
        "tests/fixtures/ex1.txt",
        "tests/fixtures/ex2.txt",
        "tests/fixtures/ex3.txt",
    ];
    let results = ["14\n", "12\n", "12\n"];

    for i in 0..fixtures.len() {
        let mut cmd = Command::cargo_bin("wcrs")?;
        cmd.arg(fixtures[i]).arg("-w");
        let out = cmd.assert().success().get_output().stdout.to_owned();
        let content = String::from_utf8(out)?;

        assert_eq!(content, String::from(results[i]));
    }
    Ok(())
}

#[test]
fn test_line_counts() -> Result<(), Box<dyn Error>> {
    let fixtures = [
        "tests/fixtures/ex1.txt",
        "tests/fixtures/ex2.txt",
        "tests/fixtures/ex3.txt",
    ];
    let results = ["0\n", "7\n", "11\n"];

    for i in 0..fixtures.len() {
        let mut cmd = Command::cargo_bin("wcrs")?;
        cmd.arg(fixtures[i]).arg("-l");
        let out = cmd.assert().success().get_output().stdout.to_owned();
        let content = String::from_utf8(out)?;

        assert_eq!(content, String::from(results[i]));
    }
    Ok(())
}

#[test]
fn test_byte_counts() -> Result<(), Box<dyn Error>> {
    let fixtures = [
        "tests/fixtures/ex1.txt",
        "tests/fixtures/ex2.txt",
        "tests/fixtures/ex3.txt",
    ];
    let results = ["57\n", "64\n", "89\n"];

    for i in 0..fixtures.len() {
        let mut cmd = Command::cargo_bin("wcrs")?;
        cmd.arg(fixtures[i]).arg("-c");
        let out = cmd.assert().success().get_output().stdout.to_owned();
        let content = String::from_utf8(out)?;

        assert_eq!(content, String::from(results[i]));
    }
    Ok(())
}
