use super::*;

pub struct MoveToCommandPlugin;

impl Plugin for MoveToCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveToCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone)]
pub struct MoveToCommand(pub Entity, pub IVec2);

fn execute_command(mut command_reader: EventReader<MoveToCommand>) {
    for command in command_reader.read() {
        println!("{:?}", command);
        // it does something
    }
}
