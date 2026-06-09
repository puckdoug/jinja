use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("jinja"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_simple_template_stdin() {
    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("template.j2")
        .write_stdin("Hello {{ NAME }}!")
        .env("NAME", "World")
        .assert()
        .success()
        .stdout("Hello World!");
}

#[test]
fn test_multiple_templates() {
    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("t1.j2")
        .arg("t2.j2")
        .write_stdin("{{ VAR1 }} and {{ VAR2 }}")
        .env("VAR1", "foo")
        .env("VAR2", "bar")
        .assert()
        .success()
        .stdout("foo and bar");
}

#[test]
fn test_input_file_flag() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.path().join("input.txt");
    fs::write(&input_file, "Hello {{ NAME }}!").unwrap();

    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("template.j2")
        .arg("--input")
        .arg(&input_file)
        .env("NAME", "World")
        .assert()
        .success()
        .stdout("Hello World!");
}

#[test]
fn test_output_file_flag() {
    let temp = TempDir::new().unwrap();
    let output_file = temp.path().join("output.txt");

    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("template.j2")
        .arg("--output")
        .arg(&output_file)
        .write_stdin("Hello {{ NAME }}!")
        .env("NAME", "World")
        .assert()
        .success();

    let content = fs::read_to_string(&output_file).unwrap();
    assert_eq!(content, "Hello World!");
}

#[test]
fn test_substitutions_file() {
    let temp = TempDir::new().unwrap();
    let subs_file = temp.path().join("vars.txt");
    fs::write(&subs_file, "NAME=Alice\nAGE=30").unwrap();

    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("template.j2")
        .arg("--substitutions")
        .arg(&subs_file)
        .write_stdin("{{ NAME }} is {{ AGE }} years old")
        .assert()
        .success()
        .stdout("Alice is 30 years old");
}

#[test]
fn test_substitutions_override_env() {
    let temp = TempDir::new().unwrap();
    let subs_file = temp.path().join("vars.txt");
    fs::write(&subs_file, "NAME=Bob").unwrap();

    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("template.j2")
        .arg("--substitutions")
        .arg(&subs_file)
        .write_stdin("Hello {{ NAME }}!")
        .env("NAME", "World")
        .assert()
        .success()
        .stdout("Hello Bob!");
}

#[test]
fn test_verbose_flag() {
    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("template.j2")
        .arg("--verbose")
        .write_stdin("Hello {{ NAME }}!")
        .env("NAME", "World")
        .assert()
        .success()
        .stdout("Hello World!");
}

#[test]
fn test_debug_flag() {
    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("template.j2")
        .arg("--debug")
        .write_stdin("Hello {{ NAME }}!")
        .env("NAME", "World")
        .assert()
        .success()
        .stdout("Hello World!");
}

#[test]
fn test_fixture_file() {
    let mut cmd = Command::cargo_bin("jinja").unwrap();
    cmd.arg("tests/fixtures/foo.j2")
        .env("FOO", "bar")
        .assert()
        .success()
        .stdout("foo is bar\n");
}
