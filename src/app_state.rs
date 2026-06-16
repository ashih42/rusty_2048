use savefile::savefile_derive::Savefile;

use crate::{game_state::GameState, grid::Grid, move_direction::MoveDirection, vector2d::Vector2D};

/// `AppState` manages the data that make up the business logic of this game.
#[derive(Clone, Debug, Savefile)]
pub struct AppState {
    pub game_state: GameState,
    pub current_turn: u16,
    pub current_score: u16,
    pub best_score: u16,
    pub winning_target: u16,
    pub grid: Grid,
    pub new_tile_positions: Vec<Vector2D<usize>>,
}

impl AppState {
    const DEFAULT_WINNING_TARGET: u16 = 2048;

    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        let mut app = Self {
            game_state: GameState::InPlay,
            current_turn: 1,
            current_score: 0,
            best_score: 0,
            winning_target: Self::DEFAULT_WINNING_TARGET,
            grid: Grid::new(num_rows, num_cols),
            new_tile_positions: Vec::new(),
        };

        app.spawn_tiles();
        app
    }

    /// This is used for unit testing.
    #[allow(dead_code)]
    pub const fn with_grid(grid: Grid) -> Self {
        Self {
            game_state: GameState::InPlay,
            current_turn: 1,
            current_score: 0,
            best_score: 0,
            winning_target: Self::DEFAULT_WINNING_TARGET,
            grid,
            new_tile_positions: Vec::new(),
        }
    }

    /// Spawn anywhere between 1 - 3 new tiles if possible.
    fn spawn_tiles(&mut self) {
        let num_spawns = rand::random_range(1..=3);

        for _ in 0..num_spawns {
            if let Some(position) = self.grid.spawn_random_tile_at_random_location() {
                self.new_tile_positions.push(position);
            }
        }
    }

    /// Reset to an initial state for a new game.
    pub fn restart(&mut self) {
        self.game_state = GameState::InPlay;
        self.current_turn = 1;
        self.current_score = 0;

        self.grid.clear();
        self.new_tile_positions.clear();
        self.spawn_tiles();
    }

    /// This is the big logic update function, called after any gameplay event
    /// that may affect the game state.
    pub fn update(&mut self, direction: MoveDirection) {
        log::info!("app.tick(), turn {}", self.current_turn);

        if matches!(self.game_state, GameState::Won | GameState::Lost) {
            return;
        }

        let score = self.grid.update(direction);
        self.update_scores(score);
        self.current_turn += 1;
        self.new_tile_positions.clear();
        self.check_if_won();

        self.spawn_tiles();
        self.check_if_lost();
    }

    /// Update both `current_score` and `best_score`.
    fn update_scores(&mut self, score: i16) {
        let updated_score = (self.current_score as i16) + score;
        let updated_score = updated_score.clamp(0, i16::MAX);
        self.current_score = updated_score as u16;

        if self.current_score > self.best_score {
            self.best_score = self.current_score;
        }
    }

    fn check_if_won(&mut self) {
        if self.grid.contains_value(self.winning_target) {
            self.game_state = GameState::Won;
        }
    }

    fn check_if_lost(&mut self) {
        if self.grid.is_dead() {
            self.game_state = GameState::Lost;
        }
    }
}
