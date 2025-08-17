use crate::core::task::Task;
use crate::store::TaskStoreError;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait TaskStore: Debug + Send + Sync {
    async fn fetch(&self, task_id: &str) -> Result<Option<Task>, TaskStoreError>;
    async fn upsert(&self, task: Task) -> Result<Task, TaskStoreError>;
    async fn delete(&self, task_id: &str) -> Result<Option<Task>, TaskStoreError>;
}
