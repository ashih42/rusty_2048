use std::time::{Duration, Instant};

use crate::{
    app_state::AppState,
    move_direction::{ALL_DIRECTIONS, MoveDirection},
    solver::Solver,
};

pub struct GreedySolver {
    time_to_wait: Duration,
    last_solved_at: Option<Instant>,
}

impl GreedySolver {
    pub fn new() -> Self {
        const DEFAULT_TIME_TO_WAIT: Duration = Duration::from_millis(1000);

        Self {
            time_to_wait: DEFAULT_TIME_TO_WAIT,
            last_solved_at: None,
        }
    }
}

impl Solver for GreedySolver {
    /// Wait for a fixed amount of time since last solve.
    fn is_ready(&self) -> bool {
        match self.last_solved_at {
            None => true,
            Some(last_solved_at) => {
                let time_since_last_solve = Instant::now() - last_solved_at;
                time_since_last_solve > self.time_to_wait
            }
        }
    }

    /// For each direction, simulate applying this direction on a new
    /// and select the direction that yielded the highest score.
    fn solve(&mut self, state: &AppState) -> MoveDirection {
        let (best_direction, _) = ALL_DIRECTIONS
            .iter()
            .map(|direction| {
                let mut grid = state.get_grid().clone();
                let score = grid.handle_move(*direction);
                (direction, score)
            })
            .max_by(|(_, a_score), (_, b_score)| a_score.cmp(b_score))
            .unwrap();

        self.last_solved_at = Some(Instant::now());
        *best_direction
    }
}
