use crate::*;

pub struct FeedablePlugin;

impl Plugin for FeedablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Feedable>()
            .add_observer(on_food_consumed)
            .add_systems(Update, progress_hunger.run_if(in_state(AppState::Playing)));
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
        self.hunger <= HUNGER_FRESH
    }

    pub fn is_overflowed(&self) -> bool {
        self.hunger >= HUNGER_OVERFLOW
    }

    pub fn is_death_starving(&self) -> bool {
        self.hunger >= HUNGER_OVERFLOW * config().feedable.max_starvation_multiplier
    }

    pub fn progress_hunger(&mut self, time_amount: f32) {
        let amount = time_amount * config().feedable.living_cost;
        self.hunger = (self.hunger + amount).clamp(
            HUNGER_FRESH,
            HUNGER_OVERFLOW * config().feedable.max_starvation_multiplier,
        );
    }

    pub fn be_fed(&mut self) {
        self.hunger -= HUNGER_OVERFLOW;
    }
}

const NEEDS_TICK_INTERVAL: f32 = 0.25;

#[allow(clippy::too_many_arguments)]
fn progress_hunger(
    mut commands: Commands,
    time: Res<Time>,
    mut tick_acc: Local<f32>,
    mut query: Query<(Entity, &mut Feedable, &mut Commandable)>,
    food_stock: Res<FoodStock>,
) {
    *tick_acc += time.delta_secs();
    if *tick_acc < NEEDS_TICK_INTERVAL {
        return;
    }
    let time_amount = *tick_acc;
    *tick_acc = 0.0;

    for (commandable_entity, mut feedable, mut commandable) in query.iter_mut() {
        let wasnt_overflowed = !feedable.is_overflowed();
        let wasnt_death_starving = !feedable.is_death_starving();

        feedable.progress_hunger(time_amount);

        if wasnt_overflowed && feedable.is_overflowed() && food_stock.amount > 0 {
            commandable.set_queue(
                CommandType::Feed(FeedCommand { commandable_entity }),
                commandable_entity,
                &mut commands,
            );
        }

        if wasnt_death_starving && feedable.is_death_starving() {
            commands.trigger(log_event!(PawnDeatEvent {
                entity: commandable_entity,
                reason: PawnDeathReason::Starvation
            }));
        }
    }
}

fn on_food_consumed(
    event: On<FoodConsumedEvent>,
    mut commands: Commands,
    mut carryable_query: Query<(Entity, &mut Carryable, &Transform), With<CarryableFoodMarker>>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let FoodConsumedEvent { amount } = *event;
    let mut amount_consumed = amount;

    for (entity, mut carryable, transform) in carryable_query.iter_mut() {
        continue_unless!(CarryableKind::Food, carryable.kind);

        if amount_consumed < carryable.amount {
            carryable.amount -= amount_consumed;
            break;
        } else {
            amount_consumed -= carryable.amount;

            commands.entity(entity).despawn();
            let grid_tile = transform.world_pos_to_grid();
            arc_navmesh
                .write()
                .remove_occupant::<Carryable>(&entity, grid_tile.x, grid_tile.y);
        }
    }
}
