#[cfg(test)]
#[cfg(feature = "grpc")]
mod tests {
    use ra2a::client::grpc::A2AGrpcClient;
    use ra2a::core::A2A;
    use ra2a::core::message::{Message, SendMessageRequest, SendMessageResponsePayload};
    use ra2a::core::role::Role;
    use ra2a::server::A2AServer;
    use std::net::SocketAddr;
    use std::str::FromStr;

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

        let server =
            A2AServer::default().with_grpc(SocketAddr::from_str("127.0.0.1:50051").unwrap());
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let serve = server
            .serve_with_shutdown(async { rx.await.expect("unexpected server shutdown signal") });
        let test = async {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            let client = A2AGrpcClient::new("http://localhost:50051").await.unwrap();
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
