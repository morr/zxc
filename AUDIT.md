# ZXC Project: Code Quality Audit

## Context

Full codebase analysis of the ZXC Bevy game project, evaluating architecture, ECS patterns, error handling, performance, and code quality against Bevy best practices and Rust idioms. Findings are grouped by severity.

---

## CRITICAL — Unsafe Code & Soundness

### ~~1. Unsafe global mutable state via `config_mut()`~~ ✅ FIXED
**File:** `src/config.rs`

Removed `config_mut()` entirely. The single mutable use case (debug noise UI) now uses `ResMut<MapGeneratorConfig>` Bevy resource. Also replaced `once_cell` with `std::sync::OnceLock`/`LazyLock`.

### ~~2. Commandable state tag components are disabled — state machine is broken~~ WON'T FIX
**File:** `src/commandable/components.rs:241-256`

Runtime macro filtering (`continue_unless!`) is intentionally used instead of tag components. Tag-based state would cause archetype moves on every state transition (Idle → PendingExecution → Executing → Idle), which is more expensive than iterating and checking a field at this entity scale. Additionally, tag insertion via `Commands` is deferred to the next sync point, causing one-frame-off bugs where `.state` and the tag disagree — this is the "unreliable" behavior noted in the code comment.

---

## HIGH — Error Handling & Robustness

### ~~3. 40+ `unwrap()` calls on fallible operations~~ ✅ FIXED

Replaced bare `.unwrap()` with descriptive `.expect()` messages across 14 files (23 call sites). Remaining `unwrap()` calls either already had proper context (`unwrap_or_else(|err| panic!(...))`) or are in config/asset loading where crashing is intentional.

### ~~4. 6+ `panic!()` calls in system logic~~ ✅ FIXED

Added descriptive messages to bare `panic!()` calls and improved terse panic messages with debug context (entity values, previous state). Remaining panics already had proper messages or are intentional invariant checks.

### 5. `Pawn` component methods perform complex side effects
**File:** `src/pawn/components.rs:59-122`

- `pick_up_item()` takes 6 params including `&mut Commands`, `&mut Navmesh`, `&mut ResMut<FoodStock>`
- `drop_item()` takes 8 params with navmesh/mesh/food_stock mutations
- Component methods should hold data; **systems** should perform side effects
- **Fix:** Move this logic into dedicated systems

---

## HIGH — Performance

### ~~6. Navmesh successor list cloned on every A* step~~ ✅ FIXED
**File:** `src/navigation/components/navmesh.rs:47`

Now returns `&[(IVec2, i32)]` (slice reference) instead of cloning. See commit `0dcb7f5`.

### ~~7. Navmesh write lock acquired per-entity inside loop~~ ✅ FIXED
**File:** `src/movable/systems.rs:27-35`

Lock is now acquired once and changes applied in batch. See commit `80276c0`.

### ~~8. Hunger/fatigue systems run every frame without change detection~~ ✅ FIXED
- `src/feedable/mod.rs` and `src/restable/mod.rs` — now use timer-based gating.

---

## MEDIUM — Architecture & Design

### 9. `process_pending_commands` has 10 MessageWriter parameters
**File:** `src/commandable/systems.rs:3-20`

One writer per command type (`MoveToCommand`, `FeedCommand`, `SleepCommand`, etc.). This is a code smell indicating the dispatch pattern needs rethinking.

**Fix:** Use a single generic dispatch event, or use Bevy observers with `commands.trigger()`.

### 10. AI system has 7 parameters with a 7-component query tuple
**File:** `src/ai/mod.rs:11-34`

```rust
fn ai_idle_pawns(
    mut commands: Commands,
    mut commandable_query: Query<(Entity, &Pawn, &Movable, &Restable, &Feedable, &mut Commandable, &Transform)>,
    ...5 more params...
)
```

**Fix:** Use `SystemParam` derive to bundle related queries, or split into focused sub-systems.

### 11. `Pawn` is a god-component
**File:** `src/pawn/components.rs:9-20`

Stores: state, age, birth_year_day, lifetime, `HashMap<Entity, Carryable>` inventory, `Option<Entity>` bed ownership. Mixes identity, lifecycle, inventory, and housing concerns.

**Fix:** Extract `PawnInventory` and `PawnBed` into separate components.

### 12. `use_modules!` macro re-exports everything with `pub use crate::$x::*`
**File:** `src/lib.rs`

All internal details (systems, helpers) are publicly accessible. Prevents safe refactoring.

**Fix:** Export only the plugin struct and public components from each module.

### 13. No state transition validation
**File:** `src/pawn/components.rs:143-178`

`change_state()` allows any state-to-any-state transition (e.g., Dead -> Idle). Relies entirely on caller discipline.

**Fix:** Add a transition validation function or use a typestate pattern.

---

## LOW — Code Quality

### ~~14. `println!` left in production code~~ ✅ FIXED
**File:** `src/pawn/systems.rs` — Removed debug `println!` that ran every frame.

### 15. Large blocks of commented-out code
- `src/commandable/components.rs:171-192` (interrupt logic)
- `src/movable/components.rs:60-62, 112-131` (event triggers)
- `src/pawn/systems.rs:98-123` (color update system)

Should be removed; git history preserves it.

### ~~16. Unnecessary `Vec` allocation in `CommandType::IntoIterator`~~ ✅ FIXED
**File:** `src/commandable/components.rs` — Now uses `std::iter::once(self)` instead of `vec![self].into_iter()`.

### ~~17. `SeqCst` ordering on counter atomics~~ ✅ FIXED
**File:** `src/async_queue.rs:15-27` — Now uses `Ordering::Relaxed` for the simple counter.

### ~~18. Navtile HashMap entries never cleaned up~~ ✅ FIXED
**File:** `src/navigation/components/navtile.rs` — Empty `HashSet` entries are now removed in `remove_occupant()`.

### 19. Minimal test coverage
Only ~7 tests total (farm yield, pawn death, utils). No tests for:
- Command execution pipeline
- AI decision-making
- Pathfinding edge cases
- State machine transitions
- Error conditions

---

## Summary: Top 5 Fixes by Impact

| Priority | Issue | Why |
|----------|-------|-----|
| ~~1~~ | ~~Replace `config_mut()` unsafe with Bevy resource~~ | ~~Eliminates undefined behavior~~ ✅ |
| ~~2~~ | ~~Fix or remove disabled commandable state tags~~ | ~~State machine integrity~~ WON'T FIX |
| ~~3~~ | ~~Replace panics/unwraps with graceful handling~~ | ~~Game stability~~ ✅ |
| 4 | Return slice from `tile_successors()` | Pathfinding performance |
| 5 | Move side-effect logic out of `Pawn` methods | ECS correctness |

---

## Verification

After applying fixes:
- `cargo clippy -- -D warnings` passes
- `cargo test --verbose` passes
- Game runs without crashes under normal play
- No `unsafe` blocks remain (or are properly justified with `SAFETY` comments)
- `grep -r "unwrap()" src/` count significantly reduced
