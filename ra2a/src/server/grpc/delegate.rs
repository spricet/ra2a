use crate::core::message::{SendMessageRequest, SendMessageResponse, SendMessageResponsePayload};
use crate::core::{A2A, A2AError, A2ATransportError};

#[derive(Debug, Clone, Copy, Default)]
pub struct A2AGrpcDelegate;

#[async_trait::async_trait]
impl A2A for A2AGrpcDelegate {
    async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError> {
        if let Some(request) = request.message {
            let reply = SendMessageResponse {
                payload: Some(SendMessageResponsePayload::Message(request)),
            };
            return Ok(reply);
        }
        Err(A2AError::Transport(A2ATransportError::MissingPayload))
    }
}
