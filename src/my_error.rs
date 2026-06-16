use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MyError {
    InvalidCommandLineArgument { arg: String },
    InvalidGridSize { expr: String },
    LoggerInitializationFailed { tty_path: String },
    SaveFileNotFound { save_file_path: String },
    SaveFileFailedToLoad { save_file_path: String },
    SaveFileFailedToSave { save_file_path: String },
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCommandLineArgument { arg } => {
                write!(f, "Invalid command line argument: {arg}")
            }
            Self::InvalidGridSize { expr } => write!(
                f,
                "Invalid grid size: {expr}\nGrid size must be two integers >= 2"
            ),
            Self::LoggerInitializationFailed { tty_path, .. } => {
                write!(f, "Could not initialize logger to TTY: {tty_path}")
            }
            Self::SaveFileNotFound { save_file_path } => {
                write!(f, "Could not find save file: {save_file_path}")
            }
            Self::SaveFileFailedToLoad { save_file_path } => write!(
                f,
                "Failed to load from corrupted save file: {save_file_path}"
            ),
            Self::SaveFileFailedToSave { save_file_path } => {
                write!(f, "Failed to save to file: {save_file_path}")
            }
        }
    }
}

impl Error for MyError {}
