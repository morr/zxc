use super::*;

#[derive(Event, Debug)]
pub struct GoToCommand {
    pub id: Entity,
    pub grid_tile: IVec2,
}

pub fn go_to_command(mut command_reader: EventReader<GoToCommand>) {
    for command in command_reader.read() {
        println!("{:?}", command);
    }
}
