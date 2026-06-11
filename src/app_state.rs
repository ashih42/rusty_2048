use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};
use savefile::savefile_derive::Savefile;

use crate::{grid::Grid, move_direction::MoveDirection, tile::Tile, vector2d::Vector2D};

#[derive(Clone, Debug, PartialEq, Savefile)]
enum GameState {
    InPlay,
    Won,
    Lost,
}

/// AppState manages the data that make up the business logic of this game.
#[derive(Clone, Debug, Savefile)]
pub struct AppState {
    should_show_grid: bool,
    should_exit: bool,
    game_state: GameState,
    current_turn: u16,
    current_score: u16,
    best_score: u16,
    winning_target: u16,
    grid: Grid,
    new_tile_positions: Vec<Vector2D<usize>>,
}

impl AppState {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        let mut app = Self {
            should_show_grid: true,
            should_exit: false,
            game_state: GameState::InPlay,
            current_turn: 1,
            current_score: 0,
            best_score: 0,
            winning_target: 2048,
            grid: Grid::new(num_rows, num_cols),
            new_tile_positions: Vec::new(),
        };

        app.spawn_tile();
        app
    }

    fn spawn_tile(&mut self) {
        if let Some(position) = self.grid.spawn_random_tile_at_random_location() {
            self.new_tile_positions.push(position);
        }
    }

    pub fn toggle_grid(&mut self) {
        self.should_show_grid = !self.should_show_grid;
    }

    /// Reset to an initial state for a new game.
    pub fn restart(&mut self) {
        self.game_state = GameState::InPlay;
        self.current_turn = 1;
        self.current_score = 0;

        self.grid.clear();
        self.new_tile_positions.clear();
        self.grid.spawn_random_tile_at_random_location();
    }

    /// This is the big logic update function, called after any gameplay event
    /// that may affect the game state.
    pub fn update(&mut self, direction: MoveDirection) {
        log::info!("app.tick(), turn {}", self.current_turn);

        if matches!(self.game_state, GameState::Won | GameState::Lost) {
            return;
        }

        let score = self.grid.handle_move(direction);
        self.update_scores(score);
        self.current_turn += 1;

        self.new_tile_positions.clear();

        if rand::random() {
            self.spawn_tile();
        }

        self.check_if_won();
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

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }
}

/// These operations use ratatui to update the view on the terminal.
impl AppState {
    pub fn render(&self, frame: &mut Frame) {
        // 1. Split screen into a top banner and a bottom grid area.
        let (total_area, banner_area, grid_area) = self.split_area(frame);

        // 2. Render the top banner.
        self.render_banner(frame, banner_area);

        // 3. Render the grid inside the remaining bottom area.
        self.render_grid(frame, grid_area);

        // 4. Render the game-over popup box overlay.
        self.render_game_over_popup(frame, total_area);
    }

