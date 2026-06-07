# Rusty 2048

This is my version of the [2048 Game](https://play2048.co/), built in Rust using the [Ratatui](https://ratatui.rs/) library.

My game expands on the original 2048 game with many exciting new features, such as user-customizable grid size, enhanced gameplay with multiplier/divider tiles, and more to come.

Since this game is a [TUI](https://en.wikipedia.org/wiki/Text-based_user_interface) app, it has graphical limitations tied to the terminal emulator settings. Currently, there is no animation at all.

## Quick Explanation of Game

You slide tiles to move and merge them into tiles with bigger numbers. To win, your goal is to make a tile with the value `2048`. You lose when the grid is full, and you can no longer affect the tiles in any way.

## Gameplay Controls

- Press `Q` (Quit) to exit.
- Press `R` (Restart) to start a new game.
- Press `G` (Grid) to toggle visibility of empty tiles.
- Press `W`, `A`, `S`, `D` or `ARROW KEYS` to slide tiles in the grid.

### Quick Tips

- After opening the game, resize your terminal window until all tiles look the same size, and then press `G` to play with the grid off.

- You may spam-click the same direction to keep the current grid state while waiting for the game to randomly spawn a new tile.

## How to Run

```
cargo run -- [<num_rows> <num_cols>]
```

`num_rows` and `num_cols` must be integers >= 2.

### Examples

You can run the game with no arguments. This starts a game with the default grid size.

```
cargo run
```

You can run the game with your specific grid size.

```
cargo run -- 6 8
```

## Possibly Upcoming Features

- Add feature to undo/rollback to a previous turn state.
- Add feature to auto-save current game state on exit
- Add feature to auto-load the saved state on opening the game.
- Add feature toggle on/off AI auto-play mode.
  - AI may choose a direction randomly.
  - AI may choose greedily the direction that produces a highest score in the immediate next turn.
- Add visuals to distinguish a tile that was just randomly spawned this turn.
- Add animations?!
