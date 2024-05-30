use super::*;

pub struct BedPlugin;

impl Plugin for BedPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bed>().init_resource::<AvailableBeds>();
    }
}

#[derive(Component, Reflect, Default)]
pub struct Bed {
    pub owner: Option<Entity>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct AvailableBeds(pub u32);
