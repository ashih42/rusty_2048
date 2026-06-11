use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{Terminal, backend::CrosstermBackend};
use savefile::{self, load_file, save_file};
use std::fs;
use std::io::{self, Stdout};

use crate::{
    app_settings::AppSettings, app_state::AppState, logger, move_direction::MoveDirection,
    my_error::MyError,
};

const DEFAULT_SAVE_FILE_PATH: &str = "save.bin";

pub struct App {
    should_exit: bool,
    state: AppState,
}

/// These are associated functions.
impl App {
    /// This is the public interface to indirectly create and run an app.
    ///
    /// If user provided help flag, simply print usage and exit.
    /// Otherwise, create an app instance and run.
    pub fn run() -> Result<(), MyError> {
        // 1. Parse settings from command line arguments.
        let settings = AppSettings::try_from_command_line()?;

        // If --help flag is provided, simply print usage and end here.
        if settings.help {
            println!("{}", AppSettings::get_usage());
            return Ok(());
        }

        // 2. Initialize logger.
        logger::initialize_logger(&settings.tty_path)?;

        // 3. Build app.
        let mut app = Self::try_build(&settings)?;

        // 4. Run app until user input to exit.
        if let Err(err) = ratatui::run(|terminal| app.run_in_ratatui(terminal)) {
            log::error!("ratatui ran into IO error: {}", err);
        }
        Ok(())
    }

    /// Either load an AppState from save file or create a new AppState with specific grid size.
    fn try_build(settings: &AppSettings) -> Result<Self, MyError> {
        let state = if settings.load_from_save_file {
            Self::try_load_from_save_file(DEFAULT_SAVE_FILE_PATH)?
        } else {
            let (num_rows, num_cols) = settings.grid_size;
            AppState::new(num_rows, num_cols)
        };

        Ok(Self {
            should_exit: false,
            state,
        })
    }

    /// Construct an AppState from save file if the file exists and is valid.
    fn try_load_from_save_file(path: &str) -> Result<AppState, MyError> {
        // 1. Check if save file exists.
        match fs::exists(path) {
            Ok(true) => (),
            Ok(false) | Err(_) => {
                return Err(MyError::SaveFileNotFoundError {
                    save_file_path: path.to_owned(),
                });
            }
        }

        // 2. Read the file contents to construct an AppState instance.
        load_file(path, 0).map_err(|_| MyError::SaveFileFailedToLoadError {
            save_file_path: path.to_owned(),
        })
    }
}

/// These are instance methods.
impl App {
    /// With `terminal` provided by ratatui, this runs forever until user input to exit.
    /// It is not clear what kind of IO error might occur when drawing to terminal or listening for events.
    fn run_in_ratatui(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.state.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Use crossterm to listen for user input from pressing down a key.
    fn handle_events(&mut self) -> io::Result<()> {
        // Note: This is BLOCKING until an event occurs.
        // This is fine for now, but this will need to be non-blocking for
        // future animations or AI auto-play features
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => (),
        };
        Ok(())
    }

    /// Listen to specific key input events.
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.save_and_exit(),
            KeyCode::Char('r') => self.state.restart(),
            KeyCode::Char('g') => self.state.toggle_grid(),
            KeyCode::Char('w') | KeyCode::Up => self.state.update(MoveDirection::Up),
            KeyCode::Char('s') | KeyCode::Down => self.state.update(MoveDirection::Down),
            KeyCode::Char('a') | KeyCode::Left => self.state.update(MoveDirection::Left),
            KeyCode::Char('d') | KeyCode::Right => self.state.update(MoveDirection::Right),
            _ => (),
        }
    }

    /// Save state to file and then exit.
    fn save_and_exit(&mut self) {
        if let Err(err) = save_file(DEFAULT_SAVE_FILE_PATH, 0, &self.state) {
            log::error!("Failed to save game to file: {}", err);
        }

        self.should_exit = true;
    }
}
