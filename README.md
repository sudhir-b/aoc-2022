# Advent of Code Solutions

This repository contains Rust solutions for the first 7 days of an Advent of Code challenge. It is organized as a Cargo workspace.

## Project Structure

The project is a Cargo workspace with independent binary crates for each day:

- `day1`: Calorie Counting
- `day2`: Rock Paper Scissors
- `day3`: Rucksack Reorganization
- `day4`: Camp Cleanup
- `day5`: Supply Stacks
- `day6`: Tuning Trouble
- `day7`: No Space Left On Device

Each crate is self-contained and includes its own input file (`input.txt`) which is embedded at compile time using `include_str!`.

## Prerequisites

- Rust (stable)

## How to Run

You can run the solution for a specific day using `cargo run`. Since this is a workspace, you need to specify the package with `-p`.

For example, to run Day 1:

```bash
cargo run -p day1
```

To run Day 7:

```bash
cargo run -p day7
```

## Implementation Details

- **Day 1**: Aggregates calorie totals to find the max calories carried by elves.
- **Day 2**: Calculates scores for Rock/Paper/Scissors strategy guides using enums.
- **Day 3**: Uses bitsets to find common item types in rucksacks.
- **Day 4**: Parses range assignments to detect containment/overlap.
- **Day 5**: Simulates crate movement between stacks.
- **Day 6**: Detects start-of-packet markers using sliding windows and `HashSet` for uniqueness checks.
- **Day 7**: Builds a directory tree structure using `Rc<RefCell<Node>>` to determine directory sizes.

Most solutions measure and print their execution time using `std::time::Instant`.
