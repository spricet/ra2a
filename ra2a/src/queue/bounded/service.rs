use crate::core::task::Task;
use crate::queue::{TaskQueue, TaskQueueError};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone)]
pub struct BoundedTaskQueue {
    tx: Sender<Task>,
    rx: Arc<Mutex<Option<Receiver<Task>>>>,
}

impl BoundedTaskQueue {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(capacity);
        Self {
            tx,
            rx: Arc::new(Mutex::new(Some(rx))),
        }
    }
}

#[async_trait::async_trait]
impl TaskQueue for BoundedTaskQueue {
    async fn push(&self, task: Task) -> Result<(), TaskQueueError> {
        self.tx.send(task).await.map_err(|_| TaskQueueError::Closed)
    }

    async fn take(&self) -> Result<Task, TaskQueueError> {
        // temporarily take ownership of the Receiver
        let mut guard = self.rx.lock().await;
        let mut rx = guard.take().ok_or(TaskQueueError::Closed)?;
        drop(guard); // <- drop guard before awaiting

        let res = rx.recv().await;

        // put it back
        let mut guard = self.rx.lock().await;
        *guard = Some(rx);
        drop(guard);

        res.ok_or(TaskQueueError::Closed)
    }
}
