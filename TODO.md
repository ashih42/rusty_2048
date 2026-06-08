# To-Do Notes

- Write unit tests for:
  - `Grid::contains_value`
  - `Grid::is_dead`
  - `Grid::has_possible_merges`

- Eliminate `Tile.cached_string_repr` since it is just unused space for most tiles.
  - Make `Tile` simply an enum, `replacing TileType`.
  - Use a global hashmap to resolve `Tile.get_str`.

- Apply flyweight pattern on `Tile::get_str`:
  - Have 3 global hashmaps:
    - number_to_str
    - multiplier_to_str
    - divider_to_str

- Consider structring the module with a hierarchy appropriate for different levels of abstractions.
  - Remove `lib.rs`

- Adjust game difficulty to become harder.
  - Try spawning more tiles every turn.
  - Try scaling the amount spawned based on amount of empty space remaining.
