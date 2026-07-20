# Contributing

Thanks for considering contributing to this project! This document covers
everything you need to get started.

## Project structure

This is a Cargo workspace split into several crates:

- `crates/core` — data model (entities, associations, cardinalities) and
  the undo/redo command engine. No GUI dependency.
- `crates/transform` — MCD → MLD → MPD transformation rules.
- `crates/sqlgen` — SQL generation per dialect.
- `crates/layout` — auto-layout algorithms.
- `crates/mcp` — MCP server exposing the engine to AI agents.
- `crates/gui` — Slint user interface. Contains no business logic.

**Rule of thumb:** if you're touching `core`, `transform`, `sqlgen`, or
`layout`, your change should never depend on `gui`. This separation is
enforced by the compiler, not just convention — keep it that way.

## Getting started

1. Install Rust via [rustup](https://rustup.rs) (stable toolchain).
2. Fork and clone the repo.
3. Build everything: `cargo build --workspace`
4. Run the tests: `cargo test --workspace`

On Linux, Slint needs a few system libraries:

sudo apt-get install libxkbcommon-dev libxcb1-dev libx11-dev libgl1-mesa-dev


## Before opening a pull request

- [ ] `cargo fmt --all` — code is formatted
- [ ] `cargo clippy --workspace -- -D warnings` — no lint warnings
- [ ] `cargo test --workspace` — all tests pass
- [ ] New behavior is covered by a test, especially for `core` and `transform`
      (Merise rules are the core value of this project — treat them with care)

## Commit convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

feat(core): add Entity and Attribute structs

fix(gui): correct association anchor recalculation

docs: update build instructions


## Developer Certificate of Origin (DCO)

We use a lightweight DCO instead of a CLA. Sign off each commit to certify
you have the right to submit your contribution:

git commit -s -m "your message"


## Good first issues

Look for issues labeled `good first issue` if you're new to the project.

## License

This project is licensed under GPLv3. By submitting a contribution, you
agree it will be licensed under the same terms.
