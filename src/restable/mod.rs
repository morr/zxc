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
    pub state: RestableState
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum RestableState {
    Active,
    Sleeping
}

const FULL_STAMINA: f32 = 100.;
const EMPTY_STAMINA: f32 = 0.;

impl Default for Restable {
    fn default() -> Self {
        Self {
            stamina: FULL_STAMINA,
            state: RestableState::Active
        }
    }
}

impl Restable {
    pub fn is_empty(&self) -> bool {
        self.stamina == EMPTY_STAMINA
    }

    pub fn is_full(&self) -> bool {
        self.stamina == FULL_STAMINA
    }

    pub fn change_stamina(&mut self, amount: f32) {
        self.stamina = (self.stamina + amount).clamp(EMPTY_STAMINA, FULL_STAMINA);
    }
}

fn progress_stamina(
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Restable, &mut Commandable, &mut Pawn)>,
    mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
    mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    let time_amount = time_scale.scale_to_seconds(time.delta_seconds());

    for (entity, mut restable, mut commandable, mut pawn) in query.iter_mut() {
        let wasnt_empty = !restable.is_empty();
        let wasnt_full = !restable.is_full();

        restable.change_stamina(match pawn.state {
            // PawnState::Idle => time_amount * config().stamina_cost.idle,
            // PawnState::Sleeping => time_amount * config().stamina_cost.sleeping,
            // PawnState::Moving => time_amount * config().stamina_cost.moving,
            // PawnState::Working(_) => time_amount * config().stamina_cost.working,
            // PawnState::Dead | PawnState::TaskAssigned(_) => 0.0,
            PawnState::Sleeping => time_amount * config().stamina_cost.sleeping,
            PawnState::Dead => 0.0,
            _ => time_amount * config().stamina_cost.living,
        });

        if wasnt_empty && restable.is_empty() {
            commandable.set_queue(
                CommandType::ToRest(ToRestCommand {
                    commandable_entity: entity,
                }),
                entity,
                &mut commands,
                &mut commandable_interrupt_writer,
                &mut tasks_scheduler,
            );
        }

        if wasnt_full && restable.is_full() {
            pawn.change_state(
                PawnState::Idle,
                entity,
                &mut commands,
                &mut pawn_state_change_event_writer,
            );
        }
    }
}
