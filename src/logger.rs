use env_logger::{Builder, Target};
use std::{
    fs::{File, OpenOptions},
    io::IsTerminal,
    path::Path,
};

use crate::my_error::MyError;

/// Initialize `env_logger`, logging to a specific TTY device if provided.
pub fn initialize_logger(tty_path: Option<&String>) -> Result<(), MyError> {
    match tty_path {
        Some(path) => {
            if !is_valid_tty(path) {
                return Err(MyError::LoggerInitializationFailed {
                    tty_path: path.clone(),
                });
            }

            let tty_file = File::create(path).map_err(|_| MyError::LoggerInitializationFailed {
                tty_path: path.clone(),
            })?;

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
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .is_ok_and(|file| file.is_terminal())
}
