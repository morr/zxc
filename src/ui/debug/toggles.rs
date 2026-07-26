use bevy::ecs::system::IntoObserverSystem;
use bevy::ui_widgets::{Activate, Button};

use crate::assets::FontAssets;
use crate::config::DebugNoiseState;
use crate::events::StateChangeEvent;
use crate::prelude::*;
use crate::ui::DebugGridState;
use crate::ui::DebugMovepathState;
use crate::ui::DebugNavmeshState;
use crate::ui::DebugUiContainerMarker;
use crate::ui::UiOpacity;
use crate::ui::ui_color;
/// TODO 3.2 prototype: debug layer toggles built on the first-party headless
/// `bevy_ui_widgets::Button` instead of egui. Buttons emit `Activate` (an
/// `EntityEvent`), handled by per-entity `.observe()` observers.
#[derive(Component, Default)]
pub struct DebugTogglesUIMarker {}

/// Which debug layer a toggle button controls; drives the active highlight.
#[derive(Component, Clone, Copy)]
pub enum DebugToggleButton {
    Grid,
    Navmesh,
    Noise,
    Movepath,
}

const TOGGLE_ACTIVE_COLOR: Color = Color::srgba(0.16, 0.5, 0.2, 0.9);

pub fn render_debug_toggles(
    mut commands: Commands,
    root_ui_id: Single<Entity, With<DebugUiContainerMarker>>,
    font_assets: Res<FontAssets>,
) {
    let font = font_assets.fira.clone();

    let row = commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: px(6.),
                padding: UiRect::all(px(10.)),
                ..default()
            },
            BackgroundColor(ui_color(UiOpacity::Light)),
            DebugTogglesUIMarker::default(),
        ))
        .id();
    commands.entity(*root_ui_id).add_child(row);

    spawn_toggle(
        &mut commands,
        row,
        &font,
        "grid",
        DebugToggleButton::Grid,
        |_activate: On<Activate>,
         mut next: ResMut<NextState<DebugGridState>>,
         state: Res<State<DebugGridState>>| {
            next.set(match state.get() {
                DebugGridState::Visible => DebugGridState::Hidden,
                DebugGridState::Hidden => DebugGridState::Visible,
            });
        },
    );
    spawn_toggle(
        &mut commands,
        row,
        &font,
        "navmesh",
        DebugToggleButton::Navmesh,
        |_activate: On<Activate>,
         mut commands: Commands,
         mut next: ResMut<NextState<DebugNavmeshState>>,
         state: Res<State<DebugNavmeshState>>| {
            let new_state = match state.get() {
                DebugNavmeshState::Visible => DebugNavmeshState::Hidden,
                DebugNavmeshState::Hidden => DebugNavmeshState::Visible,
            };
            next.set(new_state.clone());
            commands.trigger(log_event!(StateChangeEvent(new_state)));
        },
    );
    spawn_toggle(
        &mut commands,
        row,
        &font,
        "noise",
        DebugToggleButton::Noise,
        |_activate: On<Activate>,
         mut commands: Commands,
         mut next: ResMut<NextState<DebugNoiseState>>,
         state: Res<State<DebugNoiseState>>| {
            let new_state = match state.get() {
                DebugNoiseState::Hidden => DebugNoiseState::HeightNoise,
                DebugNoiseState::HeightNoise => DebugNoiseState::HumidityNoise,
                DebugNoiseState::HumidityNoise => DebugNoiseState::PropsNoise,
                DebugNoiseState::PropsNoise => DebugNoiseState::Hidden,
            };
            next.set(new_state.clone());
            commands.trigger(log_event!(StateChangeEvent(new_state)));
        },
    );
    spawn_toggle(
        &mut commands,
        row,
        &font,
        "movepath",
        DebugToggleButton::Movepath,
        |_activate: On<Activate>,
         mut next: ResMut<NextState<DebugMovepathState>>,
         state: Res<State<DebugMovepathState>>| {
            next.set(match state.get() {
                DebugMovepathState::Visible => DebugMovepathState::Hidden,
                DebugMovepathState::Hidden => DebugMovepathState::Visible,
            });
        },
    );
}

fn spawn_toggle<M>(
    commands: &mut Commands,
    row: Entity,
    font: &Handle<Font>,
    label: &str,
    kind: DebugToggleButton,
    on_activate: impl IntoObserverSystem<Activate, (), M>,
) {
    let button = commands
        .spawn((
            Button,
            kind,
            Node {
                padding: UiRect {
                    top: px(4.),
                    right: px(8.),
                    bottom: px(4.),
                    left: px(8.),
                },
                ..default()
            },
            BackgroundColor(ui_color(UiOpacity::Heavy)),
            children![(
                Text::new(label),
                TextFont {
                    font: font.clone().into(),
                    font_size: FontSize::Px(12.),
                    ..default()
                },
                TextColor(Color::WHITE),
            )],
        ))
        .observe(on_activate)
        .id();
    commands.entity(row).add_child(button);
}

pub fn update_toggle_buttons(
    grid: Res<State<DebugGridState>>,
    navmesh: Res<State<DebugNavmeshState>>,
    noise: Res<State<DebugNoiseState>>,
    movepath: Res<State<DebugMovepathState>>,
    mut buttons: Query<(Entity, &DebugToggleButton, &mut BackgroundColor)>,
    children_query: Query<&Children>,
    mut writer: TextUiWriter,
) {
    for (entity, toggle, mut background) in buttons.iter_mut() {
        let is_active = match toggle {
            DebugToggleButton::Grid => *grid.get() == DebugGridState::Visible,
            DebugToggleButton::Navmesh => *navmesh.get() == DebugNavmeshState::Visible,
            DebugToggleButton::Noise => *noise.get() != DebugNoiseState::Hidden,
            DebugToggleButton::Movepath => *movepath.get() == DebugMovepathState::Visible,
        };
        *background = BackgroundColor(if is_active {
            TOGGLE_ACTIVE_COLOR
        } else {
            ui_color(UiOpacity::Heavy)
        });

        // the noise toggle cycles through layers, show the current one
        if matches!(toggle, DebugToggleButton::Noise)
            && let Ok(children) = children_query.get(entity)
            && let Some(text_entity) = children.iter().next()
        {
            *writer.text(text_entity, 0) = match noise.get() {
                DebugNoiseState::Hidden => "noise".into(),
                DebugNoiseState::HeightNoise => "noise: height".into(),
                DebugNoiseState::HumidityNoise => "noise: humidity".into(),
                DebugNoiseState::PropsNoise => "noise: props".into(),
            };
        }
    }
}
