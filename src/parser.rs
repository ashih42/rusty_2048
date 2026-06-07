use std::env;

use crate::my_error::MyError;

/// The user can specify the grid dimensions (num_rows, num_cols) by providing either:
///   - 0 arguments (default dimensions are used instead).
///   - 2 arguments, where each value is an integer >= 2.
///
/// # Examples
/// ```ignore
/// cargo run
/// cargo run -- 3 4
/// ```
pub fn parse_settings_from_command_line() -> Result<(usize, usize), MyError> {
    const DEFAULT_NUM_ROWS: usize = 4;
    const DEFAULT_NUM_COLS: usize = 5;

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        Ok((DEFAULT_NUM_ROWS, DEFAULT_NUM_COLS))
    } else if args.len() == 2 {
        let num_rows = parse_grid_dimension(&args[0])?;
        let num_cols = parse_grid_dimension(&args[1])?;
        Ok((num_rows, num_cols))
    } else {
        Err(MyError::InvalidCommandLineArgumentsError)
    }
}

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
