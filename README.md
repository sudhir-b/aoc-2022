# Advent of Code - Rust Solutions

A Rust Cargo workspace containing solutions to Advent of Code puzzles for days 1–7. Each day is implemented as an independent binary crate with solutions using only the Rust standard library.

## Project Structure

This workspace is organized as follows:

```
.
├── day1/          # Calorie Counting - aggregates and finds top totals
├── day2/          # Rock Paper Scissors - scoring with pattern matching
├── day3/          # Rucksack Reorganization - bitset-style priority tracking
├── day4/          # Camp Cleanup - range parsing and overlap detection
├── day5/          # Supply Stacks - stack-based movement simulation
├── day6/          # Tuning Trouble - sliding window uniqueness checks
├── day7/          # No Space Left On Device - directory tree with size metrics
└── Cargo.toml     # Workspace configuration
```

Each day has its own `Cargo.toml` and `src/main.rs`, with the puzzle input embedded directly into the binary using `include_str!()`.

## Building and Running

### Build all solutions
```bash
cargo build --release
```

### Run a specific day's solution
```bash
cargo run --release -p day1
cargo run --release -p day2
# ... etc for day3 through day7
```

### Run all tests
```bash
cargo test --release
```

## Solutions Overview

### Day 1: Calorie Counting
Aggregates calorie totals from groups of elves and identifies the top three highest totals.

**Techniques:** Parsing, sorting, aggregation

### Day 2: Rock Paper Scissors
Implements Rock/Paper/Scissors scoring logic with enums and pattern matching for both part 1 (direct moves) and part 2 (desired outcomes).

**Techniques:** Enums, `FromStr` trait, pattern matching, score calculation

### Day 3: Rucksack Reorganization
Tracks item priorities across rucksack compartments and groups using bitset-style techniques.

**Techniques:** Set operations, priority calculation, group processing

### Day 4: Camp Cleanup
Parses assignment ranges to detect full containment and partial overlaps.

**Techniques:** Parsing ranges, range comparison, interval overlap detection

### Day 5: Supply Stacks
Simulates a crate stack system, processing movement commands to determine final stack states.

**Techniques:** Stack data structures, command parsing, state manipulation

### Day 6: Tuning Trouble
Scans communication protocol data for start-of-packet and start-of-message markers using sliding windows with uniqueness checks.

**Techniques:** Sliding windows, `HashSet` for uniqueness checks

### Day 7: No Space Left On Device
Builds a directory tree representation and computes directory sizes using reference counting (`Rc`) and interior mutability (`RefCell`).

**Techniques:** `Rc<RefCell<T>>`, recursive tree traversal, file system simulation

## Dependencies

All solutions use only the Rust standard library—no external crates required. Each crate specifies `edition = "2021"`.

## Performance

Some solutions include timing measurements via `std::time::Instant` to track execution performance. Run with `--release` flag for optimized builds.

## License

This project contains solutions for Advent of Code puzzles. See [adventofcode.com](https://adventofcode.com) for more information.
