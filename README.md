# Advent of Code 2022 – Rust Solutions

## Overview

This repository contains Rust implementations for the first seven puzzles from [Advent of Code 2022](https://adventofcode.com/2022). Each day lives in its own binary crate inside a shared Cargo workspace and embeds the puzzle input at compile time using `include_str!`. The solutions rely solely on the Rust standard library.

## Repository layout

| Crate | Puzzle | Highlights |
| --- | --- | --- |
| `day1` | Calorie Counting | Aggregates calorie totals for each elf and reports the sum of the top three. |
| `day2` | Rock Paper Scissors | Implements hand-shape and outcome decoding to compute total tournament score. |
| `day3` | Rucksack Reorganization | Uses bitset-style presence tracking to find shared item priorities across groups of three rucksacks. |
| `day4` | Camp Cleanup | Parses assignment ranges to detect full containment (part 1) and overlap (part 2). |
| `day5` | Supply Stacks | Simulates crate stacks, supporting both single-crate and multi-crate moves. |
| `day6` | Tuning Trouble | Slides a window over the datastream buffer to locate the first marker with unique characters. |
| `day7` | No Space Left On Device | Builds an in-memory directory tree to compute cumulative sizes and identify candidate directories for deletion. |

Every crate exposes a `main.rs` that prints the relevant answer(s) when run. Some binaries call `part_2` by default; run the alternative helper or adjust `main` if you wish to inspect `part_1` results.

## Getting started

1. Install [Rust](https://www.rust-lang.org/tools/install) (the standard toolchain includes Cargo).
2. Clone the repository and change into the project directory.
3. Run any puzzle with Cargo, for example:

   ```bash
   cargo run -p day1         # Day 1 – Calorie Counting
   cargo run -p day4 --release  # Run day 4 with optimizations enabled
   ```

### Working with puzzle inputs

Each crate expects its input file at `dayN/src/input.txt`. Replace the bundled files with your personal puzzle data if you want to reproduce your Advent of Code answers. Because the inputs are compiled into the binaries via `include_str!`, recompile (`cargo run …`) after changing any input file.

## Development notes

- Use `cargo fmt` to format the code and `cargo clippy` for linting before submitting changes.
- The workspace has no shared library code, so each day can be developed and tested independently.
- Performance-sensitive runs benefit from `cargo run --release -p dayN`.

Have fun exploring the puzzles, and happy coding!
