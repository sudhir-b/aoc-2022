# Advent of Code 2022 - Rust Solutions

A Rust Cargo workspace containing solutions for Advent of Code 2022 puzzles for days 1-7. Each day is implemented as an independent binary crate with its own solution and embedded input data.

## Structure

This project is organized as a Cargo workspace with the following structure:

```
advent-of-code-2022/
├── Cargo.toml          # Workspace configuration
├── README.md           # This file
├── .gitignore          # Git ignore file
├── day1/               # Day 1: Calorie Counting
├── day2/               # Day 2: Rock Paper Scissors
├── day3/               # Day 3: Rucksack Reorganization
├── day4/               # Day 4: Camp Cleanup
├── day5/               # Day 5: Supply Stacks
├── day6/               # Day 6: Tuning Trouble
└── day7/               # Day 7: No Space Left On Device
```

Each day directory contains:
- `Cargo.toml` - Package configuration
- `src/main.rs` - Solution implementation
- `src/input.txt` - Puzzle input data (embedded at compile time)

## Solutions Overview

### Day 1: Calorie Counting
- **Approach**: Aggregates calorie totals by splitting on double newlines, sorts the results, and sums the top three values.
- **Key Features**: Efficient sorting and vector operations

### Day 2: Rock Paper Scissors
- **Approach**: Uses enums to model opponent choices and player outcomes with `FromStr` implementations for parsing.
- **Key Features**: Pattern matching for scoring logic, enum-based type safety

### Day 3: Rucksack Reorganization
- **Approach**: Implements bitset-style priority tracking across item groups.
- **Key Features**: Efficient character-to-priority mapping, set operations

### Day 4: Camp Cleanup
- **Approach**: Parses range assignments to detect containment and overlap between elf pairs.
- **Key Features**: Range parsing and comparison logic

### Day 5: Supply Stacks
- **Approach**: Simulates crate stacking and movement commands.
- **Key Features**: Stack operations and command parsing

### Day 6: Tuning Trouble
- **Approach**: Scans for start-of-packet markers using sliding windows with HashSet uniqueness checks.
- **Key Features**: Sliding window algorithm, performance timing with `std::time::Instant`

### Day 7: No Space Left On Device
- **Approach**: Builds a directory tree using `Rc<RefCell<Node>>` to compute size metrics and find directories for deletion.
- **Key Features**: Tree data structure with shared ownership, recursive size calculations

## Usage

### Running Individual Days

Each day can be run as an independent binary:

```bash
# Run Day 1 solution
cargo run --bin day1

# Run Day 2 solution
cargo run --bin day2

# ... and so on for days 3-7
```

### Building All Solutions

```bash
# Build all days in the workspace
cargo build

# Build with optimizations
cargo build --release
```

### Running Tests

```bash
# Run tests for all crates
cargo test

# Run tests for a specific day
cargo test -p day1
```

## Implementation Details

### Dependencies
- **Rust Standard Library Only**: All solutions rely solely on the Rust standard library
- **No External Dependencies**: Clean, dependency-free implementations
- **Edition 2021**: Modern Rust features and syntax

### Performance Considerations
- Several solutions include execution timing using `std::time::Instant`
- Compile-time embedding of input data via `include_str!` macro
- Efficient algorithms appropriate for each puzzle's constraints

### Code Organization
- Each day exposes `part_1` and `part_2` helper functions where applicable
- Main functions orchestrate the solution flow and handle output
- Consistent error handling with `expect()` for parsing operations

## Requirements

- Rust 1.56+ (Edition 2021)
- Cargo (included with Rust)

## Author

This repository contains solutions to Advent of Code 2022 puzzles, implemented in Rust as a learning exercise and demonstration of various algorithmic approaches and Rust programming patterns.
