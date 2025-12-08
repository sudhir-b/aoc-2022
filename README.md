# Advent of Code Solutions

A Rust Cargo workspace containing solutions for Advent of Code days 1–7.

## Overview

This project is a collection of seven independent binary crates, each solving a different Advent of Code puzzle. All solutions are implemented in pure Rust using only the standard library, with no external dependencies.

## Project Structure

```
.
├── day1/       # Calorie counting - aggregating elf calorie totals
├── day2/       # Rock-Paper-Scissors scoring with enum-based strategy
├── day3/       # Rucksack priority tracking across item groups
├── day4/       # Range assignment parsing and overlap detection
├── day5/       # Crate stack simulation with movement commands
├── day6/       # Sliding window uniqueness detection with HashSet
├── day7/       # Directory tree traversal and size computation
├── Cargo.toml  # Workspace configuration
└── Cargo.lock  # Dependency lock file
```

## Building and Running

Each day's solution is a standalone binary crate. To build and run a specific day:

```bash
# Run a specific day (e.g., day1)
cargo run --release -p day1

# Run all days
cargo run --release -p day2
cargo run --release -p day3
# ... and so on
```

## Features

### Day 1: Calorie Counting
Aggregates calorie totals across elf groups and identifies the top three totals.

### Day 2: Rock-Paper-Scissors
Implements game scoring logic using Rust enums:
- `OpponentChoice` enum for parsing opponent moves (A, B, C)
- `OldPlayerChoice` / `PlayerChoice` enums for player strategies
- Score constants for different outcomes

### Day 3: Rucksack Organization
Tracks item priorities across rucksack groups using bitset-style priority tracking to find common items.

### Day 4: Camp Cleanup
Parses assignment ranges and detects containment and overlap relationships between pairs.

### Day 5: Supply Stacks
Simulates a stack-based crate movement system processing command sequences.

### Day 6: Tuning Trouble
Uses sliding window technique with `HashSet` to detect signal start markers based on uniqueness constraints.

### Day 7: No Space Left On Device
Builds a directory tree structure using `Rc<RefCell<Node>>` for shared mutable ownership and computes directory sizes for analysis.

## Implementation Details

- **No External Dependencies**: All solutions use only Rust's standard library
- **Embedded Input**: Puzzle input is embedded at compile-time using `include_str!()`
- **Performance Tracking**: Several solutions optionally log execution time using `std::time::Instant`
- **Memory-Safe Code**: Heavy use of Rust's type system and ownership rules for safe, efficient implementations

## Getting Started

1. Clone the repository
2. Install Rust from [rust-lang.org](https://www.rust-lang.org/)
3. Navigate to the project directory
4. Run any day's solution:

```bash
cargo run --release -p day1
```

## License

These solutions are provided as educational examples for Advent of Code puzzles.
