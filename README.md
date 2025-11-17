# Advent of Code – Rust Solutions

This repository contains a collection of small Rust binaries solving Advent of Code style programming puzzles. The project is organised as a Cargo workspace with one crate per day (`day1`–`day7`).

Each crate is a standalone binary that:

- Embeds its puzzle input at compile time using `include_str!("input.txt")`.
- Uses only the Rust standard library (no external dependencies).
- Prints the answer(s) for that day to standard output when run.

## Repository layout

- `Cargo.toml` – Workspace definition listing all daily crates.
- `day1`–`day7` – Individual binary crates, one per day.
  - `Cargo.toml` – Package manifest for that day.
  - `src/main.rs` – Solution for the day.
  - `src/input.txt` (or `input.txt` next to `main.rs`) – Embedded puzzle input referenced via `include_str!`.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021; any recent stable toolchain should work).
- `cargo` available on your `PATH`.

## Building and running

From the repository root:

```bash
# Build all days
cargo build

# Or build with optimisations
cargo build --release
```

To run a specific day, use the package name (`day1`, `day2`, ..., `day7`):

```bash
# Run day 1
cargo run -p day1

# Run day 4
cargo run -p day4

# Run day 7 with release optimisations
cargo run -p day7 --release
```

Each binary reads its bundled input, computes the puzzle solution(s), and prints the result to stdout. Some days also print simple timing information using `std::time::Instant`.

## Modifying or adding puzzles

- To change the input for a day, edit the corresponding `input.txt` file referenced by `include_str!` in that day's `main.rs`.
- To add a new day:
  1. Create a new directory `dayN` (e.g. `day8`).
  2. Add a `Cargo.toml` declaring a binary crate named `dayN`.
  3. Add your `src/main.rs` implementation.
  4. Add `"dayN"` to the `members` array in the root `Cargo.toml`.

You can then build and run the new day with `cargo run -p dayN`.
