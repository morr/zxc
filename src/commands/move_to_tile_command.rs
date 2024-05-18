use super::*;

#[derive(Event, Debug)]
pub struct MoveToTileCommand {
    pub id: Entity,
    pub grid_tile: IVec2,
}

pub fn move_to_tile_command(mut command_reader: EventReader<MoveToTileCommand>) {
    for command in command_reader.read() {
        println!("{:?}", command);
        // it does something
    }
}
