use super::*;

#[derive(Event, Debug)]
pub struct ToRestCommand(pub Entity);

pub fn to_rest_command(
    mut command_reader: EventReader<ToRestCommand>,
) {
    for command in command_reader.read() {
        println!("{:?}", command);
        // it does something
    }
}
