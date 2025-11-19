# Advent of Code – Rust Workspace (Days 1–7)

This repository is a small Rust workspace containing solutions for the first seven days of an Advent of Code challenge.

Each day lives in its own binary crate (`day1` … `day7`). Inputs are stored alongside the code and embedded at compile time using `include_str!("input.txt")`, so you can run any day without providing input on stdin.

---

## Project layout

```text
.
├── Cargo.toml        # Workspace definition
├── day1              # Day 1 binary crate
├── day2              # Day 2 binary crate
├── day3              # Day 3 binary crate
├── day4              # Day 4 binary crate
├── day5              # Day 5 binary crate
├── day6              # Day 6 binary crate
└── day7              # Day 7 binary crate
```

Each `dayN` directory has the same basic structure:

```text
(dayN)/
├── Cargo.toml
└── src/
    ├── main.rs      # Solution entry point
    └── input.txt    # Puzzle input, embedded via `include_str!`
```

Some days only implement one of the puzzle parts; others contain helper functions (e.g. `part_1`, `part_2`) and call one of them from `main`.

---

## Prerequisites

- A recent stable Rust toolchain with Cargo (install via [rustup](https://rustup.rs/)).

No external dependencies are required beyond the Rust standard library.

---

## Running the solutions

From the repository root:

```bash
# Build everything
cargo build --workspace

# Run a particular day
cargo run -p day1
cargo run -p day2
cargo run -p day3
cargo run -p day4
cargo run -p day5
cargo run -p day6
cargo run -p day7
```

If you want faster runs, you can use release mode:

```bash
cargo run -p day5 --release
```

Most binaries print the answer(s) for that day to standard output. Some also print simple timing information using `std::time::Instant`.

---

## Changing the inputs

Each day reads from `src/input.txt` via `include_str!`, so the input is compiled into the binary. To run the code against a different input:

1. Open the appropriate `dayN/src/input.txt` file.
2. Replace its contents with your own puzzle input.
3. Rebuild/re-run the corresponding day, for example:
   ```bash
   cargo run -p day3
   ```

---

## Adding more days

To extend the workspace with additional days, you can follow the existing pattern:

1. Create a new directory `day8` (or the next day number).
2. Add a `Cargo.toml` for a binary crate named `"day8"`.
3. Add `src/main.rs` and (optionally) `src/input.txt`.
4. Register the crate in the root `Cargo.toml` under the `[workspace]` `members` array.

Cargo will then allow you to build and run the new day with:

```bash
cargo run -p day8
```
