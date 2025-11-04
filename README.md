# Advent of Code - Rust Solutions

A Rust workspace containing solutions for [Advent of Code](https://adventofcode.com/) puzzles (Days 1-7).

## 📁 Project Structure

This project is organized as a Cargo workspace with seven independent binary crates, one for each day:

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

Each day is a self-contained binary crate with:
- `src/main.rs` - Solution implementation
- `src/input.txt` - Puzzle input (embedded at compile time using `include_str!`)
- `Cargo.toml` - Crate configuration

## 🛠️ Technologies

- **Language**: Rust (Edition 2021)
- **Dependencies**: Standard library only
- **Build System**: Cargo workspace

## 🚀 Usage

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.56.0 or later)

### Running Solutions

Run a specific day's solution:

```bash
cargo run --bin day1
cargo run --bin day2
# ... and so on
```

Run all solutions:

```bash
for day in day{1..7}; do
    echo "Running $day..."
    cargo run --bin $day
done
```

### Building

Build all crates:

```bash
cargo build --release
```

Build a specific day:

```bash
cargo build --bin day1 --release
```

## 📝 Solutions Overview

### Day 1: Calorie Counting
Aggregates calorie totals across groups and finds the top three highest totals.
- **Technique**: Vector sorting and summation

### Day 2: Rock Paper Scissors
Calculates scores for a Rock, Paper, Scissors tournament with specific strategy rules.
- **Technique**: Enums with `FromStr` trait implementation, pattern matching

### Day 3: Rucksack Reorganization
Finds common items across groups of rucksacks and calculates priority sums.
- **Technique**: Bitset-style priority tracking with arrays

### Day 4: Camp Cleanup
Analyzes range assignments to detect containment and overlap between pairs.
- **Technique**: Range parsing and comparison logic

### Day 5: Supply Stacks
Simulates moving crates between stacks following specific movement commands.
- **Technique**: Stack data structure manipulation

### Day 6: Tuning Trouble
Scans for start-of-packet markers by finding unique character sequences in a sliding window.
- **Technique**: Sliding window with `HashSet` for uniqueness checking

### Day 7: No Space Left On Device
Builds a directory tree from command history and calculates directory sizes.
- **Technique**: Tree structure using `Rc<RefCell<Node>>` for shared ownership

## ⚡ Performance Features

Several solutions include performance optimizations:
- Execution timing with `std::time::Instant`
- Bitset operations for efficient lookups (Day 3)
- `unstable_sort` for faster sorting where ordering doesn't matter (Day 1)
- Smart pointers (`Rc<RefCell<T>>`) for complex tree structures (Day 7)

## 📜 License

This project contains solutions to Advent of Code puzzles. Please refer to the [Advent of Code](https://adventofcode.com/) website for puzzle descriptions and terms of use.

## 🎯 Development Notes

- All solutions use only the Rust standard library (no external dependencies)
- Input files are embedded at compile time using `include_str!` macro
- Each day can be developed and tested independently
- Solutions focus on correctness and readability with some performance optimizations
