#![allow(dead_code)]

use rand::seq::IndexedRandom;
use std::time::{Duration, Instant};

use crate::{
    app_state::AppState,
    move_direction::{ALL_DIRECTIONS, MoveDirection},
    solver::Solver,
};

pub struct DumbSolver {
    time_to_wait: Duration,
    last_solved_at: Option<Instant>,
}

impl DumbSolver {
    pub fn new() -> Self {
        const DEFAULT_TIME_TO_WAIT: Duration = Duration::from_millis(1000);

        Self {
            time_to_wait: DEFAULT_TIME_TO_WAIT,
            last_solved_at: None,
        }
    }
}

impl Solver for DumbSolver {
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

    /// Randomly pick one out of 4 possible directions, not even looking at the AppState.
    fn solve(&mut self, _: &AppState) -> MoveDirection {
        let answer = *ALL_DIRECTIONS.choose(&mut rand::rng()).unwrap();
        self.last_solved_at = Some(Instant::now());

        answer
    }
}
