use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MyError {
    InvalidCommandLineArgumentsError,
    GridDimensionError(String),
    LoggerInitializationError(String),
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
        }
    }
}

impl Error for MyError {}
