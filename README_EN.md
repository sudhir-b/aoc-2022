Advent of Code – Rust Workspace (Days 1–7)

Overview
- This repository is a Rust workspace containing seven independent binary crates: day1 through day7.
- Each crate solves a daily puzzle using only the Rust standard library (no external dependencies).
- Puzzle input data is embedded at compile time via include_str! from src/input.txt in each crate.

Prerequisites
- Rust (stable) and Cargo installed. Install them via https://rustup.rs

Workspace Organization
- Cargo.toml (workspace root)
- dayN/
  - Cargo.toml (crate manifest)
  - src/
    - main.rs (solution entry point)
    - input.txt (embedded input data)

Building
- Build all workspace crates (debug mode):
  - cargo build --workspace
- Build all crates in release mode (faster execution):
  - cargo build --workspace --release

Running
- From the workspace root, run a specific day (replace N with 1–7):
  - cargo run -p dayN
  - Example: cargo run -p day3
- Run with optimizations:
  - cargo run -p dayN --release
- Alternatively, from a crate's directory:
  - cd dayN && cargo run

Run All Days
- Bash/Zsh:
  - for d in 1 2 3 4 5 6 7; do cargo run -q -p day$d --release; done
- PowerShell:
  - 1..7 | ForEach-Object { cargo run -q -p "day$_" --release }

Inputs
- Each crate reads its input from src/input.txt at compile time with include_str!.
- To try different input, modify the corresponding src/input.txt and rerun (recompilation is automatic on changes).

Performance Notes
- Some solutions display simple timing information using std::time::Instant.
- Use --release for representative timings.

Contribution / Development
- No external crates are used; solutions rely on standard library data structures and algorithms.
- Keep each day self-contained in its own crate to preserve workspace organization.

License
- No license file is provided. If you plan to publish or reuse the code, consider adding an appropriate LICENSE file.
