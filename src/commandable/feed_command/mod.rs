use rand_distr::num_traits::Zero;

use super::*;

pub struct FeedCommandPlugin;

impl Plugin for FeedCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(execute_command);
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct FeedCommand {
    pub commandable_entity: Entity,
}

fn execute_command(
    event: On<FeedCommand>,
    mut commands: Commands,
    mut commandable_query: Query<(&mut Commandable, &mut Feedable)>,
    mut food_stock: ResMut<FoodStock>,
) {
    let FeedCommand { commandable_entity } = *event;

    if let Ok((mut commandable, mut feedable)) = commandable_query.get_mut(commandable_entity) {
        if food_stock.amount.is_zero() {
            commandable.complete_executing(commandable_entity, &mut commands);
            return;
        }
        let amount_before = food_stock.amount;

        commandable.complete_executing(commandable_entity, &mut commands);

        while feedable.is_overflowed() && food_stock.amount > 0 {
            feedable.be_fed();
            food_stock.amount -= 1;
        }

        if amount_before != food_stock.amount {
            commands.trigger(log_event!(FoodConsumedEvent {
                amount: amount_before - food_stock.amount,
            }));
        }
    }
}
