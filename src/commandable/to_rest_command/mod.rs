use super::*;

pub struct ToRestCommandPlugin;

impl Plugin for ToRestCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToRestCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone)]
pub struct ToRestCommand(pub Entity);

fn execute_command(mut command_reader: EventReader<ToRestCommand>) {
    for command in command_reader.read() {
        println!("{:?}", command);
        // it does something
    }
}
