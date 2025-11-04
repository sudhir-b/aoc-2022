# Advent of Code 2022 - Rust Solutions

A Rust Cargo workspace containing solutions for Advent of Code 2022 puzzles for days 1–7. Each day is implemented as an independent binary crate within the workspace.

## Project Structure

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

## Solutions Overview

### Day 1: Calorie Counting
- **Approach**: Aggregates calorie totals per elf and finds the top three
- **Key Features**: Efficient sorting and summing of calorie counts
- **Input**: Embedded at compile-time using `include_str!`

### Day 2: Rock Paper Scissors
- **Approach**: Models game scoring with enums and pattern matching
- **Key Features**: 
  - `FromStr` implementations for parsing opponent and player choices
  - Strategic scoring based on win/draw/lose outcomes
  - Constants for move scores (Rock=1, Paper=2, Scissors=3)

### Day 3: Rucksack Reorganization
- **Approach**: Bitset-style priority tracking across item groups
- **Key Features**: Efficient character prioritization and group processing

### Day 4: Camp Cleanup
- **Approach**: Parses range assignments to detect containment and overlap
- **Key Features**: Set operations on range intervals

### Day 5: Supply Stacks
- **Approach**: Simulates crate stacks following movement commands
- **Key Features**: Stack data structure manipulation with crate movements

### Day 6: Tuning Trouble
- **Approach**: Scans for start-of-packet and start-of-message markers using sliding windows
- **Key Features**: 
  - `HashSet` uniqueness checks
  - Performance timing with `std::time::Instant`
  - Separate `part_1()` and `part_2()` functions

### Day 7: No Space Left On Device
- **Approach**: Builds a directory tree using `Rc<RefCell<Node>>` to compute size metrics
- **Key Features**:
  - Tree traversal algorithms
  - Directory size calculations
  - Memory-safe shared references with reference counting

## Common Patterns

### Input Handling
All solutions use compile-time embedded input files via `include_str!`, ensuring the input is bundled with the binary at build time.

### Performance Timing
Several solutions (days 6, 7) include execution timing using `std::time::Instant` to measure performance.

### Modular Design
Many solutions expose `part_1` and `part_2` helper functions that are invoked from `main`, enabling clean separation of puzzle parts.

### Dependencies
All solutions rely solely on the Rust standard library—no external dependencies are required.

## Building and Running

### Building the Entire Workspace
```bash
cargo build
```

### Running a Specific Day
```bash
# Run day 1 solution
cargo run -p day1

# Run day 6 solution
cargo run -p day6
```

### Building Individual Days
```bash
cargo build -p day3
```

## Requirements

- Rust 2021 Edition
- Cargo (included with Rust)

## Workspace Configuration

The project is organized as a Cargo workspace with all day crates listed as members in the root `Cargo.toml`. This allows for convenient building and testing of all solutions together or individually.

Each day crate is self-contained with its own `Cargo.toml` and can be developed and tested independently.
