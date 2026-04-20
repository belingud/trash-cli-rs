use std::env;
use std::path::Path;
use std::process;
use trash::Error;

const ERROR_PREFIX: &str = "trash";
const USAGE: &str = "Usage: trash [rm-compat-options...] <file> [<file> ...]";
const HELP_TEXT: &str = "\
trash-cli-rs

Move files and directories to the system trash.

This tool is not a semantic replacement for rm.
Documented rm-style compatibility flags are ignored for alias compatibility only.
They do not enable force deletion, interactive prompts, secure erase, or recursive traversal semantics.

Usage:
  trash [rm-compat-options...] <file> [<file> ...]
  trash --help
  trash --version

rm alias compatibility:
  Supported compatibility flags are ignored before operands.
  Unknown flags are treated as literal operands.
  To trash a filename that starts with '-', use:
    trash -- -rf
    trash ./-rf
";
const RM_COMPAT_SHORT_FLAGS: &[char] = &['d', 'f', 'i', 'I', 'P', 'R', 'r', 'v', 'x'];
const RM_COMPAT_LONG_FLAGS: &[&str] = &[
    "--dir",
    "--force",
    "--interactive",
    "--interactive=always",
    "--interactive=never",
    "--interactive=once",
    "--no-preserve-root",
    "--one-file-system",
    "--preserve-root",
    "--preserve-root=all",
    "--recursive",
    "--verbose",
];

#[derive(Debug, PartialEq, Eq)]
enum CommandMode {
    Run,
    Help,
    Version,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match command_mode(&args) {
        CommandMode::Help => {
            println!("{HELP_TEXT}");
            return;
        }
        CommandMode::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            return;
        }
        CommandMode::Run => {}
    }

    let exit_code = run(&args);

    if exit_code != 0 {
        process::exit(exit_code);
    }
}

fn run(args: &[String]) -> i32 {
    let operands = filter_operands(args);

    if operands.is_empty() {
        eprintln!("{USAGE}");
        return 1;
    }

    let mut has_error = false;

    for path in operands {
        // First check if the file exists
        if !Path::new(path).exists() {
            eprintln!("{ERROR_PREFIX}: {path}: {}", missing_path_message());
            has_error = true;
            continue;
        }

        // Try to delete the file
        match trash::delete(path) {
            Ok(_) => {
                // Success - no output as per convention
            }
            Err(e) => {
                let msg = format_error(&e);
                eprintln!("{ERROR_PREFIX}: {path}: {msg}");
                has_error = true;
            }
        }
    }

    if has_error {
        1
    } else {
        0
    }
}

fn command_mode(args: &[String]) -> CommandMode {
    for arg in args {
        match arg.as_str() {
            "--" => break,
            "-h" | "--help" => return CommandMode::Help,
            "-V" | "--version" => return CommandMode::Version,
            _ => {}
        }
    }

    CommandMode::Run
}

fn filter_operands(args: &[String]) -> Vec<&str> {
    let mut operands = Vec::new();
    let mut parse_compat_flags = true;

    for arg in args {
        let arg = arg.as_str();

        if parse_compat_flags {
            if arg == "--" {
                parse_compat_flags = false;
                continue;
            }

            if is_rm_compat_flag(arg) {
                continue;
            }
        }

        operands.push(arg);
    }

    operands
}

fn is_rm_compat_flag(arg: &str) -> bool {
    RM_COMPAT_LONG_FLAGS.contains(&arg) || is_short_rm_compat_cluster(arg)
}

fn is_short_rm_compat_cluster(arg: &str) -> bool {
    let Some(flags) = arg.strip_prefix('-') else {
        return false;
    };

    if flags.is_empty() || flags.starts_with('-') {
        return false;
    }

    flags
        .chars()
        .all(|flag| RM_COMPAT_SHORT_FLAGS.contains(&flag))
}

fn missing_path_message() -> &'static str {
    "No such file or directory"
}

fn format_error(err: &Error) -> String {
    match err {
        Error::CouldNotAccess { target: _ } => missing_path_message().to_string(),
        Error::CanonicalizePath { original: _ } => missing_path_message().to_string(),
        Error::Unknown { description } => description.clone(),
        Error::Os { description, .. } => description.clone(),
        #[cfg(all(
            unix,
            not(target_os = "macos"),
            not(target_os = "ios"),
            not(target_os = "android")
        ))]
        Error::FileSystem { path: _, source } => source.to_string(),
        Error::TargetedRoot => "cannot remove root directory".to_string(),
        Error::ConvertOsString { original } => format!("invalid path: {:?}", original),
        Error::RestoreCollision { .. } => "restore collision".to_string(),
        Error::RestoreTwins { .. } => "restore twins".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{command_mode, filter_operands, run, CommandMode};

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn detects_help_before_operands() {
        let args = args(&["--help"]);

        assert_eq!(command_mode(&args), CommandMode::Help);
    }

    #[test]
    fn detects_version_before_operands() {
        let args = args(&["--version"]);

        assert_eq!(command_mode(&args), CommandMode::Version);
    }

    #[test]
    fn treats_help_after_double_dash_as_operand() {
        let args = args(&["--", "--help"]);

        assert_eq!(command_mode(&args), CommandMode::Run);
    }

    #[test]
    fn ignores_supported_short_flag_clusters() {
        let args = args(&["-rfxP", "file1", "-v", "file2"]);

        assert_eq!(filter_operands(&args), vec!["file1", "file2"]);
    }

    #[test]
    fn ignores_supported_long_flags() {
        let args = args(&[
            "--force",
            "--recursive",
            "--interactive=once",
            "--preserve-root=all",
            "file.txt",
        ]);

        assert_eq!(filter_operands(&args), vec!["file.txt"]);
    }

    #[test]
    fn keeps_unknown_dashed_args() {
        let args = args(&["--bogus", "-W", "--interactive=maybe", "file.txt"]);

        assert_eq!(
            filter_operands(&args),
            vec!["--bogus", "-W", "--interactive=maybe", "file.txt"]
        );
    }

    #[test]
    fn treats_args_after_double_dash_as_literal_operands() {
        let args = args(&["--", "-rf", "--force"]);

        assert_eq!(filter_operands(&args), vec!["-rf", "--force"]);
    }

    #[test]
    fn keeps_explicit_paths_for_dash_prefixed_filenames() {
        let args = args(&["./-rf", "../-f", "/tmp/-v"]);

        assert_eq!(filter_operands(&args), vec!["./-rf", "../-f", "/tmp/-v"]);
    }

    #[test]
    fn returns_error_when_only_compat_flags_are_provided() {
        let args = args(&["-rf", "--force"]);

        assert_eq!(run(&args), 1);
    }
}
