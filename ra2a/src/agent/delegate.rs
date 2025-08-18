use crate::agent::AgentHandler;
use crate::core::message::{
    SendMessageConfiguration, SendMessageRequest, SendMessageResponse, SendMessageResponsePayload,
};
use crate::core::task::{GetTaskRequest, Task, TaskState, TaskStatus};
use crate::core::{A2A, A2AError, A2AProtocolError, A2ATransportError};
use crate::queue::TaskQueue;
use crate::queue::bounded::BoundedTaskQueue;
use crate::store::TaskStore;
use crate::store::memory::InMemoryTaskStore;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct A2ADelegate {
    agent: Arc<dyn AgentHandler>,
    store: Arc<dyn TaskStore>,
    queue: Arc<dyn TaskQueue>,
}

impl Debug for A2ADelegate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("A2ADelegate").finish()
    }
}

#[async_trait::async_trait]
impl A2A for A2ADelegate {
    async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError> {
        let mut message = match request.message {
            Some(message) => message,
            None => return Err(A2AError::Transport(A2ATransportError::MissingPayload)),
        };
        // ensures server owns the message_id
        message.message_id = Uuid::new_v4().to_string();
        // todo context ids should be validated to ensure user doesn't put wierd stuff in them
        // todo actually all things should be validated

        let configuration = request
            .configuration
            .unwrap_or_else(|| SendMessageConfiguration {
                // todo think about defaults
                accepted_output_modes: vec!["text/plain".to_string()],
                push_notification: None,
                history_length: 0,
                blocking: true,
            });

        let mut task = match &message.task_id {
            Some(task_id) => match self.store.fetch(task_id).await {
                Ok(Some(task)) => task,
                Ok(None) => {
                    return Err(A2AError::Protocol(A2AProtocolError::task_not_found(
                        task_id.clone(),
                    )));
                }
                Err(e) => return Err(e.into()),
            },
            None => Task::new(),
        };

        let payload = match configuration.blocking {
            true => {
                let payload = self
                    .agent
                    .handle_message(message, request.metadata, task)
                    .await?;
                match &payload {
                    SendMessageResponsePayload::Task(task) => {
                        // persist the task, audit, etc
                        self.store.upsert(task.clone()).await?;
                    }
                    SendMessageResponsePayload::Message(_message) => {
                        // no task, audit the message
                    }
                }
                payload
            }
            false => {
                task.status = Some(TaskStatus {
                    state: TaskState::Submitted.into(),
                    message: Some(message),
                    timestamp: None,
                });
                let task = self.store.upsert(task).await?;
                self.queue.push(task.clone()).await?;
                SendMessageResponsePayload::Task(task)
            }
        };
        Ok(SendMessageResponse {
            payload: Some(payload),
        })
    }

    async fn get_task(&self, request: GetTaskRequest) -> Result<Task, A2AError> {
        let task = self.store.fetch(&request.id).await?;
        match task {
            Some(task) => Ok(task),
            None => Err(A2AError::Protocol(A2AProtocolError::task_not_found(
                request.id,
            ))),
        }
    }
}

impl A2ADelegate {
    pub fn new<T: AgentHandler + 'static>(agent: Arc<T>) -> Self {
        A2ADelegate {
            agent,
            store: Arc::new(InMemoryTaskStore::default()),
            queue: Arc::new(BoundedTaskQueue::new(10)),
        }
    }
}
