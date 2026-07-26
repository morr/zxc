# Common pitfalls — Bevy 0.19 and this repo

## 1. Writing the pre-0.17 event API

```rust
// ❌ does not exist in 0.19
#[derive(Event)] struct Foo;
app.add_event::<Foo>();
fn send(mut w: EventWriter<Foo>) { w.send(Foo); }
fn recv(mut r: EventReader<Foo>) { for e in r.read() {} }
fn obs(trigger: Trigger<Foo>) { let e = trigger.event(); }
```

```rust
// ✅
#[derive(Event, Debug)] struct FooEvent;
app.add_observer(on_foo.run_if(in_state(AppState::Playing)));
fn on_foo(event: On<FooEvent>) { /* On<E> derefs to E */ }
commands.trigger(log_event!(FooEvent));
```

Symptoms of the old API: `no method named send`, `EventWriter not found`,
`Trigger not found`, `no method named event`.

## 2. Reaching for `On::target()`

There is none. `EntityEvent` carries its target in a field — read `event.entity`,
`event.commandable_entity`, or whatever the struct calls it. If the struct is a
tuple (`EntityStateChangeEvent(Entity, T)`), read `event.0`.

## 3. `get_single()` / assuming `single()` panics

`single()` and `single_mut()` return `Result` in 0.19:

```rust
let Ok(window) = windows.single() else { return };
```

## 4. `#[derive(Component, Resource)]`

Illegal in 0.19 — `Resource` already implies `Component`. Split into two types.
Related: a very broad query (`Query<EntityMut>`) now conflicts with `Res<T>`
params; add `Without<IsResource>`.

## 5. Inserting a state tag directly

```rust
// ❌ desyncs the enum from the tag, skips the log line and the state-change event
commands.entity(entity).insert(pawn_state::PawnStateDeadTag);

// ✅
pawn.change_state(PawnState::Dead, entity, &mut commands);
```

The tags are `immutable` and remove their siblings via an `on_add` hook, so a
manual insert *looks* like it works — the damage is the missing
`EntityStateChangeEvent` and the missing `log_state_change!` line, which is what
UI panels and the `live-app` verification loop both rely on.

## 6. `Time<Virtual>` deltas inside `FixedUpdate`

```rust
// ❌ advances time_scale× too fast when FixedUpdate runs several times per frame
fn tick(time: Res<Time<Virtual>>) { acc += time.delta_secs(); }

// ✅
fn tick(time: Res<Time>) { acc += time.delta_secs(); }          // Time<Fixed> in FixedUpdate
fn now(time: Res<Time<Virtual>>) { total_days(time.elapsed_secs()); }  // elapsed only
```

Full rule in CLAUDE.md § Bevy Time Types.

## 7. Glob-importing a crate module

`use crate::pawn::*;` trips the `wildcard_imports` clippy lint (CI is
`-D warnings`). `crate::prelude::*` is the only permitted glob. Adding a symbol to
the prelude is justified only at ≥10 call sites.

## 8. Expecting BRP to read ZXC components

Most `zxc::` components do not derive `Reflect`, so `world.query` counts them but
returns `"components": {}`, and `world.get_components` errors "isn't reflectable".
`filter.with` / `without` still work, so state tags are countable. Verify
behaviour through the trace log instead. See the `live-app` skill.

## 9. Querying an entity after despawn in the same system

Commands are deferred, so within one system the entity is still there — but by
the time an observer runs, it may be gone. Always `let Ok(..) = query.get(entity)
else { return }`; never `.unwrap()` on a query result keyed by an entity that
another system might have despawned.

## 10. Adding `--features bevy/dynamic_linking`

Already enabled in `Cargo.toml`. Passing it again just invalidates the feature
resolution and triggers a full rebuild. Related: avoid `cargo clean`, and if
`cargo run` dies with `failed to build archive … libbevy_*.rlib`, re-run
`cargo build` — it's a known `dynamic_linking` artifact, not a code error.

## 11. Filtering inside the loop instead of the query

```rust
// ❌
for (pawn, tag) in query.iter() { if tag.is_none() { continue } }

// ✅
Query<&Pawn, (With<PawnStateIdleTag>, Without<DyingMarker>)>
```

Archetypal filtering skips whole tables; `Option<&T>` + `continue` does not.

## 12. Forgetting to register the plugin / observer

A new system or observer does nothing until it appears in the feature's
`impl Plugin::build`, and a new feature does nothing until its plugin is added in
`main.rs`. Silent no-op, green build — check registration first when new code
"doesn't run".
