use rand_distr::num_traits::Zero;

use super::*;

pub struct FeedCommandPlugin;

impl Plugin for FeedCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FeedCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct FeedCommand {
    pub commandable_entity: Entity,
}

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<FeedCommand>,
    mut commandable_query: Query<(&mut Commandable, &mut Feedable)>,
    mut food_stock: ResMut<FoodStock>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
    mut food_consumed_event_writer: EventWriter<FoodConsumedEvent>,
) {
    let amount_before = food_stock.amount;

    for FeedCommand { commandable_entity } in command_reader.read() {
        if food_stock.amount.is_zero() {
            continue;
        }

        if let Ok((mut commandable, mut feedable)) = commandable_query.get_mut(*commandable_entity)
        {
            commandable.complete_executing(
                *commandable_entity,
                &mut commands,
                &mut commandable_event_writer,
            );

            while feedable.is_overflowed() && food_stock.amount > 0 {
                feedable.be_fed();
                food_stock.amount -= 1;
            }
        }
    }

    if amount_before != food_stock.amount {
        food_consumed_event_writer.write(log_event!(FoodConsumedEvent {
            amount: amount_before - food_stock.amount,
        }));
    }
}
