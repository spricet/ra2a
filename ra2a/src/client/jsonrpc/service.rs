use crate::client::jsonrpc::A2AJsonRpcClientError;
use crate::core::message::{SendMessageRequest, SendMessageResponse};
use crate::core::task::{GetTaskRequest, Task};
use crate::core::{
    A2A, A2AError, A2AErrorCode, A2AProtocolError, JSONRPC_GET_TASK_METHOD,
    JSONRPC_SEND_MESSAGE_METHOD,
};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};

#[derive(Debug, Clone)]
pub struct A2AJsonRpcClient {
    client: HttpClient,
}

impl A2AJsonRpcClient {
    pub fn new(url: impl AsRef<str>) -> Result<Self, A2AJsonRpcClientError> {
        let client = HttpClientBuilder::default().build(url)?;

        Ok(A2AJsonRpcClient { client })
    }
}

#[async_trait::async_trait]
impl A2A for A2AJsonRpcClient {
    async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError> {
        let response = self
            .client
            .request(JSONRPC_SEND_MESSAGE_METHOD, request)
            .await?;
        Ok(response)
    }

    async fn get_task(&self, request: GetTaskRequest) -> Result<Task, A2AError> {
        let task_id = request.id.to_string();
        let response = self.client.request(JSONRPC_GET_TASK_METHOD, request).await;
        match response {
            Ok(response) => Ok(response),
            Err(jsonrpsee::core::client::Error::Call(e)) => {
                if e.code() == A2AErrorCode::TaskNotFound as i32 {
                    Err(A2AError::Protocol(A2AProtocolError::TaskNotFound {
                        id: task_id,
                        code: A2AErrorCode::TaskNotFound,
                    }))
                } else {
                    Err(A2AError::Transport(
                        jsonrpsee::core::client::Error::Call(e).into(),
                    ))
                }
            }
            Err(e) => Err(A2AError::Transport(e.into())),
        }
    }
}
