use crate::agent::AgentHandler;
use crate::core::message::{
    SendMessageConfiguration, SendMessageRequest, SendMessageResponse, SendMessageResponsePayload,
};
use crate::core::task::{Task, TaskState, TaskStatus};
use crate::core::{A2A, A2AError, A2AProtocolError, A2ATransportError};
use crate::queue::TaskQueue;
use crate::queue::bounded::BoundedTaskQueue;
use crate::store::TaskStore;
use crate::store::memory::InMemoryTaskStore;
use std::fmt::Debug;
use std::sync::Arc;

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
        let message = match request.message {
            Some(message) => message,
            None => return Err(A2AError::Transport(A2ATransportError::MissingPayload)),
        };

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
