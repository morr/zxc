use super::*;

pub struct BedPlugin;

impl Plugin for BedPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bed>();
    }
}

#[derive(Component, Reflect, Default)]
pub struct Bed {
    pub owner: Option<Entity>,
}
