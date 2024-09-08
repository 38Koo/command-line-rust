// cargo test --test echor_test
type TestResult = Result<(), Box<dyn std::error::Error>>;


#[test]
fn failed_no_args() -> TestResult {
  assert_cmd::Command::cargo_bin("echor")?
    .assert()
    .failure()
    .stderr(predicates::prelude::predicate::str::contains("USAGE"));
  Ok(())
}

fn runs(args: &[&str], expected_file: &str) -> TestResult{
  let expected = std::fs::read_to_string(expected_file)?;
  assert_cmd::Command::cargo_bin("echor")?
    .args(args)
    .assert()
    .success()
    .stdout(expected);
  Ok(())
}

#[test]
fn hello1() -> TestResult {
  runs(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
  runs(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
  runs(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
  runs(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
