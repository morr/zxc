use rand_distr::num_traits::Zero;

use crate::*;

pub struct RestablePlugin;

impl Plugin for RestablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Restable>().add_systems(
            Update,
            progress_stamina
                .run_if(in_state(AppState::Playing))
                .run_if(in_state(SimulationState::Running)),
        );
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
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Restable, &mut Commandable, &Pawn)>,
    mut command_writer: EventWriter<ToRestCommand>,
    mut work_queue: ResMut<TasksQueue>,
) {
    let time_amount = time_scale.scale_to_seconds(time.delta_seconds());

    for (entity, mut restable, mut commandable, pawn) in query.iter_mut() {
        let was_non_zero = !restable.stamina.is_zero();

        let diff = match pawn.state {
            // PawnState::Idle => time_amount * CONFIG.stamina_cost.idle,
            // PawnState::Sleeping => time_amount * CONFIG.stamina_cost.sleeping,
            // PawnState::Moving => time_amount * CONFIG.stamina_cost.moving,
            // PawnState::Working(_) => time_amount * CONFIG.stamina_cost.working,
            // PawnState::Dead | PawnState::TaskAssigned(_) => 0.0,
            PawnState::Sleeping => time_amount * CONFIG.stamina_cost.sleeping,
            PawnState::Dead => 0.0,
            _ => time_amount * CONFIG.stamina_cost.living,
        };

        restable.stamina = (restable.stamina + diff).clamp(0.0, 100.0);

        if was_non_zero && restable.stamina.is_zero() {
            commandable.set_queue(
                CommandType::ToRest(ToRestCommand(entity)),
                entity,
                &mut commands,
                &mut work_queue,
            );
        }
    }
}
