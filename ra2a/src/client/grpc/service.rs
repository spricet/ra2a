use crate::client::grpc::A2AGrpcClientError;
use crate::core::agent::AgentCard;
use crate::core::message::{SendMessageRequest, SendMessageResponse};
use crate::core::{A2A, A2AError, GRPC_SEND_MESSAGE_PATH};
use async_trait::async_trait;
use http::uri::PathAndQuery;
use tonic::Request;
use tonic::client::Grpc;
use tonic::transport::Channel;
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

    pub async fn test(&self) -> Result<(), A2AGrpcClientError> {
        let mut grpc = Grpc::new(self.channel.clone());
        let path = PathAndQuery::from_static("/example.Greeter/SayHello");
        let card = AgentCard {
            protocol_version: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            url: "".to_string(),
            preferred_transport: None,
            additional_interfaces: vec![],
            provider: None,
            version: "".to_string(),
            documentation_url: "".to_string(),
            capabilities: None,
            security_schemes: Default::default(),
            security: vec![],
            default_input_modes: vec![],
            default_output_modes: vec![],
            skills: vec![],
            supports_authenticated_extended_card: false,
            signatures: vec![],
            icon_url: "".to_string(),
        };
        let res = grpc
            .unary(
                Request::new(card),
                path,
                ProstCodec::<AgentCard, AgentCard>::default(),
            )
            .await?;
        // Here you would typically make a gRPC call to test the connection.
        // For demonstration purposes, we will just return Ok.
        Ok(())
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
}
