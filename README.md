# Advent of Code – Rust solutions (days 1–7)

This repository is a small Rust workspace containing solutions to the first seven days of an Advent of Code puzzle set. Each day is implemented as its own binary crate under the Cargo workspace.

All solutions are written using only the Rust standard library.

## Workspace layout

```text
.
├── Cargo.toml          # Workspace definition
├── day1/               # Day 1 – calorie counting / top‑3 totals
├── day2/               # Day 2 – rock–paper–scissors scoring
├── day3/               # Day 3 – rucksack priorities across groups of three
├── day4/               # Day 4 – range assignment containment & overlap checks
├── day5/               # Day 5 – crate stack manipulation ("crane" moves)
├── day6/               # Day 6 – start‑of‑packet / start‑of‑message marker search
└── day7/               # Day 7 – directory tree size analysis
```

Each `dayN` directory is a standalone Rust binary crate with its own `Cargo.toml` and `src/main.rs`.

## Requirements

- Rust toolchain (stable) with Cargo – install via <https://rustup.rs>

## Running the solutions

From the repository root you can run any day using Cargo's workspace support:

```bash
# Run day 1
cargo run -p day1

# Run day 4
cargo run -p day4

# Run day 7 with optimizations
cargo run -p day7 --release
```

Alternatively, you can `cd` into a day directory and run it as an independent Cargo project, e.g.:

```bash
cd day3
cargo run
```

## Input data

Each day's solution reads its puzzle input at **compile time** using `include_str!("input.txt")` (or an equivalent macro), so the input file for a day must live alongside `main.rs`, typically at:

```text
dayN/src/input.txt
```

To experiment with different inputs, edit or replace the corresponding `input.txt` file and rebuild/run the binary.

## Extending

To add more days in the same style:

1. Create a new directory `dayN` with a `Cargo.toml` (binary crate, edition 2021).
2. Add a `src/main.rs` implementing the solution and reading from `include_str!("input.txt")`.
3. Append `"dayN"` to the `members` array in the root `Cargo.toml`.

Cargo will then treat the new day as another workspace member you can run with `cargo run -p dayN`.
