use savefile::savefile_derive::Savefile;
use std::fmt::Display;

/// Note: Multiplier and Divider variants store the base 2 power (aka exponent)
/// of the scalar multiplier value.
///
/// Example: `Multiplier(3)` means multiply by 2^3, and it is displayed as `*8`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Savefile)]
pub enum Tile {
    Empty,
    Number(u16),
    Multiplier(u8),
    Divider(u8),
    Bomb,
}

impl Tile {
    pub fn new_empty() -> Self {
        Self::Empty
    }

    /// If `value` is 0, this constructor creates an empty tile instead.
    pub fn new_number(value: u16) -> Self {
        if value == 0 {
            Self::Empty
        } else {
            Self::Number(value)
        }
    }

    pub fn new_multiplier(power: u8) -> Self {
        Self::Multiplier(power)
    }

    pub fn new_divider(power: u8) -> Self {
        Self::Divider(power)
    }

    pub fn new_bomb() -> Self {
        Self::Bomb
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn get_value(&self) -> Option<u16> {
        match *self {
            Self::Number(value) => Some(value),
            _ => None,
        }
    }
}

/// These operations are related to defining string representations for the tile.
impl Tile {
    /// Return a fancy string representation of the tile, which may include
    /// emojis and other unicode characters.
    pub fn get_fancy_string(&self) -> String {
        match self {
            Self::Empty => String::from(""),
            Self::Number(value) => value
                .to_string()
                .chars()
                .map(|ch| match ch {
                    '0' => "0️⃣",
                    '1' => "1️⃣",
                    '2' => "2️⃣",
                    '3' => "3️⃣",
                    '4' => "4️⃣",
                    '5' => "5️⃣",
                    '6' => "6️⃣",
                    '7' => "7️⃣",
                    '8' => "8️⃣",
                    '9' => "9️⃣",
                    _ => unreachable!(),
                })
                .collect(),
            Self::Multiplier(power) => format!("× {}", 2 << (power - 1)),
            Self::Divider(power) => format!("÷ {}", 2 << (power - 1)),
            Self::Bomb => String::from("💣"),
        }
    }
}

impl From<&str> for Tile {
    /// Construct a tile from a string containing only ascii characters.
    /// This is only used for unit testing, so it is okay to assume input string is valid and unwrap the result.
    fn from(s: &str) -> Self {
        match s {
            "." => Self::new_empty(),
            "B" => Self::new_bomb(),
            _ if s.starts_with("*") => {
                let scalar = s[1..].parse::<u16>().unwrap();
                let power = scalar.ilog2() as u8;
                Self::new_multiplier(power)
            }
            _ if s.starts_with("/") => {
                let scalar = s[1..].parse::<u16>().unwrap();
                let power = scalar.ilog2() as u8;
                Self::new_divider(power)
            }
            _ => {
                let value = s.parse::<u16>().unwrap();
                Self::new_number(value)
            }
        }
    }
}

impl Display for Tile {
    /// Represent the tile with a simple string with only ascii characters.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Number(value) => write!(f, "{}", value),
            Self::Multiplier(power) => write!(f, "*{}", 2 << (power - 1)),
            Self::Divider(power) => write!(f, "/{}", 2 << (power - 1)),
            Self::Bomb => write!(f, "B"),
        }
    }
}

/// These operations are for resolving the merging of 2 adjacent tiles.
impl Tile {
    /// Checks if the given 2 tiles can be merged.
    pub fn are_mergeable(a: &Self, b: &Self) -> bool {
        use Tile::{Bomb, Divider, Empty, Multiplier, Number};

        match (a, b) {
            // No merge if either tile is empty.
            (Empty, _) | (_, Empty) => false,

            // Always merge a bomb and a non-empty tile.
            (Bomb, _) | (_, Bomb) => true,

            // Merge only if both numbers have the same value.
            (Number(a_value), Number(b_value)) => a_value == b_value,

            // Perform multiplication and merge.
            (Number(_), Multiplier(_)) | (Multiplier(_), Number(_)) => true,

            // Perform division and merge.
            (Number(_), Divider(_)) | (Divider(_), Number(_)) => true,

            // Merge multipliers.
            (Multiplier(_), Multiplier(_)) => true,

            // Merge multiplier and divider.
            (Multiplier(_), Divider(_)) | (Divider(_), Multiplier(_)) => true,

            // Merge dividers.
            (Divider(_), Divider(_)) => true,
        }
    }

