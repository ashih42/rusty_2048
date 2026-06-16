use rusty_2048::{app::App, app_settings::AppSettings, my_error::MyError};

fn main() {
    if let Err(err) = App::run() {
        println!("Error: {err}");

        if matches!(err, MyError::InvalidCommandLineArgument { .. }) {
            println!("\n{}", AppSettings::get_usage());
        }
    }
}
