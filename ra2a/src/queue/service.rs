use crate::core::task::Task;
use crate::queue::TaskQueueError;

#[async_trait::async_trait]
pub trait TaskQueue: Send + Sync {
    async fn push(&self, task: Task) -> Result<(), TaskQueueError>;
    async fn take(&self) -> Result<Task, TaskQueueError>;
}
