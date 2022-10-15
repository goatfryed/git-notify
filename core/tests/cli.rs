use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands

#[test]
fn confirm_help() {
    run_baseline_test(
        "confirm_help",
        ["confirm", "--help"]
    );
}

#[test]
fn notify_help() {
    run_baseline_test(
        "notify_help",
        ["--help"]
    );
}

#[test]
fn notify_me_help() {
    run_baseline_test(
        "notify_me_help",
        ["me", "--help"]
    );
}

#[test]
fn notify_me() {
    run_baseline_test(
        "notify_me",
        ["me"]
    );
}

#[test]
fn watch_file() {
    run_baseline_test(
        "watch_file",
        ["watch", "README.md"]
    );
}

fn run_baseline_test<'a,I>(test_name: &str, args: I)
where I: IntoIterator<Item = &'a str>
{
    let mut cmd = Command::cargo_bin("git-notify")
        .expect("git-notify command not found");

    cmd.args(args);

    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/baseline")
        .join(test_name);


    let baseline_file = test_dir.join("expected.out");
    let output_file = test_dir.join("actual.out");

    let stdout = OpenOptions::new()
        .write(true).create(true).truncate(true)
        .open(&output_file)
        .unwrap_or_else(|e| panic!("failed to open output file {}\n{}", output_file.display(), e));

    cmd.current_dir(test_dir)
        .stdout(stdout)
        .assert().success();

    let expected_out_filepath = baseline_file.to_str().unwrap();
    let actual = fs::read_to_string(output_file.as_path()).expect("no actual.out generated");
    let expected = fs::read_to_string(baseline_file.as_path()).expect("no expected.out defined");

    assert_eq!(expected, actual, "unexpected output. Check {}", expected_out_filepath)
}