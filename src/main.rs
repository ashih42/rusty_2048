use rusty_2048::{app::App, app_settings::AppSettings};

fn main() {
    if let Err(err) = App::run() {
        eprintln!("Error: {}", err);
        eprintln!("\n{}", AppSettings::get_usage());
    }
}
