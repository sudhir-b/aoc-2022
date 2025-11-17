# Advent of Code Solutions

This repository contains Rust solutions for [Advent of Code](https://adventofcode.com/) Days 1-7. Each day's solution is implemented as an independent binary crate within a Cargo workspace.

## Project Structure

```
.
├── Cargo.toml          # Workspace configuration
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
- `src/input.txt` - Puzzle input (embedded at compile time via `include_str!`)

## Requirements

- Rust 1.56+ (Edition 2021)
- Cargo

## Building

Build all solutions:
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
# ... etc
```

Or run from a specific day directory:
```bash
cd day1
cargo run --release
```

## Solutions Overview

### Day 1: Calorie Counting
Aggregates calorie totals from input groups and finds the top three totals. Uses simple parsing and sorting to identify elves carrying the most calories.

**Key concepts:** String parsing, sorting, sum aggregation

### Day 2: Rock Paper Scissors
Models a Rock/Paper/Scissors tournament with enum-based scoring logic. Calculates total score based on opponent choices and desired outcomes.

**Key concepts:** Enums, pattern matching, `FromStr` trait implementation

### Day 3: Rucksack Reorganization
Uses bitset-style priority tracking to find common items across groups of rucksacks. Efficiently identifies shared items using bitwise operations.

**Key concepts:** Bitsets, HashMap, iterator chunking, bitwise AND operations

### Day 4: Camp Cleanup
Parses range assignments to detect containment and overlap between pairs. Both Part 1 (full containment) and Part 2 (any overlap) solutions included.

**Key concepts:** Range operations, parsing, functional filtering

### Day 5: Supply Stacks
Simulates crate stack movements based on movement commands. Models stack operations with vectors and processes sequential move instructions.

**Key concepts:** Stack operations, state manipulation, parsing

### Day 6: Tuning Trouble
Scans input with sliding windows to find unique character sequences. Uses HashSet to check for uniqueness in windows of size 4 (Part 1) and 14 (Part 2).

**Key concepts:** Sliding windows, HashSet, uniqueness detection

### Day 7: No Space Left On Device
Builds a directory tree structure using `Rc<RefCell<Node>>` for shared mutable references. Computes directory sizes and traverses the filesystem tree to find directories matching size criteria.

**Key concepts:** Tree structures, `Rc<RefCell<T>>` pattern, recursive traversal, interior mutability

## Design Philosophy

All solutions:
- Use **only the Rust standard library** (no external dependencies)
- Embed input files at **compile time** using `include_str!`
- Are implemented as **independent binary crates** within a workspace
- Focus on **idiomatic Rust** patterns and ownership principles

Several solutions include optional timing instrumentation using `std::time::Instant` to measure execution performance.

## Testing

Run tests for all days:
```bash
cargo test
```

Run tests for a specific day:
```bash
cargo test -p day3
```

## License

This is a personal learning project for Advent of Code challenges.
