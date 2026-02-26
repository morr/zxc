# ZXC Codebase Audit: Flaws & Bad Coding Habits

## Context

Full audit of the ZXC Bevy 0.18 game project to identify flaws, bad coding habits, and Bevy antipatterns. The project is a 2D top-down simulation with pawns, farming, pathfinding, and an AI task system.

**Verdict**: Solid architecture overall — clean plugin separation, good macro usage, well-structured command system. But there are several categories of real issues worth fixing.

---

## Previously Fixed

These issues were identified in earlier audits and have been resolved.

1. **Unsafe global mutable state via `config_mut()`** — Removed `config_mut()` entirely; debug noise UI now uses `ResMut<MapGeneratorConfig>`. Replaced `once_cell` with `std::sync::OnceLock`/`LazyLock`. (`src/config.rs`)
2. **40+ `unwrap()` calls on fallible operations** — Replaced bare `.unwrap()` with descriptive `.expect()` across 14 files (23 call sites). Remaining calls are in config/asset loading where crashing is intentional.
3. **6+ `panic!()` calls in system logic** — Added descriptive messages to bare `panic!()` calls and improved terse panic messages with debug context (entity values, previous state).
4. **Navmesh successor list cloned on every A\* step** — `tile_successors()` now returns `&[(IVec2, i32)]` instead of cloning. (`src/navigation/components/navmesh.rs`, commit `0dcb7f5`)
5. **Navmesh write lock acquired per-entity inside loop** — Lock now acquired once with batch application. (`src/movable/systems.rs`, commit `80276c0`)
6. **Hunger/fatigue systems ran every frame without change detection** — Now use timer-based gating. (`src/feedable/mod.rs`, `src/restable/mod.rs`)
7. **`println!` left in production code** — Removed debug `println!` that ran every frame. (`src/pawn/systems.rs`)
8. **Unnecessary `Vec` allocation in `CommandType::IntoIterator`** — Now uses `std::iter::once(self)`. (`src/commandable/components.rs`)
9. **`SeqCst` ordering on counter atomics** — Changed to `Ordering::Relaxed` for simple counter. (`src/async_queue.rs`)
10. **Navtile HashMap entries never cleaned up** — Empty `HashSet` entries are now removed in `remove_occupant()`. (`src/navigation/components/navtile.rs`)
11. **Floating-point equality comparisons** — Replaced `==` with `<=` for fresh checks and `>=` for overflow/death checks in `Feedable` and `Restable`. (`src/feedable/mod.rs`, `src/restable/mod.rs`)
12. **`be_fed()` can make hunger negative** — Clamped result to `HUNGER_FRESH` (0.0) after subtracting. (`src/feedable/mod.rs`)

---

## Previously Evaluated — WON'T FIX

### Commandable state tag components are disabled
**File:** `src/commandable/components.rs:231-275`

Runtime macro filtering (`continue_unless!`) is intentionally used instead of tag components. Tag-based state would cause archetype moves on every state transition (Idle -> PendingExecution -> Executing -> Idle), which is more expensive than iterating and checking a field at this entity scale. Additionally, tag insertion via `Commands` is deferred to the next sync point, causing one-frame-off bugs where `.state` and the tag disagree — this is the "unreliable" behavior noted in the code comment.

---

## Open Issues

### 1. Crash-prone code (panics in runtime paths)

These are `panic!()` / `unwrap()` calls in systems that run on live game data, where entities can legitimately disappear.

| Location | Code | Risk |
|---|---|---|
| `ai/mod.rs:61-65` | `panic!("Failed to get query result for workable_entity")` | Workable entity deleted between task pop and query |
| `workable/systems.rs:30-33` | `panic!("Workable completed but previous state was not BeingWorked")` | State desync after ensure_state! should already guard this — redundant panic |
| `carryable/systems.rs:25` | `panic!("Cannot spawn CarryableKind::InInventory")` | Should be compile-time unreachable, but panic is wrong tool — use `unreachable!()` or refactor enum |
| `movable/components.rs:96` | `navmesh_arc_clone.read().unwrap()` | RwLock poisoning in async task crashes the game |

**Fix approach**: Replace panics with `warn!()` + early return for entity queries. Use `unreachable!()` for truly impossible branches. Use `.expect()` with context for lock access.

