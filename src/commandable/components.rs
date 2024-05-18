use super::*;

use std::collections::VecDeque;

#[derive(Debug)]
pub enum Command {
    UserSelection(UserSelectionData),
    ToRest(Entity),
    MoveTo(Entity, IVec2),
}

#[derive(Component, Debug, Default)]
pub struct Commandable {
    pub queue: VecDeque<Command>,
}

impl Commandable {
    pub fn schedule(&mut self, command: Command) {
        self.queue.push_back(command);
    }
}
