# Advent of Code - Rust Solutions

A collection of solutions for [Advent of Code](https://adventofcode.com/) implemented in Rust.

## Project Structure

This is a Rust workspace containing solutions for each day of Advent of Code, organized as follows:

```
.
├── day1/          # Day 1 solution
├── day2/          # Day 2 solution
├── day3/          # Day 3 solution
├── day4/          # Day 4 solution
├── day5/          # Day 5 solution
├── day6/          # Day 6 solution
├── day7/          # Day 7 solution
└── Cargo.toml     # Workspace configuration
```

Each day directory contains:
- `src/main.rs` - The solution implementation
- `Cargo.toml` - Package configuration
- `input.txt` - The puzzle input (included via `include_str!` macro)

## Building

To build all solutions:

```bash
cargo build --release
```

To build a specific day:

```bash
cargo build -p day1 --release
```

## Running

To run a specific day's solution:

```bash
cargo run -p day1
cargo run -p day2
# ... and so on
```

Or to run all solutions, use the workspace:

```bash
cargo run --release
```

## Requirements

- Rust 1.56 or later (as specified in the workspace configuration)

## How to Add a New Day

1. Create a new directory (e.g., `day8/`)
2. Add it to the `members` list in the root `Cargo.toml`
3. Create a standard Rust binary project structure in that directory
4. Add your solution code to `src/main.rs`
5. Place your puzzle input in `input.txt` and load it using `include_str!("input.txt")`

## License

This project is intended for personal learning purposes. Advent of Code puzzles are created by Eric Wastl.

