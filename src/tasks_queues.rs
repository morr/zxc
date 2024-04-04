use std::sync::{atomic::{AtomicI32, Ordering}, Arc};

use bevy::tasks::AsyncComputeTaskPool;

use super::*;

pub struct TaskQueuesPlugin;

#[derive(Resource, Default)]
pub struct AsyncQueueCounter(pub Arc<AtomicI32>);

impl Plugin for TaskQueuesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsyncQueueCounter>();
    }
}

pub fn spawn_task<Fut>(queue_counter: &Res<AsyncQueueCounter>, future: Fut) -> bevy::tasks::Task<Fut::Output>
where
    Fut: std::future::Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    let thread_pool = AsyncComputeTaskPool::get();
    let queue_counter_clone = queue_counter.0.clone();

    queue_counter.0.fetch_add(1, Ordering::SeqCst);

    thread_pool.spawn(async move {
        let result = future.await;
        queue_counter_clone.fetch_sub(1, Ordering::SeqCst);
        result
    })
}

