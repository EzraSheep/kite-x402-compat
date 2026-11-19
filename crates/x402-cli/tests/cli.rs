//! Smoke tests for the binary itself.

use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_kite-x402-compat"))
}

#[test]
fn version_flag_prints_package_version() {
    let out = bin().arg("--version").output().expect("run binary");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains(env!("CARGO_PKG_VERSION")), "got: {text}");
}

#[test]
fn unknown_argument_is_a_usage_error() {
    let out = bin().arg("--no-such-flag").output().expect("run binary");
    assert_eq!(out.status.code(), Some(2));
}
