# Advent of Code – Rust solutions

This repository is a small Rust Cargo workspace containing solutions to the first seven days of **Advent of Code 2022**. Each day is implemented as its own binary crate.

## Layout

- `Cargo.toml` – workspace definition
- `day1` … `day7` – individual crates for each puzzle day
  - `src/main.rs` – solution for the day
  - `input.txt` – puzzle input for that day (embedded at compile time with `include_str!`)

Each crate is completely independent and uses only the Rust standard library.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or newer)

## Running the solutions

From the repository root you can run any day with Cargo, using the crate name as the package:

```bash
# Day 1
cargo run -p day1

# Day 2
cargo run -p day2

# Day 3
cargo run -p day3

# Day 4
cargo run -p day4

# Day 5
cargo run -p day5

# Day 6
cargo run -p day6

# Day 7
cargo run -p day7
```

By default each binary reads its corresponding `input.txt` via `include_str!` at compile time and prints the answer(s) for that day to stdout.

## Notes

- There are no shared libraries between days; each crate is focused on a single puzzle.
- Some days print timing information using `std::time::Instant` in addition to the puzzle answer.
