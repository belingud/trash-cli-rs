use std::env;
use std::path::Path;
use std::process;
use trash::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: trash <file> [<file> ...]");
        process::exit(1);
    }

    let mut has_error = false;

    for path in &args[1..] {
        // First check if the file exists
        if !Path::new(path).exists() {
            eprintln!("trash: '{}' path not exists", path);
            has_error = true;
            continue;
        }

        // Try to delete the file
        match trash::delete(path) {
            Ok(_) => {
                // Success - no output as per convention
            }
            Err(e) => {
                let msg = format_error(&e, path);
                eprintln!("trash: {}", msg);
                has_error = true;
            }
        }
    }

    if has_error {
        process::exit(1);
    }
}

fn format_error(err: &Error, path: &str) -> String {
    match err {
        Error::CouldNotAccess { target: _ } => {
            format!("'{}' path not exists", path)
        }
        Error::CanonicalizePath { original: _ } => {
            format!("'{}' path not exists", path)
        }
        Error::Unknown { description } => description.clone(),
        Error::Os { description, .. } => description.clone(),
        #[cfg(all(
            unix,
            not(target_os = "macos"),
            not(target_os = "ios"),
            not(target_os = "android")
        ))]
        Error::FileSystem { path: _, source } => {
            format!("'{}': {}", path, source)
        }
        Error::TargetedRoot => "cannot remove root directory".to_string(),
        Error::ConvertOsString { original } => {
            format!("invalid path: {:?}", original)
        }
        Error::RestoreCollision { .. } => "restore collision".to_string(),
        Error::RestoreTwins { .. } => "restore twins".to_string(),
    }
}
