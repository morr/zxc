use crate::*;

pub struct RestablePlugin;

impl Plugin for RestablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Restable>()
            .add_systems(Update, progress_stamina.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Restable {
    pub stamina: f32,
}

impl Default for Restable {
    fn default() -> Self {
        Self { stamina: 100.0 }
    }
}

fn progress_stamina(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(&mut Restable, &Pawn)>,
) {
    let time_amount = time_scale.scale_to_seconds(time.delta_seconds());

    for (mut restable, pawn) in query.iter_mut() {
        let diff = match pawn.state {
            PawnState::Idle => time_amount * CONFIG.stamina_cost.idle,
            PawnState::Sleeping => time_amount * CONFIG.stamina_cost.sleeping,
            PawnState::Moving => time_amount * CONFIG.stamina_cost.moving,
            PawnState::Working(_) => time_amount * CONFIG.stamina_cost.working,
            PawnState::Dead | PawnState::WorkAssigned(_) => 0.0,
        };

        restable.stamina = (restable.stamina + diff).clamp(0.0, 100.0);
    }
}
