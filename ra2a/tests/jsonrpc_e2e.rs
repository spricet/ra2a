#[cfg(test)]
#[cfg(feature = "agent")]
mod tests {
    use ra2a::agent::{AgentBuilder, NoopAgentHandler};
    use ra2a::client::jsonrpc::A2AJsonRpcClient;
    use ra2a::core::A2A;
    use ra2a::core::message::{Message, SendMessageRequest, SendMessageResponsePayload};
    use ra2a::core::role::Role;
    use std::net::SocketAddr;

    #[tokio::test]
    async fn test_send_message() {
        let test_message = Message {
            message_id: String::new(),
            context_id: None,
            task_id: None,
            role: Role::Agent.into(),
            content: vec![],
            metadata: None,
            extensions: vec![],
        };

        let agent = AgentBuilder::new(NoopAgentHandler)
            .with_name("test")
            .with_json_rpc_server("127.0.0.1:52123".parse::<SocketAddr>().unwrap())
            .build()
            .expect("agent build");

        let handle = agent.start_server().await.expect("agent start");
        let client = A2AJsonRpcClient::new("http://localhost:52123").unwrap();
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

        handle.shutdown().await.unwrap()
    }
}
