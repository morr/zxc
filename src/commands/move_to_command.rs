use super::*;

#[derive(Event, Debug)]
pub struct MoveToCommand {
    pub id: Entity,
    pub grid_tile: IVec2,
}

pub fn execute_move_to(mut command_reader: EventReader<MoveToCommand>) {
    for command in command_reader.read() {
        println!("{:?}", command);
        // it does something
    }
}
