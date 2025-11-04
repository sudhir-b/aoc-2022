# Advent of Code Solutions in Rust

A Rust Cargo workspace containing solutions for Advent of Code days 1–7, implemented as independent binary crates.

## Project Structure

This workspace consists of seven independent binary crates, each solving a single day's puzzle:

- **day1**: Calorie counting - aggregates calorie totals across groups and finds the top 3 totals
- **day2**: Rock, Paper, Scissors - models game scoring with enums and calculates total scores based on outcomes
- **day3**: Rucksack Reorganization - uses bitset-style priority tracking to find common items across groups
- **day4**: Camp Cleanup - parses range assignments to detect containment and overlap conditions
- **day5**: Supply Stacks - simulates crate stack movements based on commands
- **day6**: Tuning Trouble - scans sliding windows to find markers with unique characters using HashSet
- **day7**: No Space Left On Device - builds a directory tree using `Rc<RefCell<Node>>` to compute directory sizes

## Building and Running

### Build All Crates

```bash
cargo build --release
```

### Run a Specific Day

```bash
cargo run --release -p day1
cargo run --release -p day2
# ... and so on for day3-day7
```

### Run with Debug Output

```bash
cargo run -p day7  # Uses debug output (dbg! macros)
```

## Implementation Details

### Key Features

- **No external dependencies**: All solutions rely solely on the Rust standard library
- **Embedded input**: Puzzle inputs are compiled into binaries using `include_str!` macro for efficient runtime performance
- **Performance tracking**: Several solutions use `std::time::Instant` to measure and display execution time
- **Functional style**: Leverages Rust's iterator combinators and pattern matching throughout

### Day-by-Day Highlights

- **Day 1**: Simple parsing and aggregation with sorting
- **Day 2**: Enum-based game state modeling with FromStr trait implementation
- **Day 3**: Bitset operations for efficient item tracking across rucksacks
- **Day 4**: Range parsing and overlap detection logic
- **Day 5**: Stack operations with crate movement simulation
- **Day 6**: Sliding window algorithm with HashSet uniqueness checks
- **Day 7**: Advanced data structure using `Rc<RefCell<>>` for tree traversal and size calculations

## Code Style

The code follows Rust conventions with:
- Idiomatic use of enums and pattern matching
- Iterator-based functional programming where practical
- Clear variable naming and minimal comments (complexity is self-explanatory)
- Direct output via `println!` and `dbg!` macros

## Notes

- Each day's solution includes both Part 1 and Part 2 implementations where applicable
- Some days switch between parts via main function calls (e.g., `fn main()` calls `part_2()`)
- Puzzle inputs are assumed to be in `src/input.txt` for each crate
