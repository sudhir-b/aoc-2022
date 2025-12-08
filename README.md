# Advent of Code 2022 – Rust Workspace

This repository hosts my solutions for Advent of Code 2022, implemented in Rust. The workspace is split into one binary crate per puzzle day, making it easy to build, run, and iterate on each solution independently while sharing the same toolchain.

## Workspace Layout

| Crate | Puzzle | Highlights |
| --- | --- | --- |
| `day1` | Calorie Counting | Aggregates calorie totals per elf and sums the three highest values. |
| `day2` | Rock Paper Scissors | Scores rounds with enums for opponent moves and desired outcomes. |
| `day3` | Rucksack Reorganization | Uses bitset-like arrays to track character priorities across groups of three lines. |
| `day4` | Camp Cleanup | Parses range assignments to detect containment and overlap pairs. |
| `day5` | Supply Stacks | Simulates crate movements for both CrateMover 9000 and 9001 behaviors. |
| `day6` | Tuning Trouble | Slides a window across the datastream buffer to find unique marker sequences. |
| `day7` | No Space Left On Device | Builds a directory tree with `Rc<RefCell<Node>>` to compute directory sizes. |

Each crate embeds its puzzle input via `include_str!("input.txt")`, so editing `src/input.txt` in a day directory automatically refreshes the data on the next build.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (via `rustup`) with the stable toolchain
- `cargo` (installed with Rust)

## Running a Solution

Use `cargo run` and point to the crate for the day you want to execute:

```bash
# Run day 3 in debug mode
cargo run -p day3

# Run day 5 with optimizations enabled (useful for the puzzles with timing output)
cargo run -p day5 --release
```

## Adding a New Day

1. Create a new crate directory (e.g., `day8`) using `cargo new day8 --bin`.
2. Append the crate name to the `members` array in the workspace `Cargo.toml`.
3. Place your puzzle input in `day8/src/input.txt` and reference it with `include_str!`.
4. Implement `main.rs` following the existing style of separating `part_1`/`part_2` helpers when needed.

## Notes

- Several solutions print elapsed time using `std::time::Instant`; running with `--release` produces more accurate timings.
- The repository intentionally avoids external crates—everything is solved with the Rust standard library to keep builds fast and portable.

Happy puzzling!
