use crate::*;
use std::mem;

pub struct RestablePlugin;

impl Plugin for RestablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Restable>()
            .add_event::<RestCompleteEvent>()
            .add_systems(
                Update,
                progress_fatigue
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Restable {
    pub fatigue: f32,
    pub state: RestableState,
}

#[derive(Event, Debug)]
pub struct RestCompleteEvent {
    pub commandable_entity: Entity,
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum RestableState {
    Activity,
    Resting(f32),
    Dead,
}

const FULL_FATIGUE: f32 = 100.;
const EMPTY_FATIGUE: f32 = 0.;

impl Default for Restable {
    fn default() -> Self {
        Self {
            fatigue: FULL_FATIGUE,
            state: RestableState::Activity,
        }
    }
}

impl Restable {
    pub fn is_empty(&self) -> bool {
        self.fatigue == EMPTY_FATIGUE
    }

    pub fn is_full(&self) -> bool {
        self.fatigue == FULL_FATIGUE
    }

    pub fn progress_fatigue(&mut self, time_amount: f32) {
        let amount = match self.state {
            RestableState::Activity => time_amount * config().restable.activity_cost,
            RestableState::Resting(sleep_quality_multiplier) => {
                config().restable.resting_cost * time_amount * sleep_quality_multiplier
            }
            RestableState::Dead => 0.0,
        };

        self.fatigue = (self.fatigue + amount).clamp(EMPTY_FATIGUE, FULL_FATIGUE);
    }

    pub fn change_state(&mut self, new_state: RestableState, entity: Entity) -> RestableState {
        log_state_change!(
            "RestableState({:?}).state {:?} => {:?}",
            entity,
            self.state,
            new_state
        );
        mem::replace(&mut self.state, new_state)
    }

    pub fn sleep_quality_multiplier(is_sleep_in_bed: bool) -> f32 {
        match is_sleep_in_bed {
            true => config().restable.resting_on_bed_multiplier,
            false => config().restable.resting_on_ground_multiplier,
        }
    }
}

fn progress_fatigue(
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Restable, &mut Commandable)>,
    mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
    mut event_writer: EventWriter<RestCompleteEvent>,
) {
    let time_amount = time_scale.scale_to_seconds(time.delta_seconds());

    for (commandable_entity, mut restable, mut commandable) in query.iter_mut() {
        let wasnt_empty = !restable.is_empty();
        let wasnt_full = !restable.is_full();

        restable.progress_fatigue(time_amount);

        if wasnt_empty && restable.is_empty() {
            commandable.set_queue(
                CommandType::ToRest(ToRestCommand { commandable_entity }),
                commandable_entity,
                &mut commands,
                &mut commandable_interrupt_writer,
                &mut tasks_scheduler,
            );
        }

        if wasnt_full && restable.is_full() {
            event_writer.send(log_event!(RestCompleteEvent { commandable_entity }));
        }
    }
}
