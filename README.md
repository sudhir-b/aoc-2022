# Advent of Code 2022 (Days 1–7)

This repository is a Rust workspace that gathers solutions for the first seven puzzles from [Advent of Code 2022](https://adventofcode.com/2022). Each day lives in its own binary crate so the code stays isolated, easy to run, and simple to extend when tackling additional days.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- `cargo` (installed alongside Rust)

No external dependencies are required; every solution relies solely on the Rust standard library.

## Workspace layout

| Crate | Puzzle | Highlights |
| ----- | ------ | ---------- |
| `day1` | Calorie Counting | Aggregates grouped calorie totals and reports the sum of the top three elves. |
| `day2` | Rock Paper Scissors | Parses opponent moves and desired outcomes to compute the overall score table-driven style. |
| `day3` | Rucksack Reorganization | Uses bitset-style tracking across groups of three rucksacks to accumulate priority scores. |
| `day4` | Camp Cleanup | Parses range assignments to detect both full containment (part 1) and overlaps (part 2). |
| `day5` | Supply Stacks | Simulates crate movements with two crane behaviors to compute the message spelled by the top crates. |
| `day6` | Tuning Trouble | Slides a window over the datastream using a `HashSet` to detect packet and message markers. |
| `day7` | No Space Left On Device | Builds an in-memory directory tree with `Rc<RefCell<Node>>` to measure directory sizes and storage needs. |

Each crate embeds its puzzle input via `include_str!("input.txt")`, so the binaries can be run without additional file management.

## Running a solution

From the workspace root:

```bash
# Debug build
cargo run -p day5

# Optimized build (useful for timing-sensitive puzzles)
cargo run --release -p day5
```

Replace `day5` with any other crate (`day1` … `day7`). Some crates expose `part_1` and `part_2` helper functions and call the desired part from `main`. If you want to switch which part runs, edit the corresponding `main.rs` and invoke the other helper.

## Working with inputs

Puzzle inputs live alongside their binaries in `dayN/src/input.txt`. Because they are pulled in at compile time, updating the input requires a rebuild (`cargo run -p dayN` will automatically recompile when the file changes).

## Development tips

- Format code before committing: `cargo fmt --all`
- Lint for common mistakes: `cargo clippy --all-targets --all-features`
- (Optional) execute all binaries to ensure nothing regressed: `cargo run -p dayN`

## Adding more days

1. Create a new binary crate: `cargo new day8 --bin`
2. Add `"day8"` to the workspace members list in the root `Cargo.toml`.
3. Drop the new puzzle input into `day8/src/input.txt` and build your solution.

Happy puzzling!
