---
name: bevy
description: Use when writing or debugging Bevy code in this repo — components, systems, observers/events, queries, state tags, UI nodes, plugin wiring. Covers Bevy 0.19 API facts that pre-0.19 training data gets wrong, and this project's ECS conventions.
---

# Bevy 0.19 in this project

This repo pins **Bevy 0.19** (`Cargo.toml`), edition 2024, nightly toolchain.
Most model priors come from 0.12–0.16 and are wrong here. Check an API against
ground truth before writing it.

## Ground truth, in order of cost

1. **Local registry examples** — authoritative, compile against the exact version:
   ```
   ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy-0.19.0/examples/
   ```
   `examples/ecs/observers.rs`, `ecs/message.rs`, `ecs/observer_propagation.rs`,
   `ecs/relationships.rs`, `ui/*` answer most questions faster than the web.
   `bevy-0.18.0` is also unpacked — diff the two to see what a release changed.
2. **This repo** — `rg` for an existing usage. If ZXC already does it, copy that shape.
3. **Migration guides** — `https://bevy.org/learn/migration-guides/0-18-to-0-19/`
   (swap the version numbers in the URL for older hops; release notes live at
   `https://bevy.org/news/bevy-0-19/`). Use when you need *why* something moved.

Never cite an API from memory when the answer is one `rg` away in the registry.

## 0.19 facts that get written wrong

**Events are three different things.** Pick deliberately — details and this
project's convention live in the `message-to-event` skill.

| Kind | Derive | Send | Receive | Register |
|---|---|---|---|---|
| Global event | `#[derive(Event)]` | `commands.trigger(e)` | `fn on_x(ev: On<E>)` | `app.add_observer(on_x)` |
| Entity-targeted event | `#[derive(EntityEvent)]` | `commands.trigger(e)` | `On<E>`, or `commands.entity(id).observe(..)` | `add_observer` / per-entity |
| Buffered message | `#[derive(Message)]` | `MessageWriter::write` | `MessageReader::read` | `app.add_message::<M>()` |

- The observer param is **`On<E>`**, not `Trigger<E>`. `On<E>` derefs to `E`;
  read the target through the event's own field (`ev.entity`), **there is no
  `On::target()`**.
- `EntityEvent` needs a target field: named `entity`, the only field of a tuple
  struct, or `#[event_target]`. Unit structs are rejected.
- Lifecycle observers take two params: `On<Add, Mine>`, `On<Remove, Mine>`.
- Propagation is opt-in: `#[entity_event(propagate, auto_propagate)]`, toggled
  per-trigger with `ev.propagate(false)`.
- `EventWriter` / `EventReader` / `add_event` / `.send()` **do not exist**.
- Observers support run conditions in 0.19.
- **ZXC defines no `Message` types.** Default to `Event` / `EntityEvent`. The one
  `MessageWriter` in the repo (`main.rs`, `AppExit`) is a Bevy builtin — leave it.

**Queries.** `single()` / `single_mut()` return `Result`:
```rust
let Ok(window) = windows.single() else { return };
```
`get_single()` is gone. Multi-entity mutation is `query.get_many_mut([a, b])`.

**Resources are Components now (0.19).** Consequences:
- `#[derive(Resource)]` already implements `Component` — `#[derive(Component, Resource)]`
  on one type is a compile error. Split the type.
- Broad queries collide with resource params: `Query<EntityMut>` + `Res<T>` now
  conflicts; filter with `Without<IsResource>`.
- Lifecycle observers and relationships work on resources.

**Text / UI.**
```rust
TextFont {
    font: font_assets.fira.clone().into(), // FontSource, not Handle<Font>
    font_size: FontSize::Px(16.),          // not a bare f32
    ..default()
}
```
- `Val` helpers: `px(10.)`, `percent(25.)` — used throughout `src/ui/`.
- `TextLayout::new_with_justify` → `TextLayout::justify` (same for `linebreak`, `no_wrap`).
- `ComputedNode::stack_index` moved to its own `ComputedStackIndex` component.
- `Node` gained `direction: InlineDirection`.

