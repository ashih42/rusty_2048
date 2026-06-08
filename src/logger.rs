use env_logger::{Builder, Target};
use std::{
    fs::{File, OpenOptions},
    io::IsTerminal,
    path::Path,
};

use crate::my_error::MyError;

/// Initialize env_logger, logging to a specific TTY device if provided.
pub fn initialize_logger(tty_path: &Option<String>) -> Result<(), MyError> {
    match tty_path {
        Some(path) => {
            if !is_valid_tty(path) {
                return Err(MyError::LoggerInitializationError(path.clone()));
            }

            let tty_file =
                File::create(path).map_err(|_| MyError::LoggerInitializationError(path.clone()))?;

            Builder::new()
                .parse_default_env()
                .target(Target::Pipe(Box::new(tty_file)))
                .init();
        }
        None => {
            env_logger::init();
        }
    }

    Ok(())
}

/// Check if the given path is a valid path to a useable TTY.
fn is_valid_tty(path_str: &str) -> bool {
    let path = Path::new(path_str);

    // 1. Check if the path physically exists.
    if !path.try_exists().unwrap_or(false) {
        return false;
    }

    // 2. Open the file in a non-blocking or standard read/write mode,
    // using OpenOptions to avoid blocking on certain physical TTY lines.
    match OpenOptions::new().read(true).write(true).open(path) {
        Ok(file) => {
            // 3. Verify the file descriptor represents a TTY terminal.
            file.is_terminal()
        }
        Err(_) => {
            // Path exists, but cannot open it (e.g., Permission Denied).
            false
        }
    }
}
