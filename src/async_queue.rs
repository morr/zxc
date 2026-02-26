use std::sync::{
    Arc,
    atomic::{AtomicI32, Ordering},
};

use bevy::tasks::AsyncComputeTaskPool;

use super::*;

pub struct AsyncQueuePlugin;

#[derive(Resource, Default)]
pub struct AsyncQueueCounter(pub Arc<AtomicI32>);

impl AsyncQueueCounter {
    pub fn increment(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement(&self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn get(&self) -> i32 {
        self.0.load(Ordering::SeqCst)
    }
}

impl Plugin for AsyncQueuePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsyncQueueCounter>();
    }
}

pub fn spawn_async_task<Fut>(
    queue_counter: &Res<AsyncQueueCounter>,
    future: Fut,
) -> bevy::tasks::Task<Fut::Output>
where
    Fut: std::future::Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    queue_counter.increment();

    let thread_pool = AsyncComputeTaskPool::get();
    let queue_counter_clone = queue_counter.0.clone();

    thread_pool.spawn(async move {
        let result = future.await;
        queue_counter_clone.fetch_sub(1, Ordering::SeqCst);
        result
    })
}