    fn split_area(&self, frame: &Frame) -> (Rect, Rect, Rect) {
        let total_area = frame.area();

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),       // Fixed height banner
                Constraint::Percentage(100), // Rest of the screen for the grid
            ])
            .split(total_area);

        let banner_area = main_chunks[0];
        let grid_area = main_chunks[1];

        (total_area, banner_area, grid_area)
    }

    fn render_banner(&self, frame: &mut Frame, banner_area: Rect) {
        let banner_text = format!(
            " Turn: {}    |    Score: {}    |    High Score: {} ",
            self.current_turn, self.current_score, self.best_score,
        );

        let banner_widget = Paragraph::new(banner_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(" 2048 TUI "),
            )
            .style(Style::default().fg(Color::Yellow).bold());
        frame.render_widget(banner_widget, banner_area);
    }

    fn render_grid(&self, frame: &mut Frame, grid_area: Rect) {
        let (num_rows, num_cols) = (self.grid.num_rows, self.grid.num_cols);

        let row_constraints = (0..num_rows).map(|_| Constraint::Ratio(1, num_rows as u32));
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(grid_area);

        for row_idx in 0..num_rows {
            let col_constraints = (0..num_cols).map(|_| Constraint::Ratio(1, num_cols as u32));
            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(col_constraints)
                .split(rows[row_idx]);

            for col_idx in 0..num_cols {
                let position = Vector2D::new(col_idx, row_idx);
                let tile = self.grid.get_tile(&position);
                let tile_str = tile.get_str();

                // Reference: https://ratatui.rs/examples/style/colors/
                let border_color = match tile {
                    Tile::Empty => Color::Gray,
                    Tile::Multiplier(_) => Color::Green,
                    Tile::Divider(_) => Color::Red,
                    Tile::Bomb => Color::Red,
                    Tile::Number(1) => Color::Indexed(8),
                    Tile::Number(2) => Color::Indexed(3),
                    Tile::Number(4) => Color::Indexed(4),
                    Tile::Number(8) => Color::Indexed(5),
                    Tile::Number(16) => Color::Indexed(6),
                    Tile::Number(32) => Color::Indexed(7),
                    Tile::Number(64) => Color::Indexed(9),
                    Tile::Number(128) => Color::Indexed(10),
                    Tile::Number(256) => Color::Indexed(11),
                    Tile::Number(512) => Color::Indexed(12),
                    Tile::Number(1024) => Color::Indexed(13),
                    Tile::Number(2048) => Color::Indexed(14),
                    Tile::Number(_) => Color::White,
                };

                let tile_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(border_color));

                let tile_area = tile_block.inner(columns[col_idx]);
                let text_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Min(1)])
                    .split(tile_area);

                let text_widget = Paragraph::new(tile_str)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::White).bold());

                let new_tile_text_widget = Paragraph::new("NEW")
                    .alignment(Alignment::Left)
                    .style(Style::default().fg(Color::White).bold());

                // Render text for all non-empty tiles.
                if !tile.is_empty() {
                    frame.render_widget(text_widget, text_layout[1]);

                    // Render text indicating this tile just spawned on this turn.
                    if self.new_tile_positions.contains(&position) {
                        frame.render_widget(new_tile_text_widget, text_layout[0]);
                    }
                }

                // Render borders for all non-empty tiles, and
                // render borders for empty tiles only if a grid visibility flag is set.
                if self.should_show_grid || !tile.is_empty() {
                    frame.render_widget(tile_block, columns[col_idx]);
                }
            }
        }
    }

    fn render_game_over_popup(&self, frame: &mut Frame, total_area: Rect) {
        if matches!(self.game_state, GameState::Won | GameState::Lost) {
            // 4. Render the centered "YOU WIN" popup box overlay
            // Set size parameters for the popup box (30 columns wide, 6 rows high)
            let popup_area = self.centered_rect(30, 6, total_area);

            // Clear widget removes any underlying characters from the grid underneath
            frame.render_widget(Clear, popup_area);

            let border_color = if self.game_state == GameState::Won {
                Color::Green
            } else {
                Color::Red
            };

            let message_text = if self.game_state == GameState::Won {
                "YOU WIN"
            } else {
                "YOU LOSE"
            };

            // Build the victory notification dialog box
            let popup_block = Block::default()
                .title(" GAME OVER ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color).bold());

            // Center the message text vertically within the box area
            let popup_area = popup_block.inner(popup_area);
            let popup_text_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Min(1)])
                .split(popup_area);

            let popup_message = Paragraph::new(message_text)
                .alignment(Alignment::Center)
                .style(Style::default().fg(border_color).bold());

            frame.render_widget(popup_block, popup_area);
            frame.render_widget(popup_message, popup_text_layout[1]);
        }
    }

    // Helper function to build a centered bounding box geometry overlay
    fn centered_rect(&self, width: u16, height: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length((r.height.saturating_sub(height)) / 2),
                Constraint::Length(height),
                Constraint::Min(0),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((r.width.saturating_sub(width)) / 2),
                Constraint::Length(width),
                Constraint::Min(0),
            ])
            .split(popup_layout[1])[1]
    }
}
