#[cfg(test)]
#[cfg(feature = "grpc")]
mod tests {
    use ra2a::agent::{AgentBuilder, NoopAgentHandler};
    use ra2a::client::grpc::A2AGrpcClient;
    use ra2a::core::message::{Message, SendMessageRequest, SendMessageResponsePayload};
    use ra2a::core::role::Role;
    use ra2a::core::{A2A, Transport};
    use std::net::SocketAddr;

    #[tokio::test]
    async fn test_send_message() {
        let test_message = Message {
            message_id: "stuff yo".to_string(),
            context_id: "context".to_string(),
            task_id: "task".to_string(),
            role: Role::Agent.into(),
            content: vec![],
            metadata: None,
            extensions: vec![],
        };

        let agent = AgentBuilder::new(NoopAgentHandler)
            .with_name("test")
            .with_grpc_server("127.0.0.1:50051".parse::<SocketAddr>().unwrap())
            .build()
            .unwrap();
        assert_eq!(agent.supported_transports(), vec![Transport::Grpc]);

        let server = agent.start_server();

        let client = A2AGrpcClient::new("http://localhost:50051").await.unwrap();
        let res = client
            .send_message(SendMessageRequest {
                message: Some(test_message.clone()),
                configuration: None,
                metadata: None,
            })
            .await
            .unwrap();
        let payload = match res.payload.unwrap() {
            SendMessageResponsePayload::Task(_) => panic!("expected message"),
            SendMessageResponsePayload::Message(m) => m,
        };
        assert_eq!(payload, test_message);

        server.shutdown().await.unwrap();
    }
}
