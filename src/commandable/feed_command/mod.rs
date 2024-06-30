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
) {
}
