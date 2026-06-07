use std::io;

use rusty_2048::{app::App, my_error::MyError, parser};

fn main() -> io::Result<()> {
    match parser::parse_settings_from_command_line() {
        Ok(settings) => {
            let (num_rows, num_cols) = settings;
            ratatui::run(|terminal| App::new(num_rows, num_cols).run(terminal))
        }
        Err(err) => {
            print_error_and_usage(err);
            Ok(())
        }
    }
}

fn print_error_and_usage(err: MyError) {
    eprintln!("Error: {}", err);
    eprintln!("usage: cargo run -- [<num_rows> <num_cols>]");
}
