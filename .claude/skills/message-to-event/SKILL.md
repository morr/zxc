---
name: message-to-event
description: Use this skill when converting a Bevy Message to an Event+Observer in this project, or when adding a new event and choosing between Event and EntityEvent. Covers the full migration pattern including derive changes, sender/receiver rewrites, plugin registration, and logging macro updates.
---

# Migrate Bevy Message → Event + Observer

## Overview

This project is migrating from Bevy's `Message` pub/sub system (`MessageWriter`/`MessageReader`) to the `Event` + Observer pattern (`commands.trigger()` / `On<T>`). Follow this checklist for each migration.

## Event vs EntityEvent

The project uses both derives (migration to `EntityEvent` completed for all entity-targeted events):

- **`#[derive(EntityEvent)]`** — the event is *about a specific entity* (commands on a pawn, death, work/rest completion, farm progress). The derive needs a target field: either a field literally named `entity`, the only field of a tuple struct, or any field marked `#[event_target]`. Unit structs are rejected.
- **`#[derive(Event)]`** — global happenings with no single target entity (`NewDayEvent`, `RebuildMapEvent`, `ScheduleTaskEvent`, input/hover events carrying coordinates, `InternalCommandInterruptEvent`/`ReleaseCommandResourcesEvent` which carry a whole `CommandType`).

```rust
#[derive(EntityEvent, Debug)]
pub struct RestCompleteEvent { pub entity: Entity }          // field named `entity`

#[derive(EntityEvent, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct FeedCommand {
    #[event_target]                                          // explicit target marker
    pub commandable_entity: Entity,
}

#[derive(EntityEvent, Debug)]
pub struct EntityStateChangeEvent<T>(#[event_target] pub Entity, pub T); // tuple struct
```

Notes:
- `commands.trigger(...)` and global `app.add_observer(...)` work identically for both; observers read the target through the event's own field (`event.entity`, `event.commandable_entity`) — `On<E>` derefs to `E`. There is no `On::target()`.
- `EntityEvent` additionally enables entity-scoped observers (`commands.entity(e).observe(...)`) and propagation (`#[entity_event(propagate)]`).
- Convention for multi-entity events: pick the entity the event is *about* as target (e.g. `WorkCompleteEvent` targets `workable_entity`, commands target `commandable_entity`, `MergeCarryablesEvent` targets `entity_to_merge`).

## Step-by-step

### 1. Rename and re-derive the struct

```rust
// BEFORE
#[derive(Message, Debug)]
pub struct FooMessage { pub entity: Entity }

// AFTER
#[derive(Event, Debug)]
pub struct FooEvent { pub entity: Entity }
```

- Rename `*Message` → `*Event`
- Change `Message` derive → `Event` derive

### 2. Convert the handler to an observer

```rust
// BEFORE — system with MessageReader
fn process_foo(
    mut reader: MessageReader<FooMessage>,
    mut res: ResMut<MyResource>,
) {
    for FooMessage { entity } in reader.read() {
        // handle each message
    }
}

// AFTER — observer with On<T>, no loop
fn on_foo(
    event: On<FooEvent>,
    mut res: ResMut<MyResource>,
) {
    let FooEvent { entity } = *event;
    // handle single event
}
```

Key changes:
- First parameter becomes `event: On<FooEvent>`
- Remove the `for ... in reader.read()` loop — observers fire once per event
- Rename function: `process_foo` → `on_foo` (use `on_` prefix)
- `continue` → `return`, `ensure_state!(loop:` → `ensure_state!(fn:`

### 3. Update plugin registration

```rust
// BEFORE
app.add_message::<FooMessage>()
    .add_systems(FixedUpdate, process_foo.run_if(in_state(AppState::Playing)));

// AFTER
app.add_observer(on_foo);
```

- Remove `.add_message::<T>()`
- Remove the system from `.add_systems()`
- Add `.add_observer(on_foo)`

### 4. Update all senders

```rust
// BEFORE
fn some_system(mut writer: MessageWriter<FooMessage>) {
    writer.write(FooMessage { entity });
    // or with log_message!
    writer.write(log_message!(FooMessage { entity }));
}

// AFTER
fn some_system(mut commands: Commands) {
    commands.trigger(log_event!(FooEvent { entity }));
}
```

Key changes:
- Replace `MessageWriter<FooMessage>` parameter with `Commands` (or use existing `commands` if available)
- `.write(...)` → `commands.trigger(log_event!(...))`
- `log_message!` → `log_event!`
- If the function already has `mut commands: Commands`, just remove the `MessageWriter` parameter

### 5. Find all usage sites

Use LSP references or grep to find every file that uses the old name:
- `MessageWriter<FooMessage>` — senders
- `MessageReader<FooMessage>` — receivers
- `add_message::<FooMessage>` — registration
- Any direct construction `FooMessage { ... }` or `FooMessage(...)`

### 6. Verify

```bash
cargo build --verbose
cargo test --verbose
cargo clippy -- -D warnings
```
