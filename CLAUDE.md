# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ZXC is a 2D top-down simulation/strategy game built with **Bevy 0.18** (Rust, edition 2024). It uses Bevy's ECS (Entity-Component-System) architecture with a plugin-based modular design. Requires **nightly Rust toolchain**.

## Build & Development Commands

```bash
cargo build --verbose       # Build
cargo run                   # Run the game
cargo test --verbose        # Run all tests
cargo clippy -- -D warnings # Lint (CI enforced, warnings are errors)
```

Run a single test:
```bash
cargo test test_name --verbose
```

## Architecture

### Plugin System
Each feature is a Bevy plugin registered in `main.rs`. Plugins are split into two groups:
1. **Infrastructure**: daylight, camera, story_time, async_queue, assets, map, navigation, user_selection, structure, ui, input, tasks_queue
2. **Entity behavior**: ai, pawn, restable, feedable, movable, commandable, workable, carryable

### Module Convention
Modules follow a consistent pattern using macros defined in `lib.rs`:
- `use_modules!()` — declares and re-exports top-level modules
- `expose_submodules!()` — declares and re-exports submodules within a module
- Typical module structure: `mod.rs` (plugin), `components.rs`, `systems.rs`

### Core Entity: Pawn
Pawns are NPCs composed of multiple components: `Pawn`, `Movable`, `Commandable`, `Restable`, `Feedable`, `Carryable`. Each has its own state machine with tag components (e.g., `PawnStateIdleTag`, `MovableStateMovingTag`).

### Command System (`commandable/`)
Type-safe command queue with 12+ command types (move_to, work_on, feed, sleep, pick_up_item, etc.). Commands are processed sequentially per entity.

### Task System (`tasks_queue/` + `ai/`)
Central `TasksQueue` holds work requests. The AI system assigns tasks to pawns based on their current needs (hunger, fatigue) and available work.

### Navigation (`navigation/`)
A* pathfinding with navmesh, async pathfinding requests via `async_queue` to avoid frame stutter, and occupation tracking for dynamic obstacles.

### Configuration (`config.rs` + `resources/config.ron`)
RON-based global config loaded at startup. Accessed via `config()` (returns `&'static RootConfig`). Config has derived fields computed by `calculate_derived_fields()`. Covers: grid, time, pawn behavior, farming, movement costs, rest/food systems.

### Z-Index Layering (defined in `config.rs`)
`TILE_Z_INDEX(0) → PROP(5) → STRUCTURE(10) → PAWN(20) → ITEM(40) → NIGHT(100)`

### State Machines
State tag components are used for ECS query filtering. Guard macros enforce valid states:
- `ensure_state!(loop: ...)` / `ensure_state!(fn: ...)` — continue/return on state mismatch
- `continue_unless!` / `return_unless!` — silent state guards

### UI
Debug UI via `bevy_egui` (behind `debug_ui` feature flag, enabled by default).

## Verification After Each Task

After completing any task, run all these commands in parallel:

```bash
cargo build --verbose
cargo test --verbose
cargo clippy -- -D warnings
# fmt only the files you changed
RUSTFMT=~/.rustup/toolchains/nightly-aarch64-apple-darwin/bin/rustfmt cargo fmt -- src/changed_file.rs src/other_file.rs
```

## Code Conventions

- State tags use `*Tag` suffix; plugins use `*Plugin` suffix
- Event handlers use `on_*` prefix
- Logging macros: `log_state_change!`, `log_message!`, `log_event!`
- Clippy `type_complexity` lint is allowed globally
- Formatting: block indent style, reorder imports (`.rustfmt.toml`)

## Features

- `default = ["bevy_egui", "debug_ui"]`
- `bevy_egui` — enables egui integration
- `debug_ui` — enables debug UI panels

## Bevy Time Types

`Res<Time>` is a context-sensitive alias — use it for **delta accumulation and timers** in any schedule:
- In `Update` → resolves to `Time<Virtual>` (virtual/scaled delta)
- In `FixedUpdate` → resolves to `Time<Fixed>` (per-step virtual delta)

**Never use `Res<Time<Virtual>>.delta_secs()` for timer accumulation inside `FixedUpdate`.** `Time<Virtual>` is updated once per frame; when FixedUpdate runs multiple times per frame (at high time_scale), each run gets the full frame delta, causing timers to advance `time_scale`× too fast.

Use `Res<Time<Virtual>>` when you need **`elapsed_secs()`** — i.e., to read the current total virtual game time (e.g., `total_days(time.elapsed_secs())`). This is the right choice in observers and anywhere you need "what game time is it now?" rather than "how much time passed this step?".

Use `Res<Time<Real>>` only for things that must ignore pause and time_scale.

## Code Intelligence

An MCP language server (rust-analyzer) is available. Prefer it over grepping or reading files for:

- **Type information** — use `hover` on any symbol before assuming its type
- **Finding definitions** — use `definition` instead of searching for function/struct declarations
- **Finding usages** — use `references` instead of grep when you need to know where something is used
- **Diagnostics** — check `diagnostics` after making changes to confirm no errors were introduced
- **Renames** — use the LSP `rename` tool rather than find-and-replace

Only fall back to reading files directly when you need context that LSP doesn't provide (comments, logic flow, etc.).
