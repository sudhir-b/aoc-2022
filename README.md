# Advent of Code Solutions

This repository contains Rust solutions for Advent of Code puzzles (Days 1-7).

## Project Structure

This is a Cargo workspace containing seven independent binary crates, one for each day:

```
.
├── Cargo.toml          # Workspace configuration
├── day1/               # Calorie Counting
├── day2/               # Rock Paper Scissors
├── day3/               # Rucksack Reorganization
├── day4/               # Camp Cleanup
├── day5/               # Supply Stacks
├── day6/               # Tuning Trouble
└── day7/               # No Space Left On Device
```

Each day's solution is a standalone binary crate with:
- `Cargo.toml` - Package configuration
- `src/main.rs` - Solution implementation
- `src/input.txt` - Puzzle input (embedded at compile-time via `include_str!`)

## Solutions Overview

### Day 1: Calorie Counting
Aggregates calorie totals from grouped input and finds the top three highest totals.

### Day 2: Rock Paper Scissors
Models Rock/Paper/Scissors scoring using enums and pattern matching to calculate game outcomes.

### Day 3: Rucksack Reorganization
Uses bitset-style priority tracking to find common items across groups of rucksacks.

### Day 4: Camp Cleanup
Parses range assignments and detects containment and overlap between pairs.

### Day 5: Supply Stacks
Simulates crate stacks and applies movement commands to reorganize them.

### Day 6: Tuning Trouble
Scans sliding windows with HashSet uniqueness checks to find distinct character sequences.

### Day 7: No Space Left On Device
Builds a directory tree using `Rc<RefCell<Node>>` to compute directory sizes and solve space-related queries.

## Requirements

- Rust 1.56.0 or later (2021 edition)
- Cargo

## Building

Build all days at once from the workspace root:

```bash
cargo build --release
```

Or build a specific day:

```bash
cargo build --release -p day1
```

## Running

Run a specific day's solution:

```bash
cargo run --release -p day1
cargo run --release -p day2
# ... etc
```

Some solutions include execution time measurements using `std::time::Instant`.

## Features

- **Zero external dependencies** - All solutions use only the Rust standard library
- **Compile-time input embedding** - Puzzle inputs are embedded using `include_str!()` for fast loading
- **Performance focused** - Release builds with optimized algorithms
- **Clean architecture** - Each day is completely independent with clear separation

## Implementation Notes

- **Day 1**: Simple aggregation with sorting
- **Day 2**: Enum-based state modeling with `FromStr` trait implementations
- **Day 3**: Bitset operations for efficient set intersections
- **Day 4**: Range parsing and containment logic
- **Day 5**: Stack-based simulation with vector operations
- **Day 6**: Sliding window with HashSet for uniqueness
- **Day 7**: Tree traversal with smart pointers (`Rc<RefCell<Node>>`) for shared ownership

## Development

Each day can be developed and tested independently. The workspace structure allows:

```bash
# Run from workspace root
cargo run -p day3

# Or navigate to specific day
cd day5
cargo run
```

## License

This project contains solutions to Advent of Code puzzles. Advent of Code is created by Eric Wastl.
