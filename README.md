# Advent of Code Solutions

This repository contains solutions to [Advent of Code](https://adventofcode.com/) challenges written in Rust.

## Overview

The project is organized as a Rust workspace with separate binary crates for each day of the challenge. Each day contains a solution that reads input data and produces the expected output.

## Project Structure

```
├── day1/          # Day 1 solution
├── day2/          # Day 2 solution
├── day3/          # Day 3 solution
├── day4/          # Day 4 solution
├── day5/          # Day 5 solution
├── day6/          # Day 6 solution
├── day7/          # Day 7 solution
├── Cargo.toml     # Workspace configuration
└── Cargo.lock     # Dependency lock file
```

Each day directory contains:
- `src/main.rs` - The solution code
- `src/input.txt` - The puzzle input
- `Cargo.toml` - Package configuration

## Building and Running

### Build all solutions

```bash
cargo build --release
```

### Run a specific day's solution

```bash
cargo run --release -p day1
cargo run --release -p day2
# ... and so on for each day
```

### Run without optimizations (faster compilation)

```bash
cargo run -p day1
```

## Solutions

### Day 1
Calculates calorie counts carried by elves and finds the top 3 totals.

### Day 2
Rock, Paper, Scissors game strategy guide parser and score calculator.

### Day 3 - Day 7
Additional Advent of Code challenge solutions.

## Requirements

- Rust 1.56 or later (as specified by `edition = "2021"`)
- Cargo (comes with Rust)

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository
3. Navigate to the project directory

## Testing

To verify the solutions build correctly:

```bash
cargo check
```

To build and test:

```bash
cargo build
```

## Notes

- Each day's solution reads its puzzle input from `src/input.txt`
- Solutions are self-contained binaries that output their results to stdout
- The workspace structure allows running each day's solution independently

## License

These solutions are provided as-is for educational purposes.

