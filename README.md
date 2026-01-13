# Advent of Code Practice Workspace

This repository contains a set of Rust solutions for daily puzzles, where each puzzle resides in its own crate under a day-specific directory. Each crate is a simple standalone binary project that can be built and run independently.

## Repository structure

```
day1/   # Binary crate for Day 1 puzzle
day2/   # Binary crate for Day 2 puzzle
...     # Additional days continue likewise
```

The workspace is managed via Cargo: the root `Cargo.toml` declares every `day*` crate as a member so they can share common tooling and dependencies when needed.

## Running the puzzles

Each day's crate can be run individually from the root of this repository using `cargo run -p dayN`, replacing `N` with the desired day number:

```bash
cargo run -p day1
```

This will build and execute the `main.rs` file for the given day, printing the result to standard output.

To run all crates sequentially (for example during development or testing), you can use the workspace profile:

```bash
cargo run --all
```

## Adding a new day

To add a new puzzle crate:

1. Create a new directory (`dayX`) with its own `Cargo.toml` and `src/main.rs`.
2. Update the root `Cargo.toml` `members` list to include the new directory.
3. Implement the puzzle solution and include any necessary input data files.

## Notes

- Input data for each day should be placed alongside `main.rs`, commonly in a file named `input.txt`.
- Each crate is intended to run independently and print the computed answer for its specific puzzle.

