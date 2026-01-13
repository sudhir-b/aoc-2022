# Advent of Code Solutions

This Rust workspace contains solutions for the first seven days of the Advent of Code challenge. Each day is implemented as its own binary crate, and the inputs for each puzzle are included inside the respective `src` directory (most commonly as `input.txt`).

## Workspace Layout

- `day1` through `day7` — individual crates, each implementing a distinct day's puzzle. Each crate follows the default Cargo layout (`src/main.rs` with any associated assets or inputs).

## Building & Running

From the workspace root, you can build or run the solution for any day by specifying its package:

```bash
cargo build -p dayX
cargo run -p dayX
```

Replace `dayX` with the crate name (`day1`, `day2`, ..., `day7`).

Most solutions read their inputs from a local file via `include_str!`, so there is no need to provide command-line arguments or file paths when running the binaries.

## Testing

Each day's crate can be tested using Cargo if tests are added to that crate. For example:

```bash
cargo test -p day3
```

Currently, there are no automated tests included, but you can add unit or integration tests to each crate following standard Rust testing practices.

