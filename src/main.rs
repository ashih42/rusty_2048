use rusty_2048::{
    app::App, app_settings::AppSettings, logger::initialize_logger, my_error::MyError,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        eprintln!("\n{}", AppSettings::get_usage());
    }
}

fn run() -> Result<(), MyError> {
    // 1. Parse settings from command line arguments.
    let settings = AppSettings::try_from_command_line()?;

    if settings.help {
        println!("{}", AppSettings::get_usage());
        return Ok(());
    }

    // 2. Initialize logger.
    initialize_logger(&settings.tty_path)?;

    // 3. Build app.
    let mut app = build_app(&settings)?;

    // 4. Run app until user input to exit.
    if let Err(err) = ratatui::run(|terminal| app.run(terminal)) {
        log::error!("ratatui ran into IO error: {}", err);
    }
    Ok(())
}

fn build_app(settings: &AppSettings) -> Result<App, MyError> {
    if settings.load_from_save_file {
        App::try_from_save_file()
    } else {
        let (num_rows, num_cols) = settings.grid_size;
        Ok(App::new(num_rows, num_cols))
    }
}
