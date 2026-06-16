use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{Terminal, backend::CrosstermBackend};
use savefile::{self, load_file, save_file};
use std::fs;
use std::io::{self, Stdout};
use std::time::Duration;

use crate::bounded_stack::BoundedStack;
use crate::renderer::Renderer;
use crate::solver::Solver;
use crate::{
    app_settings::AppSettings, app_state::AppState, logger, move_direction::MoveDirection,
    my_error::MyError,
};

const DEFAULT_SAVE_FILE_PATH: &str = "save.bin";
const DEFAULT_OLD_STATES_STACK_CAPACITY: usize = 3;

/// `App` is reponsible for initializing, loading, saving, and restoring its `AppState`,
/// agnostic of the business logic inside `AppState`.
///
/// Also, `App` handles the main loop, which draws to terminal, listens for user key events,
/// and listens for input from solver.
pub struct App {
    should_exit: bool,
    state: AppState,
    old_states: BoundedStack<AppState>,
    renderer: Renderer,
    solver_enabled: bool,
    solver: Solver,
}

/// These are the associated functions to indrectly create and run an app.
impl App {
    /// If user provided help flag, simply print usage and exit.
    /// Otherwise, create an app instance and run.
    ///
    /// # Errors
    ///
    /// This function will return `MyError` if user provided invalid command line arguments,
    /// if logger could not be initialized for the TTY, or if app failed to build from save file.
    pub fn run() -> Result<(), MyError> {
        // 1. Parse settings from command line arguments.
        let settings = AppSettings::try_from_command_line()?;

        // If --help flag is set, simply print usage and end here.
        if settings.help {
            println!("{}", AppSettings::get_usage());
            return Ok(());
        }

        // 2. Initialize logger.
        logger::initialize_logger(settings.tty_path.as_ref())?;

        // 3. Build app.
        let mut app = Self::try_build(&settings)?;

        // 4. Run app until user input to exit.
        if let Err(err) = ratatui::run(|terminal| app.run_in_ratatui(terminal)) {
            log::error!("ratatui ran into IO error: {err}");
        }

        // 5. Save state to file.
        app.save_state_to_file(DEFAULT_SAVE_FILE_PATH)?;
        Ok(())
    }

    /// Either load an `AppState` from save file or create a new `AppState` with specific grid size.
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
            old_states: BoundedStack::new(DEFAULT_OLD_STATES_STACK_CAPACITY),
            renderer: Renderer::default(),
            solver_enabled: false,
            solver: Solver::default(),
        })
    }

    /// Construct an `AppState` from save file if the file exists and is valid.
    fn try_load_from_save_file(path: &str) -> Result<AppState, MyError> {
        // 1. Check if save file exists.
        match fs::exists(path) {
            Ok(true) => (),
            Ok(false) | Err(_) => {
                return Err(MyError::SaveFileNotFound {
                    save_file_path: path.to_owned(),
                });
            }
        }

        // 2. Read the file contents to construct an AppState instance.
        load_file(path, 0).map_err(|_| MyError::SaveFileFailedToLoad {
            save_file_path: path.to_owned(),
        })
    }
}

/// These are the instance methods.
impl App {
    /// With `terminal` provided by ratatui, this runs forever until user input to exit.
    /// It is not clear what kind of IO error might occur when drawing to terminal or listening for events.
    fn run_in_ratatui(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.renderer.render(frame, &self.state))?;
            self.handle_events()?;
            self.handle_solver_input();
        }
        Ok(())
    }

    /// Use crossterm to listen for user input from pressing down a key.
    /// Current implementation polls (waits) for 100 milliseconds, and then moves on if no events were polled.
    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => (),
            }
        }
        Ok(())
    }

    /// Listen to specific key input events.
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => self.restart(),
            KeyCode::Char('g') => self.toggle_grid_visibility(),
            KeyCode::Backspace => self.undo(),
            KeyCode::Char('w') | KeyCode::Up => self.update(MoveDirection::Up),
            KeyCode::Char('s') | KeyCode::Down => self.update(MoveDirection::Down),
            KeyCode::Char('a') | KeyCode::Left => self.update(MoveDirection::Left),
            KeyCode::Char('d') | KeyCode::Right => self.update(MoveDirection::Right),
            KeyCode::Char('z') => self.toggle_solver(),
            _ => (),
        }
    }

    /// Listen for input from solver.
    fn handle_solver_input(&mut self) {
        if self.solver_enabled && self.solver.is_ready() {
            let direction = self.solver.solve(&self.state);
            self.update(direction);
        }
    }

    fn save_state_to_file(&self, path: &str) -> Result<(), MyError> {
        save_file(DEFAULT_SAVE_FILE_PATH, 0, &self.state).map_err(|_| {
            MyError::SaveFileFailedToSave {
                save_file_path: path.to_owned(),
            }
        })
    }

    const fn exit(&mut self) {
        self.should_exit = true;
    }

    fn save_state_to_history(&mut self) {
        self.old_states.push(self.state.clone());
    }

    /// Revert to the previous state if possible.
    fn undo(&mut self) {
        if let Some(prev_state) = self.old_states.pop() {
            self.state = prev_state;
        }
    }

    fn restart(&mut self) {
        self.old_states.clear();
        self.state.restart();
    }

    const fn toggle_grid_visibility(&mut self) {
        self.renderer.toggle_grid_visibility();
    }

    const fn toggle_solver(&mut self) {
        self.solver_enabled = !self.solver_enabled;
    }

    fn update(&mut self, direction: MoveDirection) {
        self.save_state_to_history();
        self.state.update(direction);
    }
}