    /// If a merge is possible, perform the merge operation,
    /// set `a` to the result, set `b` to empty, and
    /// return a score from this merge.
    pub fn merge_tiles(a: &mut Self, b: &mut Self) -> i16 {
        use Tile::{Bomb, Divider, Empty, Multiplier, Number};

        match (*a, *b) {
            // No merge if either tile is empty.
            (Empty, _) | (_, Empty) => 0,

            // Always merge a bomb and a non-empty tile.
            (Bomb, other) | (other, Bomb) => {
                let score = Self::calculate_bomb_score(&other);
                *a = Self::new_empty();
                *b = Self::new_empty();
                score
            }

            // Merge only if both numbers have the same value.
            (Number(a_value), Number(b_value)) => {
                if a_value == b_value {
                    let (sum, score) = Self::calculate_sum_and_score(a_value, b_value);
                    *a = Self::new_number(sum);
                    *b = Self::new_empty();
                    score
                } else {
                    0
                }
            }

            // Perform multiplication and merge.
            (Number(value), Multiplier(power)) | (Multiplier(power), Number(value)) => {
                let (product, score) = Self::calculate_product_and_score(value, power);
                *a = Self::new_number(product);
                *b = Self::new_empty();
                score
            }

            // Perform division and merge.
            (Number(value), Divider(power)) | (Divider(power), Number(value)) => {
                let (quotient, score) = Self::calculate_quotient_and_score(value, power);
                *a = Self::new_number(quotient);
                *b = Self::new_empty();
                score
            }

            // Merge multipliers.
            (Multiplier(a_power), Multiplier(b_power)) => {
                *a = Self::new_multiplier(a_power + b_power);
                *b = Self::new_empty();
                0
            }

            // Merge multiplier and divider.
            (Multiplier(mult_power), Divider(div_power))
            | (Divider(div_power), Multiplier(mult_power)) => {
                *a = Self::merge_multiplier_and_divider(mult_power, div_power);
                *b = Self::new_empty();
                0
            }

            // Merge dividers.
            (Divider(a_power), Divider(b_power)) => {
                *a = Self::new_divider(a_power + b_power);
                *b = Self::new_empty();
                0
            }
        }
    }

    /// Calculate the result of a sum operation, and return a score,
    /// which is always positive.
    fn calculate_sum_and_score(a_value: u16, b_value: u16) -> (u16, i16) {
        let sum = a_value + b_value;
        let score = sum as i16;

        (sum, score)
    }

    /// Calculates the result of a multiplication operation, and return a score,
    /// which is always positive.
    fn calculate_product_and_score(value: u16, power: u8) -> (u16, i16) {
        let product = value << power;
        let score = product as i16;

        (product, score)
    }

    /// Calculates the result of a division operation, and return a score
    /// as a penalty value, which is always negative or 0 at best.
    fn calculate_quotient_and_score(value: u16, power: u8) -> (u16, i16) {
        let quotient = value >> power;
        let score = -(quotient as i16);

        (quotient, score)
    }

    /// Calculate the score from a bomb-merging interaction.
    /// If `other` is a number, the bomb deletes the number, and the number value is deducted as a penalty.
    /// Otherwise, the bomb deletes something else, and the score is 0.
    fn calculate_bomb_score(other: &Tile) -> i16 {
        match other {
            Self::Number(value) => -(*value as i16),
            _ => 0,
        }
    }