---

### 2. Task loss — silent data disappearance

`ai/mod.rs:53-119`: When AI pops a task from `TasksQueue` but fails to build a command sequence (e.g., carryable entity no longer exists at line 88-113), the task is **silently dropped**. It was popped from the queue but never returned.

```rust
} else if let Some(task) = tasks_queue.get_task() {  // popped!
    let maybe_commands_sequence = match task.0 { ... };
    if let Some(commands_sequence) = maybe_commands_sequence {
        commandable.set_queue(commands_sequence, ...);
    }
    // task gone forever if maybe_commands_sequence is None
}
```

Same issue with `TaskKind::Work` — the `unwrap_or_else(panic!)` at line 61 means either the game crashes or the task is lost. There's no middle ground.

**Fix**: On failure, push the task back to the queue or log a warning and explicitly discard it.

---

### 3. System ordering — no explicit ordering between competing writers

Three systems all call `commandable.set_queue()` on the same entities in `Update` with no ordering:

1. `progress_hunger` (`feedable/mod.rs:84-89`) — queues Feed commands
2. `progress_fatigue` (`restable/mod.rs:106-111`) — queues ToRest commands
3. `ai_idle_pawns` (`ai/mod.rs:42-52`) — queues Feed/ToRest/Work commands

`set_queue` calls `drain_queue` first, which **drops all existing queued commands**. If hunger fires before AI in the same frame, AI will immediately overwrite the Feed command. Or vice versa.

**Fix**: Add explicit `.before()` / `.after()` ordering, or consolidate need-based command assignment into one system.

---

### 4. Atomic ordering inconsistency

`async_queue.rs:17-26`:
```rust
pub fn increment(&self) { self.0.fetch_add(1, Ordering::Relaxed); }
pub fn decrement(&self) { self.0.fetch_sub(1, Ordering::Relaxed); }
pub fn get(&self)        { self.0.load(Ordering::Relaxed); }
```

But the async task completion uses `Ordering::SeqCst` at line 50:
```rust
queue_counter_clone.fetch_sub(1, Ordering::SeqCst);
```

The `decrement()` method (Relaxed) is never called — the async closure uses direct `fetch_sub` with SeqCst. So the `decrement` method is dead code. But `increment` uses Relaxed while the paired decrement uses SeqCst — mixed orderings. Should be consistent (at minimum `AcqRel` pairs or all `Relaxed` if only used for debugging).

---

### 5. Recursive movement — potential stack overflow

`movable/systems.rs:112-121`:
```rust
if remaining_time > 0.0 {
    return move_to_target_location(entity, movable, transform, remaining_time, ...);
}
```

If a pawn is very fast and path segments are short, this recurses once per path segment in a single frame. With long paths (500x500 grid, A* paths can be hundreds of tiles), this could overflow the stack.

**Fix**: Convert to a loop.

---

### 6. Commented-out code accumulation

Large blocks of dead code throughout the codebase:

- `commandable/components.rs:170-218` — commented-out methods and drain_queue logic
- `movable/components.rs:112-131` — old sync pathfinding
- `workable/systems.rs:67-143` — entire old work system (~75 lines)
- `pawn/components.rs:60-62` — commented println
- `movable/systems.rs:73-80` — commented debug prints

This clutters the codebase and makes it harder to understand intent. If the code is no longer needed, delete it — git preserves history.

---

### 7. ~~Typo: `MovableStateMovinTag`~~ FIXED

Renamed `MovableStateMovinTag` → `MovableStateMovingTag` in `movable/components.rs` and `movable/systems.rs`.

---

### 8. ~~Typo: `PawnDeatEvent`~~ FIXED

Renamed `PawnDeatEvent` → `PawnDeathEvent` in `pawn/components.rs`, `pawn/systems.rs`, `movable/systems.rs`, `feedable/mod.rs`, and `tests/pawn.rs`.

---

### 9. God system — `ai_idle_pawns`

`ai/mod.rs:12-142`: This single 130-line function handles:
- Hunger-triggered feeding
- Fatigue-triggered resting
- Work task assignment (with complex command sequence building)
- Idle wandering with random pathfinding

