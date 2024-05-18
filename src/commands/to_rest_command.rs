use super::*;

#[derive(Event, Debug)]
pub struct ToRestCommand(pub Entity);

pub fn to_rest_command(
    mut to_rest_command_reader: EventReader<ToRestCommand>,
) {
    for command in to_rest_command_reader.read() {
        println!("{:?}", command);
    }
}
