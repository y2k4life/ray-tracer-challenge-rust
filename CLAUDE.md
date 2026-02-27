# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust implementation of [The Ray Tracer Challenge by Jamis Buck](https://www.barnesandnoble.com/w/the-ray-tracer-challenge-jamis-buck/1127035142). Each chapter has its own self-contained Cargo project (`chapter_01` through `chapter_16`, plus `chapter_10a` and `chapter_10b`). **There is no workspace** — each chapter is independent. The final, complete implementation is in `chapter_16`.

## Commands

All commands must be run from within a chapter directory (e.g., `cd chapter_16`).

```bash
cargo build --all             # Build
cargo build --all --release   # Optimized build
cargo test --all              # Run all tests
cargo test <test_name>        # Run a single test by name
cargo clippy                  # Lint
cargo fmt                     # Format
cargo run --example chapter_16 --release  # Run the chapter's example
```

The `cargo xtask` command (defined in `xtask/`) runs operations across all chapters from the repo root:
```bash
cargo xtask test     # Test all chapters
cargo xtask clippy   # Clippy all chapters
cargo xtask fmt      # Format all chapters
cargo xtask build    # Build all chapters
cargo xtask release  # Release build all chapters
cargo xtask run      # Run all chapter examples (cumulative)
cargo xtask delete   # Delete generated .ppm files
cargo xtask all      # update → clean → fmt → clippy → build → test → run → clean
```

## Architecture

### Library (`rustic_ray`)

Each chapter's `src/` contains a library crate (`lib.rs`) plus binary examples in `examples/`. The public API is re-exported from `lib.rs`.

**Core types** (all implement `Copy`, `Clone`, `Debug`, `PartialEq`):
- `Point`, `Vector` — distinct structs (not tuples as in the book)
- `Color` — struct with `red`, `green`, `blue` fields
- `Matrix` — single struct for all matrix sizes (4x4 backing array)
- `Ray`, `Canvas`, `PointLight`, `Material`, `Intersection`, `Computations`
- `Transformation` — builder pattern for composing transforms
- `World`, `Camera` — scene graph and rendering

**Shapes** live in `src/shapes/` and implement the `Shape` trait (`dyn Shape`). Stored in `World` as `Vec<Box<dyn Shape>>`. Each shape implements `local_intersect()` and `local_normal_at()` in object space; the trait handles world-space transforms.

**Patterns** live in `src/patterns/` behind a `Pattern` trait, referenced from `Material`.

### Key Design Decisions

**Explicit types, not tuples.** `Point`, `Vector`, and `Color` are structs — this diverges from the book's tuple approach for type safety.

**Single `Matrix` struct.** No separate 2x2 or 3x3 types. 2x2 matrices use a 4x4 backing array with only the top-left populated. Fixed-size arrays (`[[f64; 4]; 4]`) are used instead of `Vec` for performance.

**Cached matrix inverse.** `Matrix` stores both `data` and `inverse` as `[[f64; 4]; 4]`. The inverse is computed once at construction (profiling with Valgrind/kcachegrind identified inversion as the primary hotspot). Calling `.inverse()` is zero-cost — it just swaps the two fields, returning a new `Matrix`.

**`Transformation` builder.** Accumulates transforms on a raw array and only constructs the expensive `Matrix` (with its cached inverse) on `.build()`. Use this instead of chaining `Matrix` operations.

**Floating point comparison.** `EPSILON = 0.0001`. Use `float_eq(a, b)` and `float_cmp(a, b)` everywhere instead of `==` on `f64`.

**Unique shape IDs.** Each shape gets a `u64` ID from `next_id()` in `lib.rs`, which uses an `AtomicU64` counter. No external dependencies.

**Shape boilerplate macro.** `impl_shape_common!()` in `src/shapes/` (the module entry point) expands the 6–8 identical getter/setter methods required by the `Shape` trait. All shape `impl Shape` blocks call it as their first line.

**Rust edition.** All crates use edition 2024.

### Tests

Tests are inline (`#[cfg(test)]`) in each module. Test names reference book chapter/page numbers. Every example from the book has a corresponding test. Run a specific test with:
```bash
cargo test -- float_eq  # runs tests matching "float_eq"
```

### Output

Examples render scenes and write PPM image files to the chapter directory. Delete them with `cargo xtask delete`.
