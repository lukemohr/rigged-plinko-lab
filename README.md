# Rigged Plinko Lab

A Rust simulation playground for stochastic physical systems and simulation-based optimization.

The first lab is a Plinko-like system where we will eventually optimize peg layouts to produce target landing distributions.

## Project goals

- Build a reusable stochastic experiment framework.
- Simulate Plinko-style bouncing systems.
- Run many seeded trials reproducibly.
- Estimate landing distributions.
- Add optimization over board parameters.
- Eventually support visual playback and interactive controls.

## Workspace

- `sim_core`: reusable experiment traits and evaluation types.
- `plinko_core`: Plinko-specific simulation logic.
- `plinko_cli`: command-line runner for early experiments.

## Commands

```bash
cargo fmt
cargo check
cargo test
cargo clippy --all-targets --all-features
cargo run -p plinko_cli
```
