use bevy::prelude::*;
mod systems;

#[derive(Component, Debug)]
pub struct Pawn {}

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {}
}
