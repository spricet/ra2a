#[cfg(test)]
#[cfg(feature = "server")]
mod tests {
    use ra2a::agent::NoopAgent;
    use ra2a::server::A2AServer;

    #[tokio::test]
    async fn basic_execution() {
        let server = A2AServer::new(NoopAgent).with_jsonrpc("127.0.0.1:50123".parse().unwrap());
        #[cfg(feature = "grpc")]
        let server = server.with_grpc("127.0.0.1:50124".parse().unwrap());
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let serve = server
            .serve_with_shutdown(async { rx.await.expect("unexpected server shutdown signal") });
        let handle = tokio::task::spawn(serve);

        tx.send(()).unwrap();

        let _ = handle.await.unwrap();
    }
}
