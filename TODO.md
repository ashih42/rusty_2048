# To-Do Notes

- Write unit tests for:
  - `Grid::contains_value`
  - `Grid::is_dead`
  - `Grid::has_possible_merges`
  - `Tile::are_mergeable`
  - `Tile::merge_tiles`

- Apply flyweight pattern on `Tile::get_str`:
  - Have 3 global hashmaps:
    - number_to_str
    - multiplier_to_str
    - divider_to_str

- Enforce hiding of low level details by enforcing a strict module hierarchy.
  - Remove `lib.rs`
