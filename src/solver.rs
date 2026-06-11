use crate::{app_state::AppState, move_direction::MoveDirection};

/// Solver is responsible for choosing a MoveDirection, given the current AppState.
pub trait Solver {
    /// Check if Solver is ready to give an answer.
    fn is_ready(&self) -> bool;

    /// Produce an answer.
    fn solve(&mut self, state: &AppState) -> MoveDirection;
}
