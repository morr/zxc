use crate::*;

pub struct FeedablePlugin;

impl Plugin for FeedablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Feedable>().add_systems(
            Update,
            progress_hunger
                .run_if(in_state(AppState::Playing))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

const HUNGER_FRESH: f32 = 0.;
const HUNGER_OVERFLOW: f32 = 100.;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Feedable {
    pub hunger: f32,
}

impl Default for Feedable {
    fn default() -> Self {
        Self {
            hunger: HUNGER_FRESH,
        }
    }
}

impl Feedable {
    pub fn is_fresh(&self) -> bool {
        self.hunger == HUNGER_FRESH
    }

    pub fn is_overflowed(&self) -> bool {
        self.hunger == HUNGER_OVERFLOW
    }

    pub fn progress_hunger(&mut self, time_amount: f32) {
        let amount = time_amount * config().feedable.living_cost;
        self.hunger = (self.hunger + amount).clamp(HUNGER_FRESH, HUNGER_OVERFLOW);
    }
}

fn progress_hunger(
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
        // let wasnt_fresh = !feedable.is_fresh();
        // let wasnt_full = !feedable.is_overflowed();

        feedable.progress_hunger(time_amount);

        // if wasnt_fresh && feedable.is_fresh() {
        //     commandable.set_queue(
        //         CommandType::ToRest(ToRestCommand { commandable_entity }),
        //         commandable_entity,
        //         &mut commands,
        //         &mut commandable_interrupt_writer,
        //         &mut tasks_scheduler,
        //     );
        // }
        //
        // if wasnt_full && feedable.is_overflowed() {
        //     event_writer.send(log_event!(RestCompleteEvent { commandable_entity }));
        // }
    }
}
