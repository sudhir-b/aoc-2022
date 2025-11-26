# Advent of Code 2022 - Rust Solutions

A Rust Cargo workspace containing solutions for Advent of Code 2022, days 1–7. Each day is an independent binary crate that solves its respective puzzle using only the Rust standard library.

## Project Structure

This is a Cargo workspace with seven binary crates:

```
.
├── Cargo.toml
├── day1/
├── day2/
├── day3/
├── day4/
├── day5/
├── day6/
└── day7/
```

Each `day*` crate contains:
- `src/main.rs` - Solution code
- `src/input.txt` - Puzzle input (embedded via `include_str!`)
- `Cargo.toml` - Crate configuration

## Solutions Overview

### Day 1: Calorie Counting
Aggregates calorie totals from grouped input and finds the top 3 elves with the most calories. Implements a simple sort-based approach to identify the highest values.

**Key techniques:**
- String splitting and parsing
- Sorting and aggregation

### Day 2: Rock Paper Scissors
Models a Rock-Paper-Scissors tournament with strategic scoring. Implements enums for opponent and player choices, with a scoring function that accounts for both the outcome (lose/draw/win) and the choice made.

**Key techniques:**
- `FromStr` trait implementation
- Pattern matching with enums
- Score calculation logic

### Day 3: Rucksack Reorganization
Finds common items across groups of rucksacks using bitset-style priority tracking. Processes rucksacks in groups of three and identifies common priority items using bitwise operations.

**Key techniques:**
- HashMap for priority mapping (a–z: 1–26, A–Z: 27–52)
- Bitset arrays for set operations
- Bitwise AND for intersection

### Day 4: Camp Cleanup
Detects overlapping range assignments between elf pairs. Solves both part 1 (fully contained ranges) and part 2 (any overlap detection).

**Key techniques:**
- Range parsing and comparison
- Containment detection logic
- Overlap detection with multiple conditions

### Day 5: Supply Stacks
Simulates crate movements across stacks. Models a 9-stack system with crate labels and processes movement commands to determine the final top-of-stack configuration. Includes both part 1 (reversed order) and part 2 (maintained order) implementations.

**Key techniques:**
- Vector-based stack simulation
- Crate movement and reversal logic
- State mutation and final state extraction

### Day 6: Tuning Trouble
Finds the position of the first start-of-packet marker (4 unique characters) and start-of-message marker (14 unique characters) in a signal stream using a sliding window approach.

**Key techniques:**
- Sliding window technique
- HashSet for uniqueness checks
- Character iteration and window management

### Day 7: No Space Left On Device
Builds a directory tree from terminal commands and traverses it to calculate directory sizes. Computes part 1 (sum of small directories ≤100,000 bytes) and part 2 (smallest directory to delete for required space).

**Key techniques:**
- `Rc<RefCell<Node>>` for shared mutable tree nodes
- Tree traversal and DFS
- Recursive size calculation
- Filtering and min/max operations

## Building and Running

Build all crates:
```bash
cargo build --release
```

Run a specific day's solution:
```bash
cargo run --release -p day1
cargo run --release -p day2
# ... etc
```

Run all tests (if any):
```bash
cargo test
```

## Technical Notes

- **No external dependencies:** All solutions use only the Rust standard library (`std`)
- **Embedded input:** Each solution embeds its puzzle input via `include_str!("input.txt")`
- **Performance tracking:** Most solutions include `std::time::Instant` to measure execution time
- **Edition:** Targets Rust 2021 edition

## Implementation Highlights

- Day 2 demonstrates `FromStr` trait implementation for custom enum parsing
- Day 3 showcases bitset-based set operations for efficient intersection finding
- Day 7 is the most complex, featuring smart pointer patterns (`Rc<RefCell<Node>>`) for tree construction and traversal
- Performance-conscious implementations with timing measurements
