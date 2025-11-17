# Advent of Code 2022 – Rust Workspace

## Overview
This repository contains my solutions to the first seven days of [Advent of Code 2022](https://adventofcode.com/2022), implemented in idiomatic Rust. Each day lives in its own binary crate inside a shared Cargo workspace, and every solution embeds its puzzle input at compile time with `include_str!` so that it can be executed without any additional setup.

The code intentionally sticks to the Rust standard library—no external dependencies are required—making it easy to study different problem‑solving strategies that rely purely on core language features.

## Project structure
| Crate | Puzzle | Highlights |
| --- | --- | --- |
| `day1` | Calorie Counting | Splits the input into inventories per elf, tallies calories, and reports the sum of the top three totals. |
| `day2` | Rock Paper Scissors | Models opponent/player choices with enums and computes scores based on the intended outcome strategy. |
| `day3` | Rucksack Reorganization | Assigns priorities to characters, uses bitset‑style tracking across groups of three rucksacks, and sums the shared badge priorities. |
| `day4` | Camp Cleanup | Parses numeric range assignments to count full containment and overlap relationships between pairs. |
| `day5` | Supply Stacks | Simulates crate stacks with explicit vectors and provides separate procedures for part 1 (single moves) and part 2 (bulk moves). |
| `day6` | Tuning Trouble | Slides a fixed‑size window across the datastream buffer, using `HashSet` uniqueness checks to find packet/message markers. |
| `day7` | No Space Left On Device | Builds an in‑memory directory tree with `Rc<RefCell<Node>>`, calculates directory sizes, and answers both space‑usage queries. |

Each crate’s `src/input.txt` contains the corresponding puzzle input. Updating that file will recompile the crate because the contents are embedded directly into the binary.

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) 1.70+ with Cargo (any recent stable toolchain works)

If you do not have Rust installed, the recommended way is via `rustup`:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Running the solutions
From the workspace root, run an individual day with Cargo’s `-p/--package` flag:
```bash
# Debug build (faster compilation)
cargo run -p day3

# Optimized release build
cargo run -p day3 --release
```

Most binaries print the answer(s) for both parts and, where helpful, include simple timing output. Some days implement `part_1`/`part_2` helpers—feel free to change which function `main` calls if you want to inspect both variations.

### Using your own input
1. Replace the contents of `dayN/src/input.txt` with your data.
2. Re-run `cargo run -p dayN`. Because the input is compiled in, Cargo will automatically rebuild the binary when the file changes.

## Development tips
- Format the entire workspace before sending changes:
  ```bash
  cargo fmt --all
  ```
- Run clippy for additional linting:
  ```bash
  cargo clippy --all-targets --all-features
  ```
- Run the (currently empty) test suites if you add unit tests:
  ```bash
  cargo test --all
  ```

Feel free to use this repository as a learning reference or a starting point for your own Advent of Code journey. Happy coding!
