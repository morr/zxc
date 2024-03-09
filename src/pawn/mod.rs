use bevy::prelude::*;
mod systems;

#[derive(Component)]
struct Pawn {}

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {}
}
