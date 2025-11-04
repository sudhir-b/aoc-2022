# Advent of Code - Rust Solutions

A collection of [Advent of Code](https://adventofcode.com/) puzzle solutions for Days 1-7, implemented in Rust.

## Project Structure

This repository is organized as a Cargo workspace containing seven independent binary crates, one for each day:

```
.
├── Cargo.toml          # Workspace manifest
├── day1/               # Day 1: Calorie Counting
├── day2/               # Day 2: Rock Paper Scissors
├── day3/               # Day 3: Rucksack Reorganization
├── day4/               # Day 4: Camp Cleanup
├── day5/               # Day 5: Supply Stacks
├── day6/               # Day 6: Tuning Trouble
└── day7/               # Day 7: No Space Left On Device
```

Each day's crate is self-contained with its own `main.rs` and puzzle input embedded at compile time using `include_str!`.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (2021 edition or later)
- Cargo (comes with Rust)

## Building

Build all days:
```bash
cargo build --release
```

Build a specific day:
```bash
cargo build --release -p day1
```

## Running Solutions

Run a specific day's solution:
```bash
cargo run --release -p day1
cargo run --release -p day2
# ... and so on
```

Or run from the day's directory:
```bash
cd day1
cargo run --release
```

## Solutions Overview

### Day 1: Calorie Counting
Aggregates calorie totals from grouped input data and finds the top three totals.

**Key concepts:** String parsing, vector aggregation, sorting

### Day 2: Rock Paper Scissors
Models a Rock/Paper/Scissors tournament with custom scoring rules.

**Key concepts:** Enums, pattern matching, `FromStr` trait implementation

### Day 3: Rucksack Reorganization
Finds common items across rucksack compartments using bitset-style priority tracking.

**Key concepts:** Set operations, bit manipulation, group processing

### Day 4: Camp Cleanup
Parses range assignments and detects containment and overlap relationships.

**Key concepts:** Range operations, input parsing, boolean logic

### Day 5: Supply Stacks
Simulates a crate stacking system with movement commands.

**Key concepts:** Stack operations, state mutation, command parsing

### Day 6: Tuning Trouble
Scans for unique character sequences using sliding window technique.

**Key concepts:** HashSet for uniqueness checks, sliding windows, string processing

### Day 7: No Space Left On Device
Builds and traverses a directory tree to compute size metrics.

**Key concepts:** Tree data structures, `Rc<RefCell<Node>>`, recursive traversal

## Implementation Details

- **Pure Rust**: All solutions use only the Rust standard library
- **Embedded Input**: Puzzle inputs are compiled into the binaries using `include_str!`
- **Performance Tracking**: Most solutions include execution time logging with `std::time::Instant`
- **Modular Design**: Part 1 and Part 2 solutions are often implemented as separate functions

## Performance

Each solution is optimized for clarity and correctness. Execution times are typically printed to the console where applicable.

## License

This project is provided as-is for educational purposes.

## Acknowledgments

Puzzles created by [Eric Wastl](http://was.tl/) for [Advent of Code](https://adventofcode.com/).
