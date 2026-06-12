use rusty_2048::{app::App, app_settings::AppSettings, my_error::MyError};

fn main() {
    if let Err(err) = App::run() {
        eprintln!("Error: {}", err);

        if matches!(err, MyError::InvalidCommandLineArgument { .. }) {
            eprintln!("\n{}", AppSettings::get_usage());
        }
    }
}
