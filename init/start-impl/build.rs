#![feature(exit_status_error)]
use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .stderr(Stdio::inherit()) // Prevents stderr being lost
        .output()
        .unwrap()
        .exit_ok()
        .expect("Git has failed. Review Stderr for more info");
    let git_commit =
        String::from_utf8(output.stdout).expect("Current git commit contains non utf-8 characters");

    println!("cargo:rustc-env=GIT_COMMIT={}", git_commit);
}
