use std::sync::{atomic::AtomicI32, Arc};

use super::*;

pub struct TaskQueuesPlugin;

#[derive(Resource, Default)]
pub struct AsyncQueueCounter(pub Arc<AtomicI32>);

impl Plugin for TaskQueuesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsyncQueueCounter>();
    }
}
