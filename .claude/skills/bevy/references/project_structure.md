# ZXC project structure

## Layout

```
src/
├── main.rs            # App assembly: DefaultPlugins, states, plugin registration, close_on_esc
├── lib.rs             # `pub mod` list only — no macros, no re-exports
├── prelude.rs         # the ONLY legal glob import inside the crate
├── macros.rs          # ensure_state!, continue_unless!, return_unless!,
│                      # exclusive_state_tags!, log_state_change!, log_event!
├── config.rs          # RON config loading, `config()`, z-index constants
├── state.rs           # AppState (Loading → Playing)
├── events.rs          # cross-cutting events
├── ai.rs  assets.rs  async_queue.rs  camera.rs  daylight.rs  feedable.rs
├── input.rs  restable.rs  tasks_queue.rs        # single-file features
├── carryable/  commandable/  map/  movable/  navigation/  pawn/
├── story_time/  structure/  ui/  user_selection/  workable/   # multi-file features
tests/                 # integration tests: pawn.rs, farm.rs, bed.rs, story_time.rs, config.rs, utils.rs
resources/config.ron   # the config the game loads at startup
```

**One feature = one module = one plugin.** A feature lives in `src/<name>.rs`
until it needs splitting, then becomes `src/<name>/mod.rs` + submodules
(`components.rs`, `systems.rs`, or domain-named files). Single-file module
*directories* were deliberately flattened into `.rs` files — do not recreate
`foo/mod.rs` for a module with one file.

Do **not** introduce a global `components/` or `systems/` folder. Grouping is by
feature, never by ECS role.

## Plugin registration

`main.rs` registers ~30 plugins in two `.add_plugins((..))` tuples:

1. **Infrastructure** — daylight, camera, story_time, async_queue, assets, map,
   navigation, user_selection, structure, ui (`#[cfg(feature = "debug_ui")]`),
   input, tasks_queue
2. **Entity behaviour** — ai, pawn, restable, feedable, movable, commandable,
   workable, carryable

Plus `RemotePlugin` + `RemoteHttpPlugin` (BRP on 15702 — see the `live-app`
skill), `bevy_asset_loader`'s `LoadingState`, and `EguiPlugin`.

A plugin is a unit struct + `impl Plugin`:

```rust
pub struct RestablePlugin;

impl Plugin for RestablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, progress_fatigue.run_if(in_state(AppState::Playing)));
    }
}
```

Gameplay systems **and observers** are gated on `in_state(AppState::Playing)` —
0.19 lets observers take run conditions, and this repo uses that:

```rust
app.add_observer(on_pawn_death.run_if(in_state(AppState::Playing)))
   .add_observer(on_new_day_age_pawns.run_if(in_state(AppState::Playing)));
```

## Imports

`src/prelude.rs` is the only glob:

```rust
pub use bevy::prelude::*;
pub use crate::{config::{config, TILE_Z_INDEX, /* … z-index consts */}, state::AppState};
pub use crate::{continue_unless, ensure_state, exclusive_state_tags, log_event,
                log_state_change, return_unless};
```

Rule for adding to it: **only if ≥10 files need the symbol.** Domain types
(`Pawn`, `Movable`, `ArcNavmesh`, UI helpers) are imported by full path at each
use site. Clippy warns on `wildcard_imports`, so `use crate::pawn::*` is a lint
error, not a style preference.

Multi-file modules re-export their submodules explicitly in `mod.rs`:

```rust
mod bed;
mod carryable;
pub use self::{bed::*, carryable::*, /* … */};
```

## Config

```rust
config().restable.activity_cost_per_second   // &'static RootConfig
```

Loaded from `resources/config.ron` in `main()` before the `App` is built, and
inserted as a `RootConfig` resource too, so hot edits re-run
`on_root_config_changed` (`run_if(resource_changed::<RootConfig>)`). Derived
values are computed in `calculate_derived_fields()` — add new derived numbers
there rather than recomputing them in systems.

Z-index constants live in `config.rs`:
`TILE(0) → PROP(5) → STRUCTURE(10) → PAWN(20) → ITEM(40) → NIGHT(100)`.

## Logging

`LogPlugin` runs at `filter: "info,zxc=trace"`, so `trace!`/`debug!` from crate
code reaches stdout. `log_state_change!` and `log_event!` are the two macros that
make a run auditable from the log — the `live-app` skill greps exactly those
shapes to verify behaviour. Keep emitting them on every state transition and
every triggered event.

## Tests

Integration tests in `tests/` build a headless `App`, add only the plugins under
test, and step it manually:

```rust
mod apply_default_global_config;   // seeds config() outside main() — every test file starts with this

mod pawn {
    use bevy::time::{TimePlugin, TimeUpdateStrategy};
    use zxc::prelude::*;

    #[test]
    fn lifetime_loss_to_zero_leads_to_death_event() {
        let mut app = App::new();
        app.add_plugins(TimePlugin)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs(1)))
            .add_systems(Update, progress_pawn_dying);
        // spawn via app.world_mut().spawn(..), then app.update();
    }
}
```

Note the wrapping `mod <name> { .. }` inside each test file — match it.
`TimeUpdateStrategy::ManualDuration` is how simulated time is advanced
deterministically. Prefer a test here over booting the real game.
