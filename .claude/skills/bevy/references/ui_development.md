# UI in ZXC

Two separate stacks, don't mix them up:

- **Gameplay UI** — plain `bevy_ui` (`Node` / `Text`), one plugin per panel under
  `src/ui/`. This is what the player sees.
- **Debug UI** — `bevy_egui` + `bevy-inspector-egui`, under `src/ui/debug/`,
  behind `#[cfg(feature = "debug_ui")]`. Immediate-mode, no entities.

`UiPlugin` (`src/ui/mod.rs`) aggregates the panel plugins: `UiSimulationStatePlugin`,
`UiItemsStockPlugin`, `UiPawnPlugin`, `UiFarmPlugin`, `UiBedPlugin`,
`UiCarryablePlugin`, `UiStoragePlugin`, `UiHoveredPlugin`, `UiUserSelectedPlugin`,
`UiDebugPlugin`. `UiPlugin` itself is registered in `main.rs` under the
`debug_ui` feature.

## Shared style vocabulary

```rust
pub static UI_COLOR: LazyLock<Color> = /* hex "181a1c" */;
pub static UI_SCREEN_EDGE_PX_OFFSET: Val = Val::Px(8.);
pub static UI_WINDOWS_GAP: Val = Val::Px(25.);

pub enum UiOpacity { Light, Medium, Heavy }   // 0.25 / 0.65 / 0.85
pub fn ui_color(opacity: UiOpacity) -> Color;
```

Use these instead of literal colors or spacings. Panels are built from the bundle
helpers in `src/ui/mod.rs` rather than hand-written `Node`s:

- `render_entity_node_bunlde::<T>(target_id, opacity)` — root panel bound to an entity
- `render_node_bunlde::<T>(opacity)` — root panel with no entity target
- `render_entity_component_node_bunlde::<T>()` — a column section inside a panel
- `headline_text_bundle(text, &font_assets)` — 18 px title
- `property_text_bundle::<T>(text, &font_assets)` — 16 px labelled value

(The `bunlde` typo is the existing spelling — match it, don't half-rename.)

## 0.19 node / text API

```rust
Node {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    row_gap: px(10.),
    padding: UiRect { top: px(10.), right: px(10.), bottom: px(10.), left: px(10.) },
    width: px(300.),
    ..default()
},
BackgroundColor(ui_color(UiOpacity::Medium)),

Text(text),
TextFont {
    font: font_assets.fira.clone().into(),   // FontSource — `.into()` from Handle<Font>
    font_size: FontSize::Px(16.),            // NOT a bare f32
    ..default()
},
TextColor(Color::WHITE),
```

- `px(..)` / `percent(..)` are free functions; `Val::Px(..)` still works and is
  used for the `static` constants.
- No `TextBundle` / `NodeBundle` — components are spawned as tuples.
- Fonts come from `FontAssets` (bevy_asset_loader), not `AssetServer::load` at
  spawn time.

## Entity-bound panel pattern

Each panel has a marker component carrying the entity it describes, so update
systems can find "the UI for this pawn":

```rust
#[derive(Component)]
pub struct PawnUIMarker { pawn_id: Entity }

impl TargetEntityUiMarker for PawnUIMarker {
    fn new(pawn_id: Entity) -> Self { Self { pawn_id } }
}

// per-field markers for the text nodes that need refreshing
#[derive(Component, Default)] struct PawnAgeTextUIMarker {}
#[derive(Component, Default)] struct PawnStateTextUIMarker {}
```

Spawn = build the tree once with the bundle helpers; update = query
`Query<&mut Text, With<PawnAgeTextUIMarker>>` and overwrite the string. Don't
despawn and respawn a panel to refresh a value.

Panels react to `EntityStateChangeEvent` / domain events via observers rather
than polling every frame.

## Debug UI

`src/ui/debug/` — `info.rs`, `toggles.rs`, `tasks_queue.rs`, `grid.rs`,
`navmesh.rs`, `movepath.rs`, `noise/`. egui panels read resources directly and
draw immediately; toggles flip resource flags that gizmo-drawing systems check.
Everything here stays behind `#[cfg(feature = "debug_ui")]` so a
`--no-default-features` build still links.

`bevy-inspector-egui` shows components that derive `Reflect` + `InspectorOptions`
with `#[reflect(InspectorOptions)]`. A component without those derives is
invisible in the inspector — and also unreadable over BRP (see `live-app`).
