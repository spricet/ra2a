use crate::core::A2AError;
use crate::core::message::{SendMessageRequest, SendMessageResponse};

pub const GRPC_SERVICE_NAME: &str = "a2a.v1.A2AService";
pub const GRPC_SEND_MESSAGE_PATH: &str = "/a2a.v1.A2AService/SendMessage";

#[async_trait::async_trait]
pub trait A2A {
    async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError>;
}
