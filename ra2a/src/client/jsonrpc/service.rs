use crate::client::jsonrpc::A2AJsonRpcClientError;
use crate::core::message::{SendMessageRequest, SendMessageResponse};
use crate::core::{A2A, A2AError, JSONRPC_SEND_MESSAGE_METHOD};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};

#[derive(Debug, Clone)]
pub struct A2AJsonRpcClient {
    client: HttpClient,
}

impl A2AJsonRpcClient {
    pub fn new() -> Result<Self, A2AJsonRpcClientError> {
        let client = HttpClientBuilder::default().build("http://localhost:50051")?;

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
}
