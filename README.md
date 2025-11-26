# Advent of Code 2022 - Rust Solutions

A Rust workspace containing solutions for Advent of Code 2022, days 1-7. Each day is implemented as an independent binary crate using only the Rust standard library.

## Project Structure

This is a Cargo workspace with the following structure:

```
├── Cargo.toml          # Workspace configuration
├── Cargo.lock          # Lock file for all dependencies
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
- `src/main.rs` - Main solution implementation
- `src/input.txt` - Puzzle input embedded at compile time

## Running Solutions

### Individual Days
Run a specific day's solution:

```bash
# Run day 1 solution
cargo run --bin day1

# Run day 2 solution
cargo run --bin day2

# etc. for days 3-7
```

### All Solutions
Run all solutions in sequence:

```bash
# Run all day solutions
for i in {1..7}; do
    echo "Running day $i:"
    cargo run --bin day$i
    echo "---"
done
```

## Solution Highlights

### Day 1: Calorie Counting
- Aggregates calorie totals from elves' inventories
- Uses efficient sorting to find top three calorie counts
- **Complexity**: O(n log n) for sorting

### Day 2: Rock Paper Scissors
- Models game scoring with enums for opponent choices and player strategies
- Implements `FromStr` trait for parsing input characters
- Calculates scores based on win/draw/lose conditions
- **Complexity**: O(n)

### Day 3: Rucksack Reorganization
- Uses bitset-style tracking for finding common items across groups
- Implements priority mapping for alphabet characters (a-z, A-Z)
- Processes rucksacks in groups of three for badge identification
- **Complexity**: O(n) with bitwise operations

### Day 4: Camp Cleanup
- Parses range assignments to detect containment and overlap
- Implements interval comparison logic
- **Complexity**: O(n)

### Day 5: Supply Stacks
- Simulates crate movement commands using stack data structures
- Handles multiple stacks with LIFO operations
- **Complexity**: O(n × m) where m is number of stacks

### Day 6: Tuning Trouble
- Implements sliding window algorithm with HashSet for uniqueness checks
- Finds start-of-packet and start-of-message markers
- **Complexity**: O(n) with O(k) space where k is window size

### Day 7: No Space Left On Device
- Builds directory tree using `Rc<RefCell<Node>>` for shared mutable references
- Implements tree traversal for calculating directory sizes
- Computes space requirements for system cleanup
- Includes execution time measurement
- **Complexity**: O(n) for tree construction and traversal

## Technical Features

- **Zero Dependencies**: All solutions use only the Rust standard library
- **Compile-time Input Embedding**: Uses `include_str!` macro to embed input files at compile time
- **Performance**: Many solutions include timing measurements using `std::time::Instant`
- **Memory Efficiency**: Solutions are optimized for memory usage where appropriate
- **Error Handling**: Robust parsing with proper error handling for input data

## Building

Build all workspace members:

```bash
cargo build --release
```

Build individual day:

```bash
cargo build --bin day1 --release
```

## Requirements

- Rust 2021 edition
- Cargo (included with Rust)

## License

This project contains solutions for Advent of Code puzzles. The puzzle content and inputs are copyrighted by Advent of Code and should not be redistributed. The solution code is provided for educational purposes.