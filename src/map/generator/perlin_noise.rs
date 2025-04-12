use super::*;

pub struct PerlinNoisePlugin;

impl Plugin for PerlinNoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PerlinNoiseConfig>();

        #[cfg(feature = "bevy_egui")]
        app.add_systems(Update, ui_system);
    }
}

#[derive(Resource)]
pub struct PerlinNoiseConfig {
    pub auto_generate: bool,
    pub seed: u32,
    pub frequency: f64,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Default for PerlinNoiseConfig {
    fn default() -> Self {
        Self {
            auto_generate: true,
            seed: rand::random(),
            // seed: 1655470700,
            frequency: 0.01,
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

#[cfg(feature = "bevy_egui")]
fn ui_system(
    mut egui_contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut generator_config: ResMut<PerlinNoiseConfig>,
    mut rebuild_map_event_writer: EventWriter<RebuildMapEvent>,
) {
    // let ctx = egui_contexts.ctx_mut();
    //
    // bevy_egui::egui::Window::new("Cellular Automata Settings").show(ctx, |ui| {
    //     ui.add(bevy_egui::egui::Checkbox::new(
    //         &mut generator_config.auto_generate,
    //         "Auto Generate",
    //     ));
    //     let iterations_slider = ui.add(
    //         bevy_egui::egui::Slider::new(&mut generator_config.iterations, 0..=10).text("Iterations"),
    //     );
    //     let smoothing_iterations_slider = ui.add(
    //         bevy_egui::egui::Slider::new(&mut generator_config.smoothing_iterations, 0..=10)
    //             .text("Smoothing Iterations"),
    //     );
    //     let initial_alive_probability_slider = ui.add(
    //         bevy_egui::egui::Slider::new(&mut generator_config.initial_alive_probability, 0..=100)
    //             .text("Initial Alive Probability"),
    //     );
    //
    //     let generate_new_seed_button = ui.button("Generate New Seed");
    //     if generate_new_seed_button.clicked() {
    //         generator_config.seed = Some(rand::random());
    //     }
    //
    //     let maybe_button = if generator_config.auto_generate {
    //         None
    //     } else {
    //         Some(ui.button("Generate"))
    //     };
    //
    //     let is_changed = iterations_slider.changed()
    //         || smoothing_iterations_slider.changed()
    //         || initial_alive_probability_slider.changed()
    //         || generate_new_seed_button.clicked();
    //
    //     if (maybe_button.is_some() && maybe_button.unwrap().clicked())
    //         || (generator_config.auto_generate && is_changed)
    //     {
    //         rebuild_map_event_writer.send(log_event!(RebuildMapEvent {
    //             generator_kind: GeneratorKind::CellularAutomata
    //         }));
    //     }
    // });
}