**Renames worth knowing.** `Scene` → `WorldAsset`, `SceneRoot` → `WorldAssetRoot`,
`bevy::scene` → `bevy::world_serialization`; `DefaultErrorHandler` → `FallbackErrorHandler`;
`schedule.set_executor_kind(..)` → `schedule.set_executor(MultiThreadedExecutor::new())`.

**Also commonly written wrong** (details in `references/api_0_19.md` § "APIs
pre-0.19 priors don't know"): poll a `Task` with
`bevy::tasks::futures::check_ready(&mut task)`, not `block_on(poll_once(..))`;
2D cursor→world is `camera.viewport_to_world_2d(..)`, not the ray-based 3D
variant; guard world input against UI with `Res<HoverMap>`; observer
`run_if` **drops** the event when false (no queue); app-level `States`
transitions are handled in `OnEnter`/`OnExit`, never mirrored with a custom
event.

For the full sweep see `references/api_0_19.md`.

## This project's conventions

Follow these over any generic Bevy advice — full detail in
`references/project_structure.md`.

- **Feature modules, not `components/` + `systems/` folders.** A feature is
  `src/<feature>.rs` when it fits one file, `src/<feature>/mod.rs` + submodules
  when it doesn't. `mod.rs` holds the plugin. Single-file module *directories*
  were deliberately flattened — don't reintroduce them.
- **Every feature is a plugin**, registered in `main.rs` (~30 of them, split
  infrastructure / entity-behaviour).
- **`crate::prelude::*` is the only legal glob import.** A symbol earns a place
  in it only when ≥10 files need it. Everything domain-level is imported by path.
  Clippy warns on `wildcard_imports`.
- **State machines are exclusive tag components.** Generate them with
  `exclusive_state_tags!(A, B, C)` — it emits `#[component(immutable, on_add = ..)]`
  tags that remove their siblings via a hook, so the invariant holds at the ECS
  level. Query with `With<Tag>` / `Without<Tag>`; guard with `ensure_state!(fn:/loop:)`,
  `return_unless!`, `continue_unless!`.
- **Logging macros:** `log_state_change!` (debug!) and `log_event!` (wraps an
  event value, logs it, returns it — use inline: `commands.trigger(log_event!(FooEvent { entity }))`).
- **Config** is RON, read via `config()` → `&'static RootConfig`. Z-index
  constants come from `config.rs` (`TILE(0) → PROP(5) → STRUCTURE(10) → PAWN(20) → ITEM(40) → NIGHT(100)`).
- **Time types**: see CLAUDE.md § Bevy Time Types. `Res<Time>` for deltas,
  `Res<Time<Virtual>>` only for `elapsed_secs()`, never `Time<Virtual>` deltas
  inside `FixedUpdate`.
- **Debug UI** is `bevy_egui` + `bevy-inspector-egui` behind the `debug_ui`
  feature; gameplay UI is plain `bevy_ui` nodes with marker components.

## Build workflow

`dynamic_linking` is **already on** in `Cargo.toml` — do not add
`--features bevy/dynamic_linking` to commands. Avoid `cargo clean`; a cold Bevy
rebuild costs minutes.

Verification after a change is defined in CLAUDE.md (build / test / clippy /
targeted fmt). `cargo check` is fine for a fast inner loop, but the CLAUDE.md
block is what must pass before calling a task done.

To verify something only observable at runtime, use the **`live-app` skill** —
don't hand-roll a `cargo run` + BRP session.

## References

- `references/api_0_19.md` — 0.19 breaking-change cheat sheet, grouped by area
- `references/project_structure.md` — real ZXC layout, plugin wiring, prelude rules
- `references/ecs_patterns.md` — the patterns this codebase actually uses
- `references/ui_development.md` — bevy_ui node/marker idiom + egui debug panels
- `references/common_pitfalls.md` — traps specific to 0.19 and to this repo
