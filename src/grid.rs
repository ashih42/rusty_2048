use itertools::Itertools;
use rand::seq::IndexedRandom;
use savefile::savefile_derive::Savefile;

use crate::{move_direction::MoveDirection, tile::Tile, vector2d::Vector2D};

/*
    +-------> x
    |
    |
    V
    y

    For example, in this map with num_rows=2, num_cols=4, '4' is at position x=2, y=0.

        .   .   4   .
        .  32   .  16
*/

/// Grid is responsible for operations on its Tiles.
#[derive(Clone, Debug, Savefile)]
pub struct Grid {
    pub num_rows: usize,
    pub num_cols: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    /// Create a Grid with no tiles.
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        Self {
            num_rows,
            num_cols,
            tiles: vec![Tile::new_empty(); num_rows * num_cols],
        }
    }

    /// Create a Grid from a string representation of all tiles.
    ///
    /// Currently, this is only used for unit-testing.
    #[allow(dead_code)]
    fn from_snapshot(snapshot: &str) -> Self {
        let num_rows = snapshot.lines().count();

        let first_row = snapshot.lines().next().unwrap();
        let num_cols = first_row.split_whitespace().count();

        let tiles = snapshot.split_whitespace().map(Tile::from_str).collect();

        Self {
            num_rows,
            num_cols,
            tiles,
        }
    }

    /// Set all tiles to empty.
    pub fn clear(&mut self) {
        self.tiles.fill(Tile::new_empty());
    }

    pub fn get_tile(&self, position: &Vector2D<usize>) -> &Tile {
        let index = self.get_1d_index(position);

        &self.tiles[index]
    }

    fn set_tile(&mut self, position: &Vector2D<usize>, tile: Tile) {
        let index = self.get_1d_index(position);

        self.tiles[index] = tile;
    }

    /// Swap the contents of the tiles at the 2 given positions.
    fn swap_tiles(&mut self, a_position: &Vector2D<usize>, b_position: &Vector2D<usize>) {
        let a_index = self.get_1d_index(a_position);
        let b_index = self.get_1d_index(b_position);

        self.tiles.swap(a_index, b_index);
    }

    fn get_1d_index(&self, position: &Vector2D<usize>) -> usize {
        position.x + position.y * self.num_cols
    }

    fn get_2d_position(&self, index: usize) -> Vector2D<usize> {
        let x = index % self.num_cols;
        let y = index / self.num_cols;

        Vector2D::new(x, y)
    }

    /// Return a string representation of the tiles,
    /// with a ' ' separating tiles within the same row, and
    /// with a '\n' separating tiles on different rows.
    ///
    /// Currently, this is only used for unit-testing.
    #[allow(dead_code)]
    fn generate_snapshot(&self) -> String {
        let rows: Vec<&[Tile]> = self.tiles.chunks(self.num_cols).collect();

        rows.iter()
            .map(|row| row.iter().map(|tile| tile.to_string()).join(" "))
            .join("\n")
    }

    /// Return a vector containing positions of all empty tiles.
    fn collect_empty_positions(&self) -> Vec<Vector2D<usize>> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.is_empty())
            .map(|(index, _)| self.get_2d_position(index))
            .collect()
    }

    /// Return a randomly chosen empty tile position, if possible.
    fn get_random_empty_position(&self) -> Option<Vector2D<usize>> {
        self.collect_empty_positions()
            .choose(&mut rand::rng())
            .cloned()
    }

    /// If possible, choose a random empty position and creates a random tile there,
    /// and return that position.
    ///
    /// The type of tile created follows this probability distrubition:
    /// - 70% chance to create a number, always with value = 2.
    /// - 10% chance to create a multiplier, with 1 <= power <= 3.
    /// - 10% chance to create a divider, with 1 <= power <= 3.
    /// - 10% chance to create a bomb.
    pub fn spawn_random_tile_at_random_location(&mut self) -> Option<Vector2D<usize>> {
        if let Some(position) = self.get_random_empty_position() {
            let x = rand::random_range(0..100);
            let power = rand::random_range(1..=3);

            let tile = if x < 70 {
                Tile::new_number(2)
            } else if x < 80 {
                Tile::new_multiplier(power)
            } else if x < 90 {
                Tile::new_divider(power)
            } else {
                Tile::new_bomb()
            };

            self.set_tile(&position, tile);
            Some(position)
        } else {
            None
        }
    }

    /// Check if a tile with the given value exists.
    pub fn contains_value(&self, target: u16) -> bool {
        self.tiles
            .iter()
            .any(|tile| tile.get_value() == Some(target))
    }

    /// Check if the grid is in a dead state, which would mean game over.
    ///
    /// A dead grid holds true these 2 conditions:
    /// 1. There is no empty tile.
    /// 2. There is no possible merge anywhere.
    pub fn is_dead(&self) -> bool {
        let has_empty_tiles = self.tiles.iter().any(|tile| tile.is_empty());
        let is_merge_possible = self.has_possible_merges();

        !has_empty_tiles && !is_merge_possible
    }

    /// Check if there are any 2 adjacent tiles on the grid that can be merged.
    fn has_possible_merges(&self) -> bool {
        let horizontal_positional_rows = self.generate_positional_rows(MoveDirection::Left);
        let vertical_positional_rows = self.generate_positional_rows(MoveDirection::Up);

        horizontal_positional_rows
            .into_iter()
            .chain(vertical_positional_rows)
            .into_iter()
            .any(|row| self.contains_mergeable(&row))
    }

    /// Given a direction from player input, update the grid and return a total score value
    /// resulting from all tiles merged.
    ///
    /// Example:
    ///
    /// Starting with this grid:
    /// ```ignore
    /// . 2 . 2 2 . 2 .
    /// ```
    ///
    /// After a LEFT, the grid should become:
    /// ```ignore
    /// 4 4 . . . . . .
    /// ```
    ///
    /// In this case, the score returned is (2 + 2) + (2 + 2) = 8.
    pub fn update(&mut self, direction: MoveDirection) -> i16 {
        self.generate_positional_rows(direction)
            .iter()
            .map(|row| {
                // 1. Shift all tiles as far as possible to one side,
                // and leave only empty tiles at the other side.
                self.shift(row);

                // 2. Merge adjacent tiles in the row, and obtain a score value from these merges.
                let score = self.merge(row);

                // 3. Shift again, because the last merge could have created new empty tiles
                // in the middle of the row.
                self.shift(row);

                // 4. Accumulate the score.
                score
            })
            .sum()
    }

    /// Given a `direction`, return  a vector containing positions that would be
    /// the starting position of each directional row in `generate_positional_rows`.
    fn generate_starting_positions(&self, direction: MoveDirection) -> Vec<Vector2D<usize>> {
        match direction {
            MoveDirection::Left => {
                let x = 0;
                (0..self.num_rows).map(|y| Vector2D::new(x, y)).collect()
            }
            MoveDirection::Right => {
                let x = self.num_cols - 1;
                (0..self.num_rows).map(|y| Vector2D::new(x, y)).collect()
            }
            MoveDirection::Up => {
                let y = 0;
                (0..self.num_cols).map(|x| Vector2D::new(x, y)).collect()
            }
            MoveDirection::Down => {
                let y = self.num_rows - 1;
                (0..self.num_cols).map(|x| Vector2D::new(x, y)).collect()
            }
        }
    }

    /// Given a `direction` enum, return a unit vector in 2D space representing the
    /// opposite direction to expand towards.
    fn generate_direction_offset(&self, direction: MoveDirection) -> Vector2D<i8> {
        match direction {
            MoveDirection::Left => Vector2D::new(1, 0),
            MoveDirection::Right => Vector2D::new(-1, 0),
            MoveDirection::Up => Vector2D::new(0, 1),
            MoveDirection::Down => Vector2D::new(0, -1),
        }
    }

    /// Given a `direction`, return a vector containing `positional_rows`, which are vectors containing positions.
    ///
    /// A `positional_row` is a vector containing the positions in order in a horizontal or vertical group of tiles.
    /// The ordering is most natural to understand in the case with direction = LEFT, so that
    /// the first item in the vector corresponds to the first (leftmost) item in a physical horizontal row.
    fn generate_positional_rows(&self, direction: MoveDirection) -> Vec<Vec<Vector2D<usize>>> {
        let starting_positions = self.generate_starting_positions(direction);
        let offset = self.generate_direction_offset(direction);

        starting_positions
            .iter()
            .map(|pos| self.generate_positional_row(pos, offset))
            .collect()
    }

    /// Return a vector containing all positions in a `positional_row`,
    /// given a starting position and `offset` as a direction vector to keep checking
    /// for next positions to add to this `positional_row`.
    ///
    /// This implementation converts the input usize to i8 to allow signed operations,
    /// with the reasonable assumption that the grid dimensions (`num_rows` and `num_cols` from user input)
    /// would not exceed i8::MAX.
    fn generate_positional_row(
        &self,
        starting_position: &Vector2D<usize>,
        offset: Vector2D<i8>,
    ) -> Vec<Vector2D<usize>> {
        let mut row = vec![];
        let mut pos = Vector2D::new(starting_position.x as i8, starting_position.y as i8);

        while self.is_valid_position(&pos) {
            let valid_position = Vector2D::new(pos.x as usize, pos.y as usize);
            row.push(valid_position);
            pos += offset;
        }

        row
    }

    /// Check if the given position (containing signed values) is valid.
    fn is_valid_position(&self, pos: &Vector2D<i8>) -> bool {
        let num_rows = self.num_rows as i8;
        let num_cols = self.num_cols as i8;

        0 <= pos.x && pos.x < num_cols && 0 <= pos.y && pos.y < num_rows
    }

    /// Move all tiles in this row as far as possible,
    /// by repeatedly swapping an empty tile with the next non-empty tile.
    ///
    /// Time complexity is O(n^2), where n is number of elements in `positional_row`.
    fn shift(&mut self, positional_row: &[Vector2D<usize>]) {
        for (i, current_pos) in positional_row.iter().enumerate() {
            let current_tile = self.get_tile(current_pos);

            if current_tile.is_empty() {
                let remaining_positions = &positional_row[i + 1..];

                for next_pos in remaining_positions {
                    let next_tile = self.get_tile(next_pos);

                    if !next_tile.is_empty() {
                        self.swap_tiles(current_pos, next_pos);
                        break;
                    }
                }
            }
        }
    }

    /// Merge all adjacent tiles in this row wherever possible, and
    /// return the total score from these merges.
    fn merge(&mut self, positional_row: &[Vector2D<usize>]) -> i16 {
        positional_row
            .windows(2)
            .map(|positions| {
                let a_pos = &positions[0];
                let b_pos = &positions[1];

                let a_index = self.get_1d_index(a_pos);
                let b_index = self.get_1d_index(b_pos);

                let [a_tile, b_tile] = self.tiles.get_disjoint_mut([a_index, b_index]).unwrap();
                Tile::merge_tiles(a_tile, b_tile)
            })
            .sum()
    }

    /// Check if the given row contains any mergeable adjacent tiles.
    fn contains_mergeable(&self, positional_row: &[Vector2D<usize>]) -> bool {
        for positions in positional_row.windows(2) {
            let a_pos = &positions[0];
            let b_pos = &positions[1];

            let a_tile = self.get_tile(a_pos);
            let b_tile = self.get_tile(b_pos);

            if Tile::are_mergeable(a_tile, b_tile) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_grid(grid: &Grid) {
        println!("num_rows: {}", grid.num_rows);
        println!("num_cols: {}", grid.num_cols);
        println!("snapshot:\n{}\n", grid.generate_snapshot());
    }

    #[test]
    fn test_print_grid() {
        let (num_rows, num_cols) = (2, 3);
        let mut grid = Grid::new(num_rows, num_cols);

        print_grid(&grid);

        let empty_positions = grid.collect_empty_positions();
        println!("\nempty_positions: {:?}", empty_positions);

        let empty_position = grid.get_random_empty_position();
        println!("\nempty_position: {:?}", empty_position);
        println!();

        for i in 0..10 {
            println!("i = {i}");
            grid.spawn_random_tile_at_random_location();
            print_grid(&grid);
        }
    }

    #[test]
    fn test_from_snapshot() {
        let snapshot = "\
            .   .   4   .
            .  32   .  16";
        let grid = Grid::from_snapshot(snapshot);

        assert_eq!(grid.num_rows, 2);
        assert_eq!(grid.num_cols, 4);
        assert_eq!(grid.generate_snapshot(), ". . 4 .\n. 32 . 16");

        print_grid(&grid);

        let empty_positions = grid.collect_empty_positions();
        println!("\nempty_positions: {:?}", empty_positions);
    }

    #[test]
    fn test_exactly_one_empty_position() {
        let snapshot = "\
            8   2   4   .
            2  32   2  16";
        let mut grid = Grid::from_snapshot(snapshot);

        assert_eq!(grid.get_random_empty_position(), Some(Vector2D::new(3, 0)));

        grid.spawn_random_tile_at_random_location();
        assert_eq!(grid.get_random_empty_position(), None);
    }

    #[test]
    fn test_generate_starting_positions() {
        // (0, 0), (1, 0), (2, 0), (3, 0)
        // (0, 1), (1, 1), (2, 1), (3, 1)
        let grid = Grid::new(2, 4);

        assert_eq!(
            grid.generate_starting_positions(MoveDirection::Left),
            vec![Vector2D::new(0, 0), Vector2D::new(0, 1)]
        );

        assert_eq!(
            grid.generate_starting_positions(MoveDirection::Right),
            vec![Vector2D::new(3, 0), Vector2D::new(3, 1)]
        );

        assert_eq!(
            grid.generate_starting_positions(MoveDirection::Up),
            vec![
                Vector2D::new(0, 0),
                Vector2D::new(1, 0),
                Vector2D::new(2, 0),
                Vector2D::new(3, 0)
            ]
        );

        assert_eq!(
            grid.generate_starting_positions(MoveDirection::Down),
            vec![
                Vector2D::new(0, 1),
                Vector2D::new(1, 1),
                Vector2D::new(2, 1),
                Vector2D::new(3, 1)
            ]
        );
    }

    #[test]
    fn test_generate_positional_rows() {
        // (0, 0), (1, 0), (2, 0), (3, 0)
        // (0, 1), (1, 1), (2, 1), (3, 1)
        let grid = Grid::new(2, 4);

        assert_eq!(
            grid.generate_positional_rows(MoveDirection::Left),
            vec![
                vec![
                    Vector2D::new(0, 0),
                    Vector2D::new(1, 0),
                    Vector2D::new(2, 0),
                    Vector2D::new(3, 0),
                ],
                vec![
                    Vector2D::new(0, 1),
                    Vector2D::new(1, 1),
                    Vector2D::new(2, 1),
                    Vector2D::new(3, 1),
                ]
            ]
        );

        assert_eq!(
            grid.generate_positional_rows(MoveDirection::Right),
            vec![
                vec![
                    Vector2D::new(3, 0),
                    Vector2D::new(2, 0),
                    Vector2D::new(1, 0),
                    Vector2D::new(0, 0),
                ],
                vec![
                    Vector2D::new(3, 1),
                    Vector2D::new(2, 1),
                    Vector2D::new(1, 1),
                    Vector2D::new(0, 1),
                ]
            ]
        );

        assert_eq!(
            grid.generate_positional_rows(MoveDirection::Up),
            vec![
                vec![Vector2D::new(0, 0), Vector2D::new(0, 1)],
                vec![Vector2D::new(1, 0), Vector2D::new(1, 1)],
                vec![Vector2D::new(2, 0), Vector2D::new(2, 1)],
                vec![Vector2D::new(3, 0), Vector2D::new(3, 1)],
            ]
        );

        assert_eq!(
            grid.generate_positional_rows(MoveDirection::Down),
            vec![
                vec![Vector2D::new(0, 1), Vector2D::new(0, 0)],
                vec![Vector2D::new(1, 1), Vector2D::new(1, 0)],
                vec![Vector2D::new(2, 1), Vector2D::new(2, 0)],
                vec![Vector2D::new(3, 1), Vector2D::new(3, 0)],
            ]
        );
    }
}
