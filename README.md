# Advent of Code 2022 — Rust Workspace

This repository is a Cargo workspace that hosts my solutions for the first seven days of [Advent of Code 2022](https://adventofcode.com/2022). Every puzzle lives in its own binary crate (`day1` … `day7`) so the solutions stay isolated, fast to compile, and easy to run independently.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (the latest stable toolchain via `rustup` is recommended)
- `cargo`, which is installed automatically with Rust

## Repository layout

| Crate | Puzzle | Approach highlights | How to run |
| ----- | ------ | ------------------- | ---------- |
| `day1` | Calorie Counting | Parses groups of numbers, sums per-Elf calorie totals, and keeps the top three totals before summing them. | `cargo run -p day1 --release` |
| `day2` | Rock Paper Scissors | Models both opponents and desired outcomes with enums and scores rounds according to the new strategy guide interpretation. | `cargo run -p day2 --release` |
| `day3` | Rucksack Reorganization | Uses priority bitsets to track character occurrences across rucksack groups and sums the shared badge priorities. | `cargo run -p day3 --release` |
| `day4` | Camp Cleanup | Parses assignment ranges and filters the pairs that either fully contain or overlap each other. | `cargo run -p day4 --release` |
| `day5` | Supply Stacks | Simulates crate stacks with `Vec<Vec<&str>>`, applying the crane procedures for part 1 and part 2 variants. | `cargo run -p day5 --release` |
| `day6` | Tuning Trouble | Slides a window across the datastream and uses `HashSet` uniqueness checks to find packet/message markers. | `cargo run -p day6 --release` |
| `day7` | No Space Left On Device | Builds an in-memory directory tree (`Rc<RefCell<Node>>`) to compute directory sizes and determine deletion candidates. | `cargo run -p day7 --release` |

> ℹ️ Running with `--release` matches the timing measurements printed by some solutions, but it is optional during development.

## Updating puzzle input

Each crate embeds its puzzle input via `include_str!("input.txt")`. To try different data:

1. Open the corresponding `dayN/src/input.txt` file.
2. Replace its contents with your own puzzle input.
3. Re-run the crate with `cargo run -p dayN`.

Because the data is compiled into the binary, you must rebuild (or rerun) the crate after changing an input file.

## Building and validating

```bash
# Build everything
cargo build --workspace

# Run all (if/when tests exist)
cargo test --workspace

# Check formatting and lints
cargo fmt
cargo clippy --workspace --all-targets --all-features
```

## Adding more days

To extend the workspace with additional puzzles:

1. Create a new binary crate (for example `cargo new day8 --bin`).
2. Add the crate name to the `[workspace]` `members` array in the root `Cargo.toml`.
3. Put your solution in `day8/src/main.rs` (and `input.txt`).
4. Run it with `cargo run -p day8`.

Happy Advent of Coding! 🎄🦀
