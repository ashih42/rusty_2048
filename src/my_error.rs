use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MyError {
    InvalidCommandLineArgumentsError,
    GridDimensionError(String),
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCommandLineArgumentsError => write!(f, "Invalid command line arguments."),
            Self::GridDimensionError(s) => write!(f, "{}", s),
        }
    }
}

impl Error for MyError {}
