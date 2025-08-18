use crate::core::A2AError;
use crate::core::message::{SendMessageRequest, SendMessageResponse};
use crate::core::task::{GetTaskRequest, Task};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub const GRPC_SERVICE_NAME: &str = "a2a.v1.A2AService";
pub const GRPC_SEND_MESSAGE_PATH: &str = "/a2a.v1.A2AService/SendMessage";
pub const GRPC_GET_TASK_PATH: &str = "/a2a.v1.A2AService/GetTask";
pub const JSONRPC_SEND_MESSAGE_METHOD: &str = "message/send";
pub const JSONRPC_GET_TASK_METHOD: &str = "tasks/get";

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

    async fn get_task(&self, request: GetTaskRequest) -> Result<Task, A2AError>;
}

impl Display for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transport::Grpc => write!(f, "grpc"),
            Transport::JsonRpc => write!(f, "json-rpc"),
        }
    }
}
