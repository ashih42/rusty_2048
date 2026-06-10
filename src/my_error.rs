use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MyError {
    InvalidCommandLineArgumentsError,
    GridDimensionError(String),
    LoggerInitializationError(String),
    SaveDataError { save_file_path: String },
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCommandLineArgumentsError => write!(f, "Invalid command line arguments."),
            Self::GridDimensionError(s) => write!(f, "{}", s),
            Self::LoggerInitializationError(tty_path) => {
                write!(
                    f,
                    "Could not initialize logger to write to TTY: {}",
                    tty_path
                )
            }
            Self::SaveDataError { save_file_path } => write!(
                f,
                "Failed to load from save data\n\
                Recommendation: Delete the corrupted save data: {}",
                save_file_path
            ),
        }
    }
}

impl Error for MyError {}
