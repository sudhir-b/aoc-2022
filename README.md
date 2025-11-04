# Advent of Code 2022 — Rust (Days 1–7)

This repository is a Rust Cargo workspace containing seven independent binary crates, each solving an Advent of Code 2022 puzzle (days 1–7). Every day lives in its own crate and compiles its puzzle input directly into the binary via `include_str!`, so you can build and run without any external files at runtime.

- Language: Rust (stable)
- Layout: Cargo workspace with members `day1` … `day7`
- Dependencies: Standard library only

## Repository structure

```
.
├── Cargo.toml            # Workspace manifest
├── day1 … day7/          # One binary crate per day
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs       # Solution entrypoint
│       └── input.txt     # Puzzle input embedded at compile time
└── target/               # Build artifacts (gitignored)
```

## What’s implemented

- Day 1 — Calorie Counting: aggregate per-elf totals; sum of top three.
- Day 2 — Rock Paper Scissors: outcome-driven scoring using enums.
- Day 3 — Rucksack Reorganization: priority calculation across groups using bitset-like arrays.
- Day 4 — Camp Cleanup: range containment and overlap checks.
- Day 5 — Supply Stacks: simulate crate movements between stacks.
- Day 6 — Tuning Trouble: sliding window uniqueness search with a `HashSet`.
- Day 7 — No Space Left On Device: directory tree with `Rc<RefCell<Node>>` and size computations.

Some solutions log execution time using `std::time::Instant`.

## Prerequisites

- Rust toolchain via rustup (https://rustup.rs)
- A recent stable compiler is sufficient

Verify your toolchain:

```
rustc --version
cargo --version
```

## Build

Build the entire workspace:

```
cargo build
```

For optimized builds (recommended for timing):

```
cargo build --release
```

## Run

Run a specific day:

```
# Debug build
cargo run -p day1

# Release build
cargo run -p day1 --release
```

Replace `day1` with any of `day2` … `day7`.

Run all days (bash):

```
for d in 1 2 3 4 5 6 7; do
  echo "=== Day $d (release) ==="
  cargo run -p day$d --release
done
```

## Inputs

Each crate embeds its input at compile time from `src/input.txt` using `include_str!`. To try your own input:

- Edit the corresponding `src/input.txt` file for the day
- Rebuild and run that crate

No runtime file paths are needed.

## Extending (adding more days)

- Create a new binary crate and add it to the workspace:

```
cargo new day8 --bin
```

- Add `"day8"` to the `members` array in the top-level `Cargo.toml`
- Put your puzzle input in `day8/src/input.txt`
- Implement your solution in `day8/src/main.rs`

## Acknowledgments

- Advent of Code by Eric Wastl — https://adventofcode.com