It queries 7 components + 2 additional queries + 2 resources. It makes decisions about every aspect of pawn behavior in one place.

**Impact**: Hard to test individual behaviors, hard to extend with new behaviors, hard to reason about ordering.

---

### 10. Missing entity cleanup on despawn

When a pawn dies or is despawned:
- `on_pawn_death` in `movable/systems.rs` sets movable to idle but doesn't cancel pending `PathfindingTask` components
- Queued commands in `Commandable` are not drained (beds not unclaimed, tasks not returned)
- Navmesh occupancy for carried items in inventory is not updated
- `Pawn.inventory` items become orphaned entities

There's no centralized despawn handler that cleans up all related state.

---

### 11. `MovableState` derives `States` unnecessarily

`movable/components.rs:5`:
```rust
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, InspectorOptions, Reflect)]
pub enum MovableState {
```

`States` is for Bevy's state machine system (app states like `AppState::Loading`/`AppState::Playing`). Using it on a per-entity enum that's stored as a component field is incorrect — it's not registered as a Bevy State and the derive does nothing useful here. It should be removed.

---

### 12. `Pawn::pick_up_item` / `drop_item` — methods with too many params doing world mutation

`pawn/components.rs:59-122`: These methods take `&mut Commands`, `&mut Navmesh`, `&mut ResMut<FoodStock>`, etc. — they're effectively systems disguised as component methods. This pattern:
- Makes the component harder to test
- Breaks ECS convention (components are data, systems are behavior)
- Requires `#[allow(clippy::too_many_arguments)]`

**Better approach**: Move this logic into dedicated systems that react to pickup/drop events.

---

### 13. `process_pending_commands` has 10 MessageWriter parameters

**File:** `src/commandable/systems.rs:3-20`

One writer per command type (`MoveToCommand`, `FeedCommand`, `SleepCommand`, etc.). This is a code smell indicating the dispatch pattern needs rethinking.

**Fix:** Use a single generic dispatch event, or use Bevy observers with `commands.trigger()`.

---

### 14. `Pawn` is a god-component

**File:** `src/pawn/components.rs:9-20`

Stores: state, age, birth_year_day, lifetime, `HashMap<Entity, Carryable>` inventory, `Option<Entity>` bed ownership. Mixes identity, lifecycle, inventory, and housing concerns.

**Fix:** Extract `PawnInventory` and `PawnBed` into separate components.

---

### 15. `use_modules!` macro re-exports everything with `pub use crate::$x::*`

**File:** `src/lib.rs`

All internal details (systems, helpers) are publicly accessible. Prevents safe refactoring.

**Fix:** Export only the plugin struct and public components from each module.

---

### 16. No state transition validation

**File:** `src/pawn/components.rs:143-178`

`change_state()` allows any state-to-any-state transition (e.g., Dead -> Idle). Relies entirely on caller discipline.

**Fix:** Add a transition validation function or use a typestate pattern.

---

### 17. Minimal test coverage

Only ~7 tests total (farm yield, pawn death, utils). No tests for:
- Command execution pipeline
- AI decision-making
- Pathfinding edge cases
- State machine transitions
- Error conditions

---

## Summary — Priority Order

### Must Fix (crashes / data loss)
1. **Task loss** in AI when command sequence build fails (#2)
2. **Panics** in runtime paths that handle entity queries (#1)
3. **`be_fed()` negative hunger** (#5)

### Should Fix (correctness / performance)
4. **Float equality** — inconsistent comparisons (#3)
5. **System ordering** — competing `set_queue` writers (#4)
6. **Recursive movement** — stack overflow risk (#7)
7. **Missing despawn cleanup** (#12)

### Should Clean Up (code quality)
8. **Commented-out code** — delete it (#8)
9. **Typos** — `MovinTag`, `DeatEvent` (#9, #10)
10. **Dead `States` derive** on `MovableState` (#13)
11. **Atomic ordering** inconsistency (#6)
12. **God system** `ai_idle_pawns` (#11)
13. **Component methods doing system work** (#14)
14. **10 MessageWriter params** in command dispatch (#15)
15. **`Pawn` god-component** — extract inventory/bed (#16)
16. **`use_modules!` over-exports** (#17)
17. **No state transition validation** (#18)
18. **Minimal test coverage** (#19)