    /// Return a new tile from merging a multiplier and a divider tile.
    /// This result tile may be either a multiplier, divider, or empty.
    fn merge_multiplier_and_divider(multiplier_power: u8, divider_power: u8) -> Self {
        if multiplier_power > divider_power {
            let resultant_power = multiplier_power - divider_power;
            Self::new_multiplier(resultant_power)
        } else if multiplier_power < divider_power {
            let resultant_power = divider_power - multiplier_power;
            Self::new_divider(resultant_power)
        } else {
            Self::new_empty()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Tile::from("."), Tile::new_empty());
        assert_eq!(Tile::from("64"), Tile::new_number(64));
        assert_eq!(Tile::from("*8"), Tile::new_multiplier(3));
        assert_eq!(Tile::from("/2"), Tile::new_divider(1));
        assert_eq!(Tile::from("B"), Tile::new_bomb());
    }

    #[test]
    fn test_are_mergeable() {
        let empty = Tile::new_empty();
        let number2 = Tile::new_number(2);
        let number4 = Tile::new_number(4);
        let multiplier = Tile::new_multiplier(1);
        let divider = Tile::new_divider(1);
        let bomb = Tile::new_bomb();

        assert!(!Tile::are_mergeable(&empty, &empty));
        assert!(!Tile::are_mergeable(&empty, &number2));
        assert!(!Tile::are_mergeable(&empty, &multiplier));
        assert!(!Tile::are_mergeable(&empty, &divider));
        assert!(!Tile::are_mergeable(&empty, &bomb));

        assert!(!Tile::are_mergeable(&number2, &empty));
        assert!(Tile::are_mergeable(&number2, &number2));
        assert!(!Tile::are_mergeable(&number2, &number4));
        assert!(Tile::are_mergeable(&number2, &multiplier));
        assert!(Tile::are_mergeable(&number2, &divider));
        assert!(Tile::are_mergeable(&number2, &bomb));

        assert!(!Tile::are_mergeable(&multiplier, &empty));
        assert!(Tile::are_mergeable(&multiplier, &number2));
        assert!(Tile::are_mergeable(&multiplier, &multiplier));
        assert!(Tile::are_mergeable(&multiplier, &divider));
        assert!(Tile::are_mergeable(&multiplier, &bomb));

        assert!(!Tile::are_mergeable(&divider, &empty));
        assert!(Tile::are_mergeable(&divider, &number2));
        assert!(Tile::are_mergeable(&divider, &multiplier));
        assert!(Tile::are_mergeable(&divider, &divider));
        assert!(Tile::are_mergeable(&divider, &bomb));

        assert!(!Tile::are_mergeable(&bomb, &empty));
        assert!(Tile::are_mergeable(&bomb, &number2));
        assert!(Tile::are_mergeable(&bomb, &multiplier));
        assert!(Tile::are_mergeable(&bomb, &divider));
        assert!(Tile::are_mergeable(&bomb, &bomb));
    }

    #[test]
    fn test_merge_tiles() {
        // Empty + Empty
        {
            let mut a = Tile::new_empty();
            let mut b = Tile::new_empty();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Empty + Number
        {
            let mut a = Tile::new_empty();
            let mut b = Tile::new_number(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_number(2));
            assert_eq!(score, 0);
        }

        // Empty + Multiplier
        {
            let mut a = Tile::new_empty();
            let mut b = Tile::new_multiplier(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_multiplier(1));
            assert_eq!(score, 0);
        }

        // Empty + Divider
        {
            let mut a = Tile::new_empty();
            let mut b = Tile::new_divider(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_divider(1));
            assert_eq!(score, 0);
        }

        // Empty + Bomb
        {
            let mut a = Tile::new_empty();
            let mut b = Tile::new_bomb();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_bomb());
            assert_eq!(score, 0);
        }

        // Number + Empty
        {
            let mut a = Tile::new_number(2);
            let mut b = Tile::new_empty();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_number(2));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Number(2) + Number(2) => Number(4)
        {
            let mut a = Tile::new_number(2);
            let mut b = Tile::new_number(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_number(4));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 4);
        }

