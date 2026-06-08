use std::{cell::OnceCell, fmt::Display};

/// Note: Multiplier and Divider variants store the base 2 power (aka exponent)
/// of the scalar multiplier value.
///
/// Example: `Multiplier(3)` means multiply by 2^3, and it is displayed as `*8`.
#[derive(Debug, Clone, Copy)]
enum TileType {
    Empty,
    Number(u16),
    Multiplier(u8),
    Divider(u8),
}

#[derive(Debug, Clone)]
pub struct Tile {
    cached_string_repr: OnceCell<String>,
    tile_type: TileType,
}

impl Tile {
    pub fn new_empty() -> Self {
        Self {
            cached_string_repr: OnceCell::new(),
            tile_type: TileType::Empty,
        }
    }

    /// If `value` is 0, this constructor creates an empty tile instead.
    pub fn new_number(value: u16) -> Self {
        let tile_type = if value == 0 {
            TileType::Empty
        } else {
            TileType::Number(value)
        };

        Self {
            cached_string_repr: OnceCell::new(),
            tile_type,
        }
    }

    pub fn new_multiplier(power: u8) -> Self {
        Self {
            cached_string_repr: OnceCell::new(),
            tile_type: TileType::Multiplier(power),
        }
    }

    pub fn new_divider(power: u8) -> Self {
        Self {
            cached_string_repr: OnceCell::new(),
            tile_type: TileType::Divider(power),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.tile_type, TileType::Empty)
    }

    pub fn get_value(&self) -> Option<u16> {
        match self.tile_type {
            TileType::Number(value) => Some(value),
            _ => None,
        }
    }

