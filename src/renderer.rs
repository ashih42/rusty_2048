use elsa::FrozenMap;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{app_state::AppState, game_state::GameState, tile::Tile, vector2d::Vector2D};

#[derive(Default)]
pub struct Renderer {
    should_show_grid: bool,
    tile_string_cache: FrozenMap<Tile, Box<str>>,
}

impl Renderer {
    pub fn toggle_grid(&mut self) {
        self.should_show_grid = !self.should_show_grid;
    }

    pub fn render(&self, frame: &mut Frame, state: &AppState) {
        // 1. Split screen into a top banner and a bottom grid area.
        let (total_area, banner_area, grid_area) = self.split_area(frame);

        // 2. Render the top banner.
        self.render_banner(frame, banner_area, state);

        // 3. Render the grid inside the remaining bottom area.
        self.render_grid(frame, grid_area, state);

        // 4. Render the game-over popup box overlay.
        self.render_game_over_popup(frame, total_area, state);
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

    fn render_banner(&self, frame: &mut Frame, banner_area: Rect, state: &AppState) {
        let banner_text = format!(
            " Turn: {}    |    Score: {}    |    High Score: {} ",
            state.current_turn, state.current_score, state.best_score,
        );

        let banner_widget = Paragraph::new(banner_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(" Rusty 2048 "),
            )
            .style(Style::default().fg(Color::Yellow).bold());
        frame.render_widget(banner_widget, banner_area);
    }

    fn render_grid(&self, frame: &mut Frame, grid_area: Rect, state: &AppState) {
        let (num_rows, num_cols) = (state.grid.num_rows, state.grid.num_cols);

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
                let tile = state.grid.get_tile(&position);
                let tile_str = self.get_tile_str(tile);
                let border_color = self.get_tile_color(tile);

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
                    if state.new_tile_positions.contains(&position) {
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

    /// Return a color that appropriately represents the tile.
    /// Reference: https://ratatui.rs/examples/style/colors/
    fn get_tile_color(&self, tile: &Tile) -> Color {
        match tile {
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
        }
    }

    /// Get a cached string representation for the tile.
    /// If not found, update the cache, and then return the cached string.
    fn get_tile_str(&self, tile: &Tile) -> &str {
        if let Some(s) = self.tile_string_cache.get(tile) {
            return s;
        }

        self.tile_string_cache
            .insert(*tile, tile.get_fancy_string().into())
    }

    fn render_game_over_popup(&self, frame: &mut Frame, total_area: Rect, state: &AppState) {
        if matches!(state.game_state, GameState::Won | GameState::Lost) {
            // 4. Render the centered "YOU WIN" popup box overlay
            // Set size parameters for the popup box (30 columns wide, 6 rows high)
            let popup_area = self.centered_rect(30, 6, total_area);

            // Clear widget removes any underlying characters from the grid underneath
            frame.render_widget(Clear, popup_area);

            let border_color = if state.game_state == GameState::Won {
                Color::Green
            } else {
                Color::Red
            };

            let message_text = if state.game_state == GameState::Won {
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

    /// Helper function to build a centered bounding box geometry overlay
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
