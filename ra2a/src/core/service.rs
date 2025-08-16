use crate::core::A2AError;
use crate::core::message::{SendMessageRequest, SendMessageResponse};
use serde::{Deserialize, Serialize};

pub const GRPC_SERVICE_NAME: &str = "a2a.v1.A2AService";
pub const GRPC_SEND_MESSAGE_PATH: &str = "/a2a.v1.A2AService/SendMessage";
pub const JSONRPC_SEND_MESSAGE_METHOD: &str = "message/send";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Transport {
    Grpc,
    JsonRpc,
}

#[async_trait::async_trait]
pub trait A2A {
    async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError>;
}
