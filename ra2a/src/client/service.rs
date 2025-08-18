use crate::client::A2AClientError;
use crate::client::jsonrpc::A2AJsonRpcClient;
use crate::core::message::{SendMessageRequest, SendMessageResponse};
use crate::core::task::{GetTaskRequest, Task};
use crate::core::{A2A, A2AError, Transport};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum A2AClient {
    JsonRpc(A2AJsonRpcClient),
    #[cfg(feature = "grpc")]
    Grpc(crate::client::grpc::A2AGrpcClient),
}

impl A2AClient {
    pub async fn new(transport: Transport, url: impl AsRef<str>) -> Result<Self, A2AClientError> {
        let client = match transport {
            #[cfg(feature = "grpc")]
            Transport::Grpc => {
                Self::Grpc(crate::client::grpc::A2AGrpcClient::new(url.as_ref().to_string()).await?)
            }
            Transport::JsonRpc => Self::JsonRpc(A2AJsonRpcClient::new(url.as_ref().to_string())?),
        };
        Ok(client)
    }
}

#[async_trait]
impl A2A for A2AClient {
    async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError> {
        match self {
            A2AClient::JsonRpc(c) => c.send_message(request).await,
            #[cfg(feature = "grpc")]
            A2AClient::Grpc(c) => c.send_message(request).await,
        }
    }

    async fn get_task(&self, request: GetTaskRequest) -> Result<Task, A2AError> {
        match self {
            A2AClient::JsonRpc(c) => c.get_task(request).await,
            #[cfg(feature = "grpc")]
            A2AClient::Grpc(c) => c.get_task(request).await,
        }
    }
}
