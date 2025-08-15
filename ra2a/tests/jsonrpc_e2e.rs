#[cfg(test)]
#[cfg(feature = "server")]
mod tests {
    use ra2a::client::jsonrpc::A2AJsonRpcClient;
    use ra2a::core::A2A;
    use ra2a::core::message::{Message, SendMessageRequest, SendMessageResponsePayload};
    use ra2a::core::role::Role;
    use ra2a::server::jsonrpc::A2AJsonRpcServer;

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

        let server = A2AJsonRpcServer {};
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let serve = server.serve(async { rx.await.expect("unexpected server shutdown signal") });
        let test = async {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            let client = A2AJsonRpcClient::new("http://localhost:50123").unwrap();
            let res = client
                .send_message(SendMessageRequest {
                    message: Some(test_message.clone()),
                    configuration: None,
                    metadata: None,
                })
                .await
                .unwrap();
            tx.send(()).unwrap();
            res
        };
        let (_, res) = tokio::join!(serve, test);
        let payload = match res.payload.unwrap() {
            SendMessageResponsePayload::Task(_) => panic!("expected message"),
            SendMessageResponsePayload::Message(m) => m,
        };
        assert_eq!(payload, test_message);
    }
}
