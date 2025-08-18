use crate::client::grpc::A2AGrpcClientError;
use crate::core::message::{SendMessageRequest, SendMessageResponse};
use crate::core::task::{GetTaskGrpcRequest, GetTaskRequest, Task};
use crate::core::{
    A2A, A2AError, A2AErrorCode, A2AProtocolError, GRPC_GET_TASK_PATH, GRPC_SEND_MESSAGE_PATH,
};
use async_trait::async_trait;
use http::uri::PathAndQuery;
use tonic::client::Grpc;
use tonic::transport::Channel;
use tonic::{Code, Request};
use tonic_prost::ProstCodec;

#[derive(Debug, Clone)]
pub struct A2AGrpcClient {
    channel: Channel,
}

impl A2AGrpcClient {
    pub async fn new(url: impl Into<String>) -> Result<Self, A2AGrpcClientError> {
        let url = url.into();
        let channel = Channel::from_shared(url)?.connect().await?;
        Ok(Self { channel })
    }
}

#[async_trait]
impl A2A for A2AGrpcClient {
    async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> Result<SendMessageResponse, A2AError> {
        let mut grpc = Grpc::new(self.channel.clone());
        grpc.ready()
            .await
            .map_err(|e| tonic::Status::unavailable(format!("client not ready: {e}")))?;
        let res = grpc
            .unary(
                Request::new(request),
                PathAndQuery::from_static(GRPC_SEND_MESSAGE_PATH),
                ProstCodec::<SendMessageRequest, SendMessageResponse>::default(),
            )
            .await;
        match res {
            Ok(res) => Ok(res.into_inner()),
            // todo unpack various grpc error codes to discover protocol errors underneath, which probably depends on the spec getting updated
            Err(err) => Err(A2AError::from(err)),
        }
    }

    async fn get_task(&self, request: GetTaskRequest) -> Result<Task, A2AError> {
        let task_id = request.id.clone();
        let request: GetTaskGrpcRequest = request.into();
        let mut grpc = Grpc::new(self.channel.clone());
        grpc.ready()
            .await
            .map_err(|e| tonic::Status::unavailable(format!("client not ready: {e}")))?;
        let res = grpc
            .unary(
                Request::new(request),
                PathAndQuery::from_static(GRPC_GET_TASK_PATH),
                ProstCodec::<GetTaskGrpcRequest, Task>::default(),
            )
            .await;
        match res {
            Ok(res) => Ok(res.into_inner()),
            // todo unpack various grpc error codes to discover protocol errors underneath, which probably depends on the spec getting updated
            Err(err) => match err.code() {
                Code::NotFound => Err(A2AError::Protocol(A2AProtocolError::TaskNotFound {
                    id: task_id,
                    code: A2AErrorCode::TaskNotFound,
                })),
                _ => Err(A2AError::from(err)),
            },
        }
    }
}
