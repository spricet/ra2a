use crate::agent::A2AAgentError;
use crate::core::message::{Message, SendMessageResponsePayload};
use crate::core::task::Task;
use crate::core::util::Object;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait AgentHandler: Debug + Send + Sync {
    async fn handle_message(
        &self,
        message: Message,
        metadata: Option<Object>,
        task: Task,
    ) -> Result<SendMessageResponsePayload, A2AAgentError>;
}

#[derive(Debug, Clone, Default)]
pub struct NoopAgentHandler;

#[async_trait]
impl AgentHandler for NoopAgentHandler {
    async fn handle_message(
        &self,
        message: Message,
        _metadata: Option<Object>,
        _task: Task,
    ) -> Result<SendMessageResponsePayload, A2AAgentError> {
        Ok(SendMessageResponsePayload::Message(message))
    }
}
