use crate::agent::AgentHandler;
use crate::core::message::{SendMessageRequest, SendMessageResponse, SendMessageResponsePayload};
use crate::core::{A2A, A2AError, A2ATransportError};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct A2ADelegate {
    agent: Arc<dyn AgentHandler>,
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
        if let Some(message) = request.message {
            if let Some(_configuration) = request.configuration {
                // todo
            }

            let _task_id = match message.task_id.as_str() {
                "" => None,
                _ => Some(message.task_id.clone()),
            };
            // fetch the task

            let payload = self
                .agent
                .handle_message(message, request.metadata, None)
                .await?;
            if let SendMessageResponsePayload::Task(_task) = &payload {
                // persist the task
            }
            return Ok(SendMessageResponse {
                payload: Some(payload),
            });
        }
        Err(A2AError::Transport(A2ATransportError::MissingPayload))
    }
}

impl A2ADelegate {
    pub fn new<T: AgentHandler + 'static>(agent: Arc<T>) -> Self {
        A2ADelegate { agent }
    }
}