    /// TODO: Instead, apply the flyweight pattern to cache all
    /// common and unexpected cases as they occur.
    ///
    /// For commonly occurring tiles, this getter method simply returns
    /// hard-coded static values.
    ///
    /// For other tiles, this method updates and returns this instance's cached value
    /// with the interior mutability pattern.
    pub fn get_str(&self) -> &str {
        match self.tile_type {
            TileType::Empty => ".",
            TileType::Number(1) => "1",
            TileType::Number(2) => "2",
            TileType::Number(4) => "4",
            TileType::Number(8) => "8",
            TileType::Number(16) => "16",
            TileType::Number(32) => "32",
            TileType::Number(64) => "64",
            TileType::Number(128) => "128",
            TileType::Number(256) => "256",
            TileType::Number(512) => "512",
            TileType::Number(1024) => "1024",
            TileType::Number(2048) => "2048",
            TileType::Multiplier(1) => "*2",
            TileType::Multiplier(2) => "*4",
            TileType::Multiplier(3) => "*8",
            TileType::Divider(1) => "/2",
            TileType::Divider(2) => "/4",
            TileType::Divider(3) => "/8",
            _ => self.cached_string_repr.get_or_init(|| self.to_string()),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "." => Self::new_empty(),
            _ if s.starts_with("*") => {
                let power = s[1..].parse::<u8>().unwrap();
                Self::new_multiplier(power)
            }
            _ if s.starts_with("/") => {
                let power = s[1..].parse::<u8>().unwrap();
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
    /// This also implements Tile::to_string()
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tile_type {
            TileType::Empty => write!(f, "."),
            TileType::Number(value) => write!(f, "{}", value),
            TileType::Multiplier(power) => write!(f, "*{}", 2 << (power - 1)),
            TileType::Divider(power) => write!(f, "/{}", 2 << (power - 1)),
        }
    }
}

/// These operations are for resolving the merging of 2 adjacent tiles.
impl Tile {
    /// Checks if the given 2 tiles can be merged.
    pub fn are_mergeable(a: &Self, b: &Self) -> bool {
        use TileType::{Divider, Empty, Multiplier, Number};

        match (a.tile_type, b.tile_type) {
            // No change.
            (Empty, Empty) => false,

            // No change.
            (Empty, Number(_)) => false,

            // No change.
            (Empty, Multiplier(_)) => false,

            // No change.
            (Empty, Divider(_)) => false,

            // No change.
            (Number(_), Empty) => false,

            // If both numbers have same value, then merge.
            // Otherwise, no change.
            (Number(a_value), Number(b_value)) => a_value == b_value,

            // Calculate product.
            (Number(_), Multiplier(_)) => true,

            // Calculate quotient.
            (Number(_), Divider(_)) => true,

            // No change.
            (Multiplier(_), Empty) => false,

            // Calculate product.
            (Multiplier(_), Number(_)) => true,

            // Merge multipliers.
            (Multiplier(_), Multiplier(_)) => true,

            // Merge multiplier and divider.
            (Multiplier(_), Divider(_)) => true,

            // No change.
            (Divider(_), Empty) => false,

            // Calculate quotient.
            (Divider(_), Number(_)) => true,

            // Merge divider and multiplier.
            (Divider(_), Multiplier(_)) => true,

            // Merge dividers.
            (Divider(_), Divider(_)) => true,
        }
    }

    /// If a merge is possible, perform the merge operation,
    /// set `a` to the result, set `b` to empty, and
    /// return a score from this merge.
    pub fn merge_tiles(a: &mut Self, b: &mut Self) -> i16 {
        use TileType::{Divider, Empty, Multiplier, Number};

        match (a.tile_type, b.tile_type) {
            // No change.
            (Empty, Empty) => 0,

            // No change.
            (Empty, Number(_)) => 0,

            // No change.
            (Empty, Multiplier(_)) => 0,

            // No change.
            (Empty, Divider(_)) => 0,

            // No change.
            (Number(_), Empty) => 0,

            // If both numbers have same value, then calculate sum.
            // Otherwise, no change.
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

            // Calculate product.
            (Number(value), Multiplier(power)) => {
                let (product, score) = Self::calculate_product_and_score(value, power);
                *a = Self::new_number(product);
                *b = Self::new_empty();
                score
            }

            // Calculate quotient.
            (Number(value), Divider(power)) => {
                let (quotient, score) = Self::calculate_quotient_and_score(value, power);
                *a = Self::new_number(quotient);
                *b = Self::new_empty();
                score
            }

            // No change.
            (Multiplier(_), Empty) => 0,

            // Calculate product.
            (Multiplier(power), Number(value)) => {
                let (product, score) = Self::calculate_product_and_score(value, power);
                *a = Self::new_number(product);
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
            (Multiplier(mult_power), Divider(div_power)) => {
                *a = Self::merge_multiplier_and_divider(mult_power, div_power);
                *b = Self::new_empty();
                0
            }

            // No change.
            (Divider(_), Empty) => 0,

            // Calculate quotient.
            (Divider(power), Number(value)) => {
                let (quotient, score) = Self::calculate_quotient_and_score(value, power);
                *a = Self::new_number(quotient);
                *b = Self::new_empty();
                score
            }

            // Merge divider and multiplier.
            (Divider(div_power), Multiplier(mult_power)) => {
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
    fn test_are_mergeable() {
        let empty = Tile::new_empty();
        let number2 = Tile::new_number(2);
        let number4 = Tile::new_number(4);
        let multiplier = Tile::new_multiplier(1);
        let divider = Tile::new_divider(1);

        assert!(!Tile::are_mergeable(&empty, &empty));
        assert!(!Tile::are_mergeable(&empty, &number2));
        assert!(!Tile::are_mergeable(&empty, &multiplier));
        assert!(!Tile::are_mergeable(&empty, &divider));

        assert!(!Tile::are_mergeable(&number2, &empty));
        assert!(Tile::are_mergeable(&number2, &number2));
        assert!(!Tile::are_mergeable(&number2, &number4));
        assert!(Tile::are_mergeable(&number2, &multiplier));
        assert!(Tile::are_mergeable(&number2, &divider));

        assert!(!Tile::are_mergeable(&multiplier, &empty));
        assert!(Tile::are_mergeable(&multiplier, &number2));
        assert!(Tile::are_mergeable(&multiplier, &multiplier));
        assert!(Tile::are_mergeable(&multiplier, &divider));

        assert!(!Tile::are_mergeable(&divider, &empty));
        assert!(Tile::are_mergeable(&divider, &number2));
        assert!(Tile::are_mergeable(&divider, &multiplier));
        assert!(Tile::are_mergeable(&divider, &divider));
    }
}
