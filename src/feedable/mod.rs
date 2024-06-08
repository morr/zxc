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

const FULL_HUNGER: f32 = 100.;
const NO_HUNGER: f32 = 0.;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Feedable {
    pub hunger: f32,
}

impl Default for Feedable {
    fn default() -> Self {
        Self {
            hunger: FULL_HUNGER,
        }
    }
}

impl Feedable {
    pub fn is_empty(&self) -> bool {
        self.hunger == NO_HUNGER
    }

    pub fn is_full(&self) -> bool {
        self.hunger == FULL_HUNGER
    }

    pub fn progress_saturation(&mut self, time_amount: f32) {
        let amount = time_amount * config().feedable.hunger_cost;
        self.hunger = (self.hunger + amount).clamp(NO_HUNGER, FULL_HUNGER);
    }
}

fn progress_saturation(
    // mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Feedable)>,
    // mut query: Query<(Entity, &mut Feedable, &mut Commandable)>,
    // mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    // mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
    // // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
    // mut event_writer: EventWriter<RestCompleteEvent>,
) {
    let time_amount = time_scale.scale_to_seconds(time.delta_seconds());

    // for (commandable_entity, mut feedable, mut commandable) in query.iter_mut() {
    for (_commandable_entity, mut feedable) in query.iter_mut() {
        // let wasnt_empty = !feedable.is_empty();
        // let wasnt_full = !feedable.is_full();

        feedable.progress_saturation(time_amount);

        // if wasnt_empty && feedable.is_empty() {
        //     commandable.set_queue(
        //         CommandType::ToRest(ToRestCommand { commandable_entity }),
        //         commandable_entity,
        //         &mut commands,
        //         &mut commandable_interrupt_writer,
        //         &mut tasks_scheduler,
        //     );
        // }
        //
        // if wasnt_full && feedable.is_full() {
        //     event_writer.send(log_event!(RestCompleteEvent { commandable_entity }));
        // }
    }
}
