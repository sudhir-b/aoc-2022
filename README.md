# Advent of Code 2022 - Rust Solutions

A Rust workspace containing solutions for Advent of Code 2022 puzzles for days 1-7. Each day is implemented as an independent binary crate with its own solution logic and embedded input data.

## Project Structure

This is a Cargo workspace with the following layout:

```
.
├── Cargo.toml          # Workspace configuration
├── Cargo.lock          # Workspace lock file
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
- `src/main.rs` - Main solution code
- `src/input.txt` - Embedded puzzle input (via `include_str!`)

## Solutions Overview

### Day 1: Calorie Counting
- **Approach**: Aggregates calorie totals per elf and finds the top three
- **Key Techniques**: String parsing, vector sorting, summing slices
- **Complexity**: O(n log n) due to sorting

### Day 2: Rock Paper Scissors
- **Approach**: Uses enums to model choices and scoring logic
- **Key Techniques**: Enum implementations, `FromStr` trait, pattern matching
- **Features**: Separate handling for both parts of the puzzle

### Day 3: Rucksack Reorganization
- **Approach**: Bitset-style priority tracking across item groups
- **Key Techniques**: Character priority mapping, set operations
- **Complexity**: O(n) with efficient character handling

### Day 4: Camp Cleanup
- **Approach**: Parses range assignments to detect containment and overlap
- **Key Techniques**: Range parsing, vector operations, functional style
- **Features**: Separate `part_1()` and `part_2()` functions with timing

### Day 5: Supply Stacks
- **Approach**: Simulates crate stack movements following commands
- **Key Techniques**: Stack operations, command parsing, string manipulation
- **Features**: Part 1 and part 2 implementations with execution timing

### Day 6: Tuning Trouble
- **Approach**: Scans for start-of-packet and start-of-message markers using sliding windows
- **Key Techniques**: Sliding window algorithm, `HashSet` for uniqueness checks
- **Complexity**: O(n) with constant space for the window
- **Features**: 4-character and 14-character window sizes for parts 1 and 2

### Day 7: No Space Left On Device
- **Approach**: Builds a directory tree using `Rc<RefCell<Node>>` to compute size metrics
- **Key Techniques**: Tree data structures, shared ownership, recursive traversal
- **Features**: Directory size calculations and filesystem simulation

## Building and Running

### Prerequisites
- Rust 2021 edition toolchain
- Cargo (included with Rust)

### Building the Workspace
```bash
cargo build --workspace
```

### Running Individual Days
Each day can be run independently:

```bash
# Run day 1 solution
cargo run --bin day1

# Run day 2 solution
cargo run --bin day2

# ... and so on for days 3-7
```

### Building All at Once
```bash
cargo build --release --workspace
```

## Code Patterns

The solutions follow several consistent patterns:

1. **Input Embedding**: All solutions use `include_str!("input.txt")` to embed input data at compile time
2. **Standard Library Only**: Solutions rely solely on the Rust standard library
3. **Optional Timing**: Some solutions include `std::time::Instant` for performance measurement
4. **Helper Functions**: Several days expose `part_1()` and `part_2()` helper functions invoked from `main()`

## Performance Characteristics

- **Day 1**: Efficient sorting and slicing operations
- **Day 2**: Enum-based pattern matching for optimal performance
- **Day 3**: Linear time complexity with character-based operations
- **Day 4**: Functional style with efficient filtering
- **Day 5**: Stack-based operations with minimal allocations
- **Day 6**: O(n) sliding window with constant space
- **Day 7**: Tree traversal with shared ownership patterns

## License

This project contains solutions for Advent of Code 2022 puzzles. The code is provided for educational purposes to demonstrate various Rust programming techniques and problem-solving approaches.