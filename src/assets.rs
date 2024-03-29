use crate::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsCollection>()
            .add_systems(Startup, load_assets);
    }
}

#[derive(Resource, Default)]
pub struct AssetsCollection {
    pub pawn_idle: Handle<ColorMaterial>,
    pub pawn_moving: Handle<ColorMaterial>,
    pub pawn_pathfinding: Handle<ColorMaterial>,
    pub pawn_pathfinding_error: Handle<ColorMaterial>,
}

fn load_assets(mut assets: ResMut<AssetsCollection>, mut materials: ResMut<Assets<ColorMaterial>>) {
    assets.pawn_idle = materials.add(ColorMaterial::from(Color::hex("f6f8fa").unwrap()));
    assets.pawn_moving = materials.add(ColorMaterial::from(Color::hex("e178c5").unwrap()));
    assets.pawn_pathfinding = materials.add(ColorMaterial::from(Color::hex("fb8f44").unwrap()));
    assets.pawn_pathfinding_error = materials.add(ColorMaterial::from(Color::RED));
}
