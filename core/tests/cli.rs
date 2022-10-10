use std::env::set_current_dir;
use std::fs::OpenOptions;
use std::path::Path;
use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use filecmp::cmp;

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

fn run_baseline_test<'a,I>(test_name: &str, args: I)
where I: IntoIterator<Item = &'a str>
{
    let mut cmd = Command::cargo_bin("git-notify")
        .expect("git-notify command not found");

    cmd.args(args);

    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/baseline")
        .join(test_name);

    set_current_dir(&test_dir)
        .expect(&format!("couldn't change working dir to {}", test_dir.display()));

    let baseline_file = Path::new("./expected.out");
    let output_file = Path::new("./actual.out");

    let stdout = OpenOptions::new()
        .write(true).create(true).truncate(true)
        .open(&output_file)
        .unwrap_or_else(|e| panic!("failed to open output file {}\n{}", output_file.display(), e));

    cmd.stdout(stdout)
        .assert().success();

    let is_equal = cmp(&output_file, &baseline_file, false)
        .unwrap();

    assert!(is_equal, "unexpected output. Check {}", output_file.display())
}