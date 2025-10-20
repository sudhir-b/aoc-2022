Advent of Code – Rust Workspace (Days 1–7)

Overview
- This repository is a Rust workspace containing seven independent binary crates: day1 through day7.
- Each crate solves one daily puzzle using only the Rust standard library (no external dependencies).
- Puzzle input is embedded at compile time via include_str! from src/input.txt inside each crate.

Requirements
- Rust (stable) and Cargo installed. Get them via https://rustup.rs

Workspace layout
- Cargo.toml (workspace root)
- dayN/
  - Cargo.toml (crate manifest)
  - src/
    - main.rs (solution entry point)
    - input.txt (embedded puzzle input)

Build
- Build all crates in the workspace (debug):
  - cargo build --workspace
- Build all crates in release mode (faster execution):
  - cargo build --workspace --release

Run
- From the workspace root, run an individual day (replace N with 1–7):
  - cargo run -p dayN
  - Example: cargo run -p day3
- Run with optimizations:
  - cargo run -p dayN --release
- Alternatively, from within a crate directory:
  - cd dayN && cargo run

Run all days
- Bash/Zsh:
  - for d in 1 2 3 4 5 6 7; do cargo run -q -p day$d --release; done
- PowerShell:
  - 1..7 | ForEach-Object { cargo run -q -p "day$_" --release }

Inputs
- Each crate reads its input from src/input.txt at compile-time with include_str!.
- To try different input, edit the corresponding src/input.txt and re-run (rebuild is automatic on change).

Performance notes
- Some solutions print simple timing information using std::time::Instant.
- Use --release for representative timings.

Contributing / Development
- No external crates are used; solutions rely on standard library data structures and algorithms.
- Keep each day self-contained under its own crate to preserve the workspace organization.

License
- No license file is included. If you plan to publish or reuse the code, consider adding an appropriate LICENSE file.
