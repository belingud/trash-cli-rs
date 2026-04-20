use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_trash")
}

fn run_trash(args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .args(args)
        .output()
        .expect("failed to run trash binary")
}

fn run_trash_in(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .current_dir(dir)
        .args(args)
        .output()
        .expect("failed to run trash binary")
}

fn make_tempdir() -> TempDir {
    TempDir::new().expect("failed to create temp dir")
}

fn write_file(path: &Path) {
    fs::write(path, b"test data").expect("failed to write file")
}

fn create_directory(path: &Path) {
    fs::create_dir_all(path).expect("failed to create directory")
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr should be utf-8")
}

fn path_str(path: &Path) -> &str {
    path.to_str().expect("path should be utf-8")
}

#[test]
fn trashes_files_when_supported_rm_flags_are_ignored() {
    let tempdir = make_tempdir();
    let file1 = tempdir.path().join("file1.txt");
    let file2 = tempdir.path().join("file2.txt");
    write_file(&file1);
    write_file(&file2);

    let output = run_trash(&["-rfxP", path_str(&file1), path_str(&file2)]);

    assert!(output.status.success());
    assert!(!file1.exists());
    assert!(!file2.exists());
}

#[test]
fn trashes_directory_when_recursive_compat_flags_are_ignored() {
    let tempdir = make_tempdir();
    let dir = tempdir.path().join("build");
    let nested = dir.join("nested.txt");
    create_directory(&dir);
    write_file(&nested);

    let output = run_trash(&["--force", "--recursive", path_str(&dir)]);

    assert!(output.status.success());
    assert!(!dir.exists());
}

#[test]
fn keeps_processing_operands_after_a_missing_path_error() {
    let tempdir = make_tempdir();
    let existing = tempdir.path().join("existing.txt");
    let missing = tempdir.path().join("missing.txt");
    write_file(&existing);

    let output = run_trash(&[path_str(&existing), path_str(&missing)]);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(1));
    assert!(!existing.exists());
    assert!(stderr.contains(&format!(
        "trash: {}: No such file or directory",
        path_str(&missing)
    )));
}

#[test]
fn treats_unsupported_short_flag_as_a_literal_operand() {
    let tempdir = make_tempdir();
    let existing = tempdir.path().join("existing.txt");
    write_file(&existing);

    let output = run_trash(&["-W", path_str(&existing)]);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(1));
    assert!(!existing.exists());
    assert!(stderr.contains("trash: -W: No such file or directory"));
}

#[test]
fn allows_dash_prefixed_names_after_double_dash() {
    let tempdir = make_tempdir();
    let dash_file = tempdir.path().join("-rf");
    write_file(&dash_file);

    let output = run_trash_in(tempdir.path(), &["--", "-rf"]);

    assert!(output.status.success());
    assert!(!dash_file.exists());
}

#[test]
fn allows_dash_prefixed_names_via_explicit_relative_path() {
    let tempdir = make_tempdir();
    let dash_file = tempdir.path().join("-file");
    write_file(&dash_file);

    let output = run_trash_in(tempdir.path(), &["./-file"]);

    assert!(output.status.success());
    assert!(!dash_file.exists());
}

#[test]
fn treats_unsupported_long_flag_as_a_literal_operand() {
    let tempdir = make_tempdir();
    let existing = tempdir.path().join("existing.txt");
    write_file(&existing);

    let output = run_trash(&["--interactive=maybe", path_str(&existing)]);
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(1));
    assert!(!existing.exists());
    assert!(stderr.contains("trash: --interactive=maybe: No such file or directory"));
}
