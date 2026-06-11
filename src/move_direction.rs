#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub const ALL_DIRECTIONS: [MoveDirection; 4] = [
    MoveDirection::Left,
    MoveDirection::Right,
    MoveDirection::Up,
    MoveDirection::Down,
];
