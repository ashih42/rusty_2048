use indoc::indoc;
use std::env;

use crate::my_error::MyError;

/// AppSettings holds data that are used for initializing App, by parsing the command line arguments
/// for relevant flags.
///
/// Note: Some command line flags override other flags, if both are provided.
/// - --help overrides everything, only showing the usage page, not running the app.
/// - --load overrides --grid, loading the saved game (ignoring the specified grid size).
#[derive(Debug)]
pub struct AppSettings {
    pub help: bool,
    pub load_from_save_file: bool,
    pub grid_size: (usize, usize),
    pub tty_path: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        const DEFAULT_NUM_ROWS: usize = 4;
        const DEFAULT_NUM_COLS: usize = 5;

        Self {
            help: false,
            load_from_save_file: false,
            grid_size: (DEFAULT_NUM_ROWS, DEFAULT_NUM_COLS),
            tty_path: None,
        }
    }
}

impl AppSettings {
    /// This constructor may fail if given invalid arguments.
    pub fn try_from_command_line() -> Result<Self, MyError> {
        let mut settings = Self::default();

        for arg in env::args().skip(1) {
            settings.parse_arg(&arg)?;
        }
        Ok(settings)
    }

    /// Process a command line argument, which must always be a complete flag,
    /// with its values given in this format:
    /// ---flag=value1,value2
    fn parse_arg(&mut self, arg: &str) -> Result<(), MyError> {
        let help_flag = "--help";
        let load_flag = "--load";
        let grid_size_prefix = "--grid=";
        let tty_prefix = "--tty=";

        match arg {
            _ if arg == help_flag => {
                self.help = true;
            }
            _ if arg == load_flag => {
                self.load_from_save_file = true;
            }
            _ if arg.starts_with(grid_size_prefix) => {
                let input = &arg[grid_size_prefix.len()..];
                self.grid_size = Self::parse_grid_size(input)?;
            }
            _ if arg.starts_with(tty_prefix) => {
                let tty_path = &arg[tty_prefix.len()..];
                self.tty_path = Some(tty_path.to_owned());
            }
            _ => {
                return Err(MyError::InvalidCommandLineArgument {
                    arg: arg.to_owned(),
                });
            }
        };

        Ok(())
    }

    /// Parse the grid dimensions (2 numbers).
    /// Example input format: "NUM_ROWS,NUM_COLS"
    fn parse_grid_size(input: &str) -> Result<(usize, usize), MyError> {
        let dimensions: Vec<&str> = input.split(',').collect();

        if dimensions.len() != 2 {
            return Err(MyError::InvalidGridSize {
                expr: input.to_owned(),
            });
        }

        let num_rows = Self::parse_grid_dimension(dimensions[0])?;
        let num_cols = Self::parse_grid_dimension(dimensions[1])?;
        Ok((num_rows, num_cols))
    }

    // Parse a single grid dimension, which must be a positive integer >= 2.
    fn parse_grid_dimension(input: &str) -> Result<usize, MyError> {
        match input.trim().parse::<usize>() {
            Err(_) | Ok(0 | 1) => Err(MyError::InvalidGridSize {
                expr: input.to_owned(),
            }),
            Ok(n) => Ok(n),
        }
    }

    pub fn get_usage() -> &'static str {
        indoc! {"
            Usage:
            Run the game directly with optional command line flags.
            rusty_2048 [ <flag> ... ]
            
            Run the game from cargo with optional command line flags.
            cargo run -- [ <flag> ... ]

            Command line flags:
              --help                            Show this usage page.
              --load                            Load the game from save file.
              --grid=<num_rows>,<num_cols>      Start the game with a specific grid size.
              --tty=<tty_path>                  Enable logging to a specific tty.

            Keyboard Controls in Game:
              [ Q ]                             Close the app.
              [ R ]                             Start a new game.
              [ G ]                             Toggle grid visibility on/off.
              [ Z ]                             Toggle auto-play on/off.
              [ WASD ] or [ ARROW KEYS ]        Move all tiles toward a direction.
            
        "}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_size() {
        // Valid input
        {
            let mut settings = AppSettings::default();

            let result = settings.parse_arg("--grid=3,4");
            assert!(result.is_ok());
            assert_eq!(settings.grid_size, (3, 4));
        }

        // Invalid input: only 1 token
        {
            let mut settings = AppSettings::default();

            let result = settings.parse_arg("--grid=what");
            assert!(result.is_err());
        }

        // Invalid input: 3 tokens
        {
            let mut settings = AppSettings::default();

            let result = settings.parse_arg("--grid=3,4,5");
            assert!(result.is_err());
        }

        // Invalid input: 2 non-integers
        {
            let mut settings = AppSettings::default();

            let result = settings.parse_arg("--grid=2.4,6.8");
            assert!(result.is_err());
        }

        // Invalid input: 2 non-integers
        {
            let mut settings = AppSettings::default();

            let result = settings.parse_arg("--grid=2.4,6.8");
            assert!(result.is_err());
        }

        // Invalid input: 2 integers that are too small
        {
            let mut settings = AppSettings::default();

            let result = settings.parse_arg("--grid=1,1");
            assert!(result.is_err());
        }
    }
}
