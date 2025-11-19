# Advent of Code – Rust Workspace

This repository is a small Rust workspace containing solutions to Advent of Code puzzles (days 1–7). Each day lives in its own binary crate and is completely independent from the others.

## Repository layout

- `Cargo.toml` – Cargo workspace definition
- `day1` … `day7` – individual binary crates, one per day
  - `Cargo.toml` – crate manifest
  - `src/main.rs` – puzzle solution entrypoint
  - `src/input.txt` – puzzle input embedded at compile time

### Per‑day overview

- **day1** – Parses groups of calorie counts separated by blank lines, sums each group, and prints the sum of the three largest totals.
- **day2** – Rock–paper–scissors scoring. Interprets the second column of the input as the *desired outcome* (lose / draw / win) and computes the total score accordingly.
- **day3** – Rucksack priority calculation. Processes the input in groups of three lines, finds the shared item type using bitsets, and sums their priorities.
- **day4** – Range assignment analysis. Provides implementations for:
  - Part 1: count pairs where one range fully contains the other.
  - Part 2: count pairs where the ranges overlap at all (this is what `main` runs).
- **day5** – Crate‑moving simulation. Models nine stacks of crates and applies move instructions:
  - `part_1`: moves crates one at a time (reversing order).
  - `part_2`: moves crates in batches while preserving order (this is what `main` runs).
- **day6** – Communication marker detection using a sliding window and a `HashSet`:
  - `part_1`: finds the first start‑of‑packet marker (4 distinct characters).
  - `part_2`: finds the first start‑of‑message marker (14 distinct characters, run from `main`).
- **day7** – Builds an in‑memory directory tree using `Rc<RefCell<Node>>`, computes directory sizes, and reports:
  - The sum of total sizes of all small directories (≤ 100,000).
  - The size of the smallest directory that can be deleted to free enough space.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain with Cargo)

No external crates are used; everything is implemented with the Rust standard library.

## Running the solutions

From the repository root, you can build and run any day using Cargo. Because this is a workspace of binary crates, use the package name (`day1`, `day2`, …) when running:

```bash
# Run day 1
cargo run -p day1

# Run day 4 (runs part 2 by default)
cargo run -p day4

# Run day 7 in release mode for speed
cargo run -p day7 --release
```

Alternatively, you can `cd` into an individual day and run it directly:

```bash
cd day3
cargo run
```

## Puzzle inputs

Each crate reads its input using `include_str!("input.txt")`, so the input is compiled into the binary. To change or experiment with different inputs:

1. Edit the corresponding `src/input.txt` file for the day you care about.
2. Rebuild / rerun that day with Cargo.

No extra configuration is required beyond having Rust and Cargo installed.
