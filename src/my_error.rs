use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MyError {
    InvalidCommandLineArgumentError { arg: String },
    GridDimensionError(String),
    LoggerInitializationError { tty_path: String },
    SaveFileNotFoundError { save_file_path: String },
    SaveFileFailedToLoadError { save_file_path: String },
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCommandLineArgumentError { arg } => {
                write!(f, "Invalid command line argument: {}", arg)
            }
            Self::GridDimensionError(s) => write!(f, "{}", s),
            Self::LoggerInitializationError { tty_path, .. } => {
                write!(
                    f,
                    "Could not initialize logger to write to TTY: {}",
                    tty_path
                )
            }
            Self::SaveFileNotFoundError { save_file_path } => {
                write!(f, "Could not locate save file: {}", save_file_path)
            }
            Self::SaveFileFailedToLoadError { save_file_path } => write!(
                f,
                "Failed to load from corrupted save file: {}",
                save_file_path
            ),
        }
    }
}

impl Error for MyError {}