        // Number(2) + Number(4) => Does Not Merge
        {
            let mut a = Tile::new_number(2);
            let mut b = Tile::new_number(4);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_number(2));
            assert_eq!(b, Tile::new_number(4));
            assert_eq!(score, 0);
        }

        // Number + Multiplier
        {
            let mut a = Tile::new_number(2);
            let mut b = Tile::new_multiplier(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_number(4));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 4);
        }

        // Number + Divider => Number
        {
            let mut a = Tile::new_number(2);
            let mut b = Tile::new_divider(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_number(1));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, -1);
        }

        // Number + Divider => Empty
        {
            let mut a = Tile::new_number(2);
            let mut b = Tile::new_divider(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Number + Bomb
        {
            let mut a = Tile::new_number(2);
            let mut b = Tile::new_bomb();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, -2);
        }

        // Multiplier + Empty
        {
            let mut a = Tile::new_multiplier(1);
            let mut b = Tile::new_empty();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_multiplier(1));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Multiplier + Number
        {
            let mut a = Tile::new_multiplier(1);
            let mut b = Tile::new_number(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_number(4));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 4);
        }

        // Multiplier + Multiplier
        {
            let mut a = Tile::new_multiplier(1);
            let mut b = Tile::new_multiplier(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_multiplier(3));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Multiplier + Divider => Multiplier
        {
            let mut a = Tile::new_multiplier(2);
            let mut b = Tile::new_divider(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_multiplier(1));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Multiplier + Divider => Divider
        {
            let mut a = Tile::new_multiplier(1);
            let mut b = Tile::new_divider(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_divider(1));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Multiplier + Divider => Empty
        {
            let mut a = Tile::new_multiplier(1);
            let mut b = Tile::new_divider(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Multiplier + Bomb
        {
            let mut a = Tile::new_multiplier(1);
            let mut b = Tile::new_bomb();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Divider + Empty
        {
            let mut a = Tile::new_divider(1);
            let mut b = Tile::new_empty();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_divider(1));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Divider + Number => Number
        {
            let mut a = Tile::new_divider(1);
            let mut b = Tile::new_number(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_number(1));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, -1);
        }

        // Divider + Number => Empty
        {
            let mut a = Tile::new_divider(2);
            let mut b = Tile::new_number(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Divider + Multiplier => Multiplier
        {
            let mut a = Tile::new_divider(1);
            let mut b = Tile::new_multiplier(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_multiplier(1));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Divider + Multiplier => Divider
        {
            let mut a = Tile::new_divider(2);
            let mut b = Tile::new_multiplier(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_divider(1));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Divider + Multiplier => Empty
        {
            let mut a = Tile::new_divider(1);
            let mut b = Tile::new_multiplier(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Divider + Divider
        {
            let mut a = Tile::new_divider(1);
            let mut b = Tile::new_divider(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_divider(3));
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Divider + Bomb
        {
            let mut a = Tile::new_divider(1);
            let mut b = Tile::new_bomb();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Bomb + Empty
        {
            let mut a = Tile::new_bomb();
            let mut b = Tile::new_empty();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_bomb());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Bomb + Number
        {
            let mut a = Tile::new_bomb();
            let mut b = Tile::new_number(2);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, -2);
        }

        // Bomb + Multiplier
        {
            let mut a = Tile::new_bomb();
            let mut b = Tile::new_multiplier(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Bomb + Divider
        {
            let mut a = Tile::new_bomb();
            let mut b = Tile::new_divider(1);
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }

        // Bomb + Bomb
        {
            let mut a = Tile::new_bomb();
            let mut b = Tile::new_bomb();
            let score = Tile::merge_tiles(&mut a, &mut b);

            assert_eq!(a, Tile::new_empty());
            assert_eq!(b, Tile::new_empty());
            assert_eq!(score, 0);
        }
    }
}
