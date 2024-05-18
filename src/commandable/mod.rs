use std::collections::VecDeque;

use crate::*;

pub struct CommandablePlugin;

impl Plugin for CommandablePlugin {
    fn build(&self, app: &mut App) {
        app
            // .register_type::<Commandable>()
            .add_systems(
                Update,
                process_commands
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

#[derive(Debug)]
pub enum Command {
    UserSelection(UserSelectionData),
    ToRest(Entity),
    MoveTo { id: Entity, grid_tile: IVec2 },
}

#[derive(Component, Debug, Default)]
pub struct Commandable {
    pub queue: VecDeque<Command>,
}

pub fn process_commands(
    // mut commands: Commands,
    mut commandable_query: Query<&mut Commandable>,
    mut user_selection_command_writer: EventWriter<UserSelectionCommand>,
    mut to_rest_command_writer: EventWriter<ToRestCommand>,
    mut move_to_command_writer: EventWriter<MoveToCommand>,
) {
    for mut commandable in &mut commandable_query {
        if let Some(command) = commandable.queue.pop_front() {
            match command {
                Command::UserSelection(data) => {
                    user_selection_command_writer.send(UserSelectionCommand(Some(data)));
                }
                Command::ToRest(entity) => {
                    to_rest_command_writer.send(ToRestCommand(entity));
                }
                Command::MoveTo { id, grid_tile } => {
                    move_to_command_writer.send(MoveToCommand { id, grid_tile });
                }
            }
            // Update the state of the entity to indicate it is executing a command
            // commands.entity(entity).insert(ExecutingCommand);
        }
    }
}
