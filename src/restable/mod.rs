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

const FATIGUE_FRESH: f32 = 0.;
const FATIGUE_OVERFLOW: f32 = 100.;

impl Default for Restable {
    fn default() -> Self {
        Self {
            fatigue: FATIGUE_FRESH,
            state: RestableState::Activity,
        }
    }
}

impl Restable {
    pub fn is_fresh(&self) -> bool {
        self.fatigue == FATIGUE_FRESH
    }

    pub fn is_overflowed(&self) -> bool {
        self.fatigue == FATIGUE_OVERFLOW
    }

    pub fn progress_fatigue(&mut self, time_amount: f32) {
        let amount = match self.state {
            RestableState::Activity => time_amount * config().restable.activity_cost,
            RestableState::Resting(sleep_quality_multiplier) => {
                config().restable.resting_cost * time_amount * sleep_quality_multiplier
            }
            RestableState::Dead => 0.0,
        };

        self.fatigue = (self.fatigue + amount).clamp(FATIGUE_FRESH, FATIGUE_OVERFLOW);
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
    mut commandable_release_resources_writer: EventWriter<ReleaseCommandResourcesEvent>,
    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
    mut event_writer: EventWriter<RestCompleteEvent>,
) {
    let time_amount = time_scale.scale_to_seconds(time.delta_secs());

    for (commandable_entity, mut restable, mut commandable) in query.iter_mut() {
        let wasnt_fresh = !restable.is_fresh();
        let wasnt_overflowed = !restable.is_overflowed();

        restable.progress_fatigue(time_amount);

        if wasnt_overflowed && restable.is_overflowed() {
            commandable.set_queue(
                CommandType::ToRest(ToRestCommand { commandable_entity }),
                commandable_entity,
                &mut commands,
                &mut commandable_interrupt_writer,
                &mut commandable_release_resources_writer,
            );
        }

        if wasnt_fresh && restable.is_fresh() {
            event_writer.send(log_event!(RestCompleteEvent { commandable_entity }));
        }
    }
}
