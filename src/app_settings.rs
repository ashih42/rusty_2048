use std::env;

use crate::my_error::MyError;

/// This data object contains data that are used before starting App.
pub struct AppSettings {
    pub grid_size: (usize, usize),
    pub tty_path: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        const DEFAULT_NUM_ROWS: usize = 4;
        const DEFAULT_NUM_COLS: usize = 5;

        Self {
            grid_size: (DEFAULT_NUM_ROWS, DEFAULT_NUM_COLS),
            tty_path: None,
        }
    }
}

impl AppSettings {
    /// This constructor may fail if given invalid arguments.
    pub fn try_from_command_line() -> Result<Self, MyError> {
        let mut settings = Self::default();
        let args: Vec<String> = env::args().skip(1).collect();

        settings.parse_args(&args)?;
        Ok(settings)
    }

    /// Process each argument independently.
    /// If any argument is invalid, this operation returns an error.
    fn parse_args(&mut self, args: &[String]) -> Result<(), MyError> {
        let grid_size_prefix = "--grid=";
        let tty_prefix = "--tty=";

        for arg in args {
            match arg {
                _ if arg.starts_with(grid_size_prefix) => {
                    let input = &arg[grid_size_prefix.len()..];
                    self.grid_size = Self::parse_grid_size(input)?;
                }
                _ if arg.starts_with(tty_prefix) => {
                    let tty_path = &arg[tty_prefix.len()..];
                    self.tty_path = Some(tty_path.to_string());
                }
                _ => {
                    return Err(MyError::InvalidCommandLineArgumentsError);
                }
            };
        }

        Ok(())
    }

    /// Parse the grid dimensions (2 numbers).
    /// Example input format: "NUM_ROWS,NUM_COLS"
    fn parse_grid_size(input: &str) -> Result<(usize, usize), MyError> {
        let dimensions: Vec<&str> = input.split(',').collect();

        if dimensions.len() != 2 {
            return Err(MyError::InvalidCommandLineArgumentsError);
        }

        let num_rows = Self::parse_grid_dimension(dimensions[0])?;
        let num_cols = Self::parse_grid_dimension(dimensions[1])?;

        Ok((num_rows, num_cols))
    }

    // Parse a single grid dimension, which must be a positive integer >= 2.
    fn parse_grid_dimension(input: &str) -> Result<usize, MyError> {
        match input.trim().parse::<usize>() {
            Err(_) => Err(MyError::GridDimensionError(format!(
                "Invalid grid dimension: {}\nGrid dimension must be an integer that is 2 or larger.",
                input
            ))),
            Ok(0 | 1) => Err(MyError::GridDimensionError(format!(
                "Invalid grid dimension: {}\nGrid dimension must be an integer that is 2 or larger.",
                input,
            ))),
            Ok(n) => Ok(n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_app_settings() {
        let settings = AppSettings::default();

        assert_eq!(settings.grid_size, (4, 5));
        assert_eq!(settings.tty_path, None);
    }

    #[test]
    fn test_parse_grid_size() {
        // Valid input
        {
            let mut settings = AppSettings::default();

            let args = vec!["--grid=3,4".to_string()];
            let result = settings.parse_args(&args);

            assert!(result.is_ok());
            assert_eq!(settings.grid_size, (3, 4));
        }

        // Invalid input: only 1 token
        {
            let mut settings = AppSettings::default();

            let args = vec!["--grid=what".to_string()];
            let result = settings.parse_args(&args);

            assert!(result.is_err());
        }

        // Invalid input: 3 tokens
        {
            let mut settings = AppSettings::default();

            let args = vec!["--grid=3,4,5".to_string()];
            let result = settings.parse_args(&args);

            assert!(result.is_err());
        }

        // Invalid input: 2 non-integers
        {
            let mut settings = AppSettings::default();

            let args = vec!["--grid=2.4,6.8".to_string()];
            let result = settings.parse_args(&args);

            assert!(result.is_err());
        }

        // Invalid input: 2 non-integers
        {
            let mut settings = AppSettings::default();

            let args = vec!["--grid=2.4,6.8".to_string()];
            let result = settings.parse_args(&args);

            assert!(result.is_err());
        }

        // Invalid input: 2 integers that are too small
        {
            let mut settings = AppSettings::default();

            let args = vec!["--grid=1,1".to_string()];
            let result = settings.parse_args(&args);

            assert!(result.is_err());
        }
    }
}
