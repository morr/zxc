use super::*;

pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Storage>();
    }
}

#[derive(Component, Reflect, Default)]
pub struct Storage;
