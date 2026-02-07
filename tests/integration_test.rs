use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

#[test]
fn test_svg_to_jpeg_conversion() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_resvg-jpeg"));
    let input_path = Path::new("tests/test.svg");
    let output_path = Path::new("tests/output.jpg");

    // Ensure output doesn't exist
    if output_path.exists() {
        std::fs::remove_file(output_path).unwrap();
    }

    cmd.arg("--input")
        .arg(input_path)
        .arg("--output")
        .arg(output_path)
        .arg("--width")
        .arg("100")
        .arg("--quality")
        .arg("85")
        .arg("--background")
        .arg("white")
        .assert()
        .success();

    assert!(output_path.exists());

    // Clean up
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_invalid_quality() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_resvg-jpeg"));
    cmd.arg("--input")
        .arg("tests/test.svg")
        .arg("--quality")
        .arg("101")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Warning: Invalid quality '101'. Using default 80.",
        ));
}

#[test]
fn test_invalid_background() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_resvg-jpeg"));
    cmd.arg("--input")
        .arg("tests/test.svg")
        .arg("--background")
        .arg("not-a-color")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Warning: Failed to parse background color 'not-a-color'. Using default white.",
        ));
}

#[test]
fn test_invalid_width() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_resvg-jpeg"));
    cmd.arg("--input")
        .arg("tests/test.svg")
        .arg("--width")
        .arg("0")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Width must be greater than 0"));
}

#[test]
fn test_missing_input_file() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_resvg-jpeg"));
    cmd.arg("--input")
        .arg("tests/non_existent.svg")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read input file"));
}
