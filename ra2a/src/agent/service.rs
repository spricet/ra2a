use crate::agent::A2AAgentError;
use crate::core::message::{Message, SendMessageResponsePayload};
use crate::core::task::Task;
use crate::core::util::Object;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Agent: Debug + Send + Sync {
    async fn handle_message(
        &self,
        message: Message,
        metadata: Option<Object>,
        task: Option<Task>,
    ) -> Result<SendMessageResponsePayload, A2AAgentError>;
}

#[derive(Debug, Clone, Default)]
pub struct NoopAgent;

#[async_trait]
impl Agent for NoopAgent {
    async fn handle_message(
        &self,
        message: Message,
        _metadata: Option<Object>,
        _task: Option<Task>,
    ) -> Result<SendMessageResponsePayload, A2AAgentError> {
        Ok(SendMessageResponsePayload::Message(message))
    }
}
