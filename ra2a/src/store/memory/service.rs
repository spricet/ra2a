use crate::core::task::Task;
use crate::store::{TaskStore, TaskStoreError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
pub struct InMemoryTaskStore {
    store: Arc<Mutex<HashMap<String, Task>>>,
}

#[async_trait::async_trait]
impl TaskStore for InMemoryTaskStore {
    async fn fetch(&self, task_id: &str) -> Result<Option<Task>, TaskStoreError> {
        let store = self.store.lock().await;
        Ok(store.get(task_id).cloned())
    }

    async fn upsert(&self, task: Task) -> Result<Task, TaskStoreError> {
        let mut store = self.store.lock().await;
        store.insert(task.id.clone(), task.clone());
        Ok(task)
    }

    async fn delete(&self, task_id: &str) -> Result<Option<Task>, TaskStoreError> {
        let mut store = self.store.lock().await;
        Ok(store.remove(task_id))
    }
}
