use crate::*;

pub struct FeedablePlugin;

impl Plugin for FeedablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Feedable>()
            .add_event::<FoodConsumedEvent>()
            .add_systems(
                Update,
                (progress_hunger, process_consumed_food)
                    .chain()
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

#[derive(Event, Debug)]
pub struct FoodConsumedEvent {
    pub amount: u32,
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
        self.hunger = (self.hunger + amount).clamp(
            HUNGER_FRESH,
            HUNGER_OVERFLOW * config().feedable.max_starvation_multiplier,
        );
    }

    pub fn feed(&mut self) {
        self.hunger -= HUNGER_OVERFLOW;
    }
}

fn progress_hunger(
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Feedable, &mut Commandable)>,
    mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    mut commandable_release_resources_writer: EventWriter<ReleaseCommandResourcesEvent>,
    food_stock: Res<FoodStock>,
) {
    let time_amount = time_scale.scale_to_seconds(time.delta_seconds());

    for (commandable_entity, mut feedable, mut commandable) in query.iter_mut() {
        let wasnt_overflowed = !feedable.is_overflowed();

        feedable.progress_hunger(time_amount);

        if wasnt_overflowed && feedable.is_overflowed() && food_stock.amount > 0 {
            commandable.set_queue(
                CommandType::Feed(FeedCommand { commandable_entity }),
                commandable_entity,
                &mut commands,
                &mut commandable_interrupt_writer,
                &mut commandable_release_resources_writer,
            );
        }
    }
}

fn process_consumed_food(
    mut commands: Commands,
    mut event_reader: EventReader<FoodConsumedEvent>,
    mut carryable_query: Query<(Entity, &mut Carryable, &Transform), With<CarryableFoodMarker>>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    for FoodConsumedEvent {
        amount: mut amount_consumed,
    } in event_reader.read()
    {
        for (entity, mut carryable, transform) in carryable_query.iter_mut() {
            continue_unless!(CarryableKind::Food, carryable.kind);

            if amount_consumed < carryable.amount {
                carryable.amount -= amount_consumed;
                break;
            } else {
                amount_consumed -= carryable.amount;

                commands.entity(entity).despawn_recursive();
                let grid_tile = transform.world_pos_to_grid();
                arc_navmesh
                    .write()
                    .remove_occupant::<Carryable>(&entity, grid_tile.x, grid_tile.y);
            }
        }
    }
}
