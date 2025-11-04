# Advent of Code - Rust Solutions

This repository contains Rust solutions for [Advent of Code](https://adventofcode.com/) puzzles (Days 1-7).

## Project Structure

This is a Cargo workspace containing seven independent binary crates, one for each day:

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

Each day is a standalone binary crate with:
- `src/main.rs` - Solution implementation
- `src/input.txt` - Puzzle input (embedded at compile-time via `include_str!`)

## Requirements

- Rust 1.56 or higher (Edition 2021)
- No external dependencies (all solutions use only the Rust standard library)

## Running Solutions

### Run a specific day

```bash
cargo run -p day1
cargo run -p day2
# ... etc
```

### Run all solutions

```bash
for day in day{1..7}; do
    echo "=== Running $day ==="
    cargo run -p $day
    echo
done
```

### Build all solutions

```bash
cargo build --release
```

## Solution Highlights

- **Day 1**: Aggregates calorie totals from grouped entries and finds the top three
- **Day 2**: Models Rock/Paper/Scissors scoring with enums and custom trait implementations
- **Day 3**: Uses bitset-style priority tracking for rucksack item organization across groups
- **Day 4**: Parses range assignments to detect containment and overlap patterns
- **Day 5**: Simulates crate stack operations with movement commands
- **Day 6**: Scans sliding windows with HashSet uniqueness checks for signal detection
- **Day 7**: Builds a directory tree using `Rc<RefCell<Node>>` for filesystem traversal and size computation

## Performance

Several solutions include execution timing using `std::time::Instant` to measure performance. Results are printed to the console along with puzzle answers.

## Development

Each crate is completely independent and can be modified, built, or run separately:

```bash
cd day3
cargo run
cargo test  # if tests are present
```

## License

This project is open source and available for educational purposes.
