#[cfg(test)]
#[cfg(feature = "grpc")]
mod tests {
    use ra2a::agent::{AgentBuilder, NoopAgentHandler};
    use ra2a::client::grpc::A2AGrpcClient;
    use ra2a::core::message::{Message, SendMessageRequest, SendMessageResponsePayload};
    use ra2a::core::role::Role;
    use ra2a::core::{A2A, Transport};

    #[tokio::test]
    async fn test_send_message() {
        let test_message = Message {
            message_id: String::new(),
            context_id: String::new(),
            task_id: String::new(),
            role: Role::Agent.into(),
            content: vec![],
            metadata: None,
            extensions: vec![],
        };

        let agent = AgentBuilder::new(NoopAgentHandler)
            .with_name("test")
            .with_grpc_server("[::]:0".parse().unwrap())
            .build()
            .unwrap();
        assert_eq!(agent.supported_transports(), vec![Transport::Grpc]);

        let server = agent.start_server().await.unwrap();
        let addr = server.local_addr(Transport::Grpc).unwrap();

        let client = A2AGrpcClient::new(format!("http://localhost:{}", addr.port()))
            .await
            .unwrap();
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
