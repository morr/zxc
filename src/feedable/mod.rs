use crate::*;

pub struct FeedablePlugin;

impl Plugin for FeedablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Feedable>().add_systems(
            Update,
            progress_saturation
                .run_if(in_state(AppState::Playing))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

const FULL_SATURATION: f32 = 100.;
const EMPTY_SATURATION: f32 = 0.;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Feedable {
    pub saturation: f32,
}

impl Default for Feedable {
    fn default() -> Self {
        Self {
            saturation: FULL_SATURATION,
        }
    }
}

impl Feedable {
    pub fn is_empty(&self) -> bool {
        self.saturation == EMPTY_SATURATION
    }

    pub fn is_full(&self) -> bool {
        self.saturation == FULL_SATURATION
    }

    pub fn progress_saturation(&mut self, time_amount: f32) {
        let amount = match self.state {
            FeedableState::Activity => time_amount * config().feedable.activity_cost,
            FeedableState::Resting(sleep_quality_multiplier) => {
                config().feedable.resting_cost * time_amount * sleep_quality_multiplier
            }
            FeedableState::Dead => 0.0,
        };

        self.saturation = (self.saturation + amount).clamp(EMPTY_SATURATION, FULL_SATURATION);
    }

    pub fn change_state(&mut self, new_state: FeedableState, entity: Entity) -> FeedableState {
        log_state_change!(
            "FeedableState({:?}).state {:?} => {:?}",
            entity,
            self.state,
            new_state
        );
        mem::replace(&mut self.state, new_state)
    }

    pub fn sleep_quality_multiplier(is_sleep_in_bed: bool) -> f32 {
        match is_sleep_in_bed {
            true => config().feedable.resting_on_bed_multiplier,
            false => config().feedable.resting_on_ground_multiplier,
        }
    }
}

fn progress_saturation(
    // mut commands: Commands,
    // time: Res<Time>,
    // time_scale: Res<TimeScale>,
    // mut query: Query<(Entity, &mut Feedable, &mut Commandable)>,
    // mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    // mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
    // // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
    // mut event_writer: EventWriter<RestCompleteEvent>,
) {
    // let time_amount = time_scale.scale_to_seconds(time.delta_seconds());
    //
    // for (commandable_entity, mut feedable, mut commandable) in query.iter_mut() {
    //     let wasnt_empty = !feedable.is_empty();
    //     let wasnt_full = !feedable.is_full();
    //
    //     feedable.progress_saturation(time_amount);
    //
    //     if wasnt_empty && feedable.is_empty() {
    //         commandable.set_queue(
    //             CommandType::ToRest(ToRestCommand { commandable_entity }),
    //             commandable_entity,
    //             &mut commands,
    //             &mut commandable_interrupt_writer,
    //             &mut tasks_scheduler,
    //         );
    //     }
    //
    //     if wasnt_full && feedable.is_full() {
    //         event_writer.send(log_event!(RestCompleteEvent { commandable_entity }));
    //     }
    // }
}
