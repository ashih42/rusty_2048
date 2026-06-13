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

/// Try each direction, and only look at the resulting score, and if the direction won the game.
fn solve_by_greedy(state: &AppState) -> MoveDirection {
    // Try each direction, and record the (direction, score, won) data for each direction.
    let trials: Vec<(MoveDirection, i16, bool)> = ALL_DIRECTIONS
        .iter()
        .map(|direction| {
            let mut grid = state.grid.clone();
            let score = grid.update(*direction);
            let won = grid.contains_value(state.winning_target);
            (*direction, score, won)
        })
        .collect();

    // If there are any directions that won the game, select one among them that produced the highest score.
    if let Some(winner) = trials
        .iter()
        .filter(|(_, _, won)| *won)
        .max_by(|(_, a_score, _), (_, b_score, _)| a_score.cmp(b_score))
    {
        return winner.0;
    }

    // Otherwise, select one among all directions that produced the highest score.
    let winner = trials
        .iter()
        .max_by(|(_, a_score, _), (_, b_score, _)| a_score.cmp(b_score))
        .unwrap();

    winner.0
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    use super::*;

    #[test]
    fn test_solve_by_greedy() {
        // No winner. Simply pick the direction with highest score.
        {
            let snapshot = "\
             .   .    B   .
            *2   .    2  *8
             .   .   *4   .";
            let grid = Grid::from_snapshot(snapshot);
            let state = AppState::with_grid(grid);

            assert_eq!(solve_by_greedy(&state), MoveDirection::Right);
        }

        // Pick the direction that won but produced a lower score than a non-winner.
        {
            let snapshot = "\
             .   .      B   .
            *2   .    512  *8
            .    .     *4   .";
            let grid = Grid::from_snapshot(snapshot);
            let state = AppState::with_grid(grid);

            assert_eq!(solve_by_greedy(&state), MoveDirection::Down);
        }

        // Among 2 winners, pick the direction with the highest score.
        {
            let snapshot = "\
             2   .     *4   .
            *2   .    512  *8
             B   .     *4   .";
            let grid = Grid::from_snapshot(snapshot);
            let state = AppState::with_grid(grid);

            assert_eq!(solve_by_greedy(&state), MoveDirection::Up);
        }
    }
}
