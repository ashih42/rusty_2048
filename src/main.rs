use rusty_2048::{
    app::App, app_settings::AppSettings, logger::initialize_logger, my_error::MyError,
};

fn main() {
    if let Err(err) = run() {
        print_error_and_usage(err);
    }
}

fn run() -> Result<(), MyError> {
    let settings = AppSettings::try_from_command_line()?;
    initialize_logger(&settings.tty_path)?;

    let (num_rows, num_cols) = settings.grid_size;
    log::info!("Launching rusty_2048 with grid size: ({num_rows}, {num_cols})");

    if let Err(err) = ratatui::run(|terminal| App::new(num_rows, num_cols).run(terminal)) {
        log::error!("ratatui ran into IO error: {}", err);
    }

    Ok(())
}

fn print_error_and_usage(err: MyError) {
    eprintln!("Error: {}", err);
    eprintln!("usage: cargo run -- [--grid=<num_rows>,<num_cols>] [--tty=<tty_path>]");
}
