use rand::seq::IndexedRandom;
use std::time::{Duration, Instant};

use crate::{
    app_state::AppState,
    move_direction::{ALL_DIRECTIONS, MoveDirection},
};

/// Solver is responsible for choosing a MoveDirection, given an AppState.
/// Also, it maintains a timer so that it doesn't answer again too quickly.
///
/// This implements the Strategy pattern, although currently `solve_fn`
/// does not need to change at runtime.
pub struct Solver {
    solve_fn: fn(&AppState) -> MoveDirection,
    cooldown: Duration,
    last_solved_at: Option<Instant>,
}

impl Default for Solver {
    fn default() -> Self {
        const DEFAULT_COOLDOWN: Duration = Duration::from_millis(1000);

        Self {
            solve_fn: solve_by_greedy,
            cooldown: DEFAULT_COOLDOWN,
            last_solved_at: None,
        }
    }
}

impl Solver {
    /// Wait for a fixed amount of time since last solve.
    pub fn is_ready(&self) -> bool {
        match self.last_solved_at {
            None => true,
            Some(last_solved_at) => {
                let time_since_last_solve = Instant::now() - last_solved_at;
                time_since_last_solve > self.cooldown
            }
        }
    }

    /// Use the assigned `solve_fn` to find the best MoveDirection,
    /// update the internal timer, and return the answer.
    pub fn solve(&mut self, state: &AppState) -> MoveDirection {
        let answer = (self.solve_fn)(state);
        self.last_solved_at = Some(Instant::now());

        answer
    }
}

#[allow(dead_code)]
/// Randomly pick one out of 4 possible directions, not even looking at the AppState.
fn solve_by_random(_: &AppState) -> MoveDirection {
    ALL_DIRECTIONS.choose(&mut rand::rng()).copied().unwrap()
}

/// Try updating a new grid with each each direction,
/// and select the direction that produced the highest score.
fn solve_by_greedy(state: &AppState) -> MoveDirection {
    let (best_direction, _) = ALL_DIRECTIONS
        .iter()
        .map(|direction| {
            let mut grid = state.grid.clone();
            let score = grid.update(*direction);
            (direction, score)
        })
        .max_by(|(_, a_score), (_, b_score)| a_score.cmp(b_score))
        .unwrap();

    *best_direction
}
