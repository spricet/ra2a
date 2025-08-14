use crate::core::A2AError;
use crate::core::message::{SendMessageRequest, SendMessageResponse};

#[async_trait::async_trait]
pub trait A2A {
    async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError>;
}
