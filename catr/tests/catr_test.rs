use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/one-line.txt";
const SPIDERS: &str = "tests/inputs/three-lines.txt";
const BUSTLE: &str = "tests/inputs/multiple-lines.txt";


// --------------------------------------------------
#[test]
fn usage() -> Result<()> {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn skips_bad_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?.args(args).output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

// --------------------------------------------------
fn run_stdin(
    input_file: &str,
    args: &[&str],
    expected_file: &str,
) -> Result<()> {
    println!("{}", input_file);
    let input = fs::read_to_string(input_file)?;
    // println!("input: {}", input);
    let expected = fs::read_to_string(expected_file)?;    // inputを標準入力に渡す
    // println!("expected: {}", expected);
    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .output()
        .unwrap();
    println!("output: {:?}", output);
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    println!("stdout: {}", stdout);
    assert_eq!(stdout, expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn bustle_stdin() -> Result<()> {
    run_stdin(BUSTLE, &["-"], "tests/expected/multiple-lines.txt.stdin.out")
}

// --------------------------------------------------
#[test]
fn bustle_stdin_n() -> Result<()> {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected/multiple-lines.txt.n.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle_stdin_b() -> Result<()> {
    run_stdin(
        BUSTLE,
        &["-b", "-"],
        "tests/expected/multiple-lines.txt.b.stdin.out",
    )
}

// --------------------------------------------------
#[test]
fn empty() -> Result<()> {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

// --------------------------------------------------
#[test]
fn empty_n() -> Result<()> {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

// --------------------------------------------------
#[test]
fn empty_b() -> Result<()> {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

// --------------------------------------------------
#[test]
fn fox() -> Result<()> {
    run(&[FOX], "tests/expected/one-line.txt.out")
}

// --------------------------------------------------
#[test]
fn fox_n() -> Result<()> {
    run(&["-n", FOX], "tests/expected/one-line.txt.n.out")
}

// --------------------------------------------------
#[test]
fn fox_b() -> Result<()> {
    run(&["-b", FOX], "tests/expected/one-line.txt.b.out")
}

// --------------------------------------------------
#[test]
fn spiders() -> Result<()> {
    run(&[SPIDERS], "tests/expected/three-lines.txt.out")
}

// --------------------------------------------------
#[test]
fn spiders_n() -> Result<()> {
    run(&["--number", SPIDERS], "tests/expected/three-lines.txt.n.out")
}

// --------------------------------------------------
#[test]
fn spiders_b() -> Result<()> {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/expected/three-lines.txt.b.out",
    )
}

// --------------------------------------------------
#[test]
fn bustle() -> Result<()> {
    run(&[BUSTLE], "tests/expected/multiple-lines.txt.out")
}

// --------------------------------------------------
#[test]
fn bustle_n() -> Result<()> {
    run(&["-n", BUSTLE], "tests/expected/multiple-lines.txt.n.out")
}

// --------------------------------------------------
#[test]
fn bustle_b() -> Result<()> {
    run(&["-b", BUSTLE], "tests/expected/multiple-lines.txt.b.out")
}

// --------------------------------------------------
#[test]
fn all() -> Result<()> {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

// --------------------------------------------------
#[test]
fn all_n() -> Result<()> {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

// --------------------------------------------------
#[test]
fn all_b() -> Result<()> {
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
