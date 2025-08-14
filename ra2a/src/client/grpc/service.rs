use crate::client::grpc::A2AGrpcClientError;
use crate::core::agent::AgentCard;
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
