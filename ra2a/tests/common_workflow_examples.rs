#[cfg(test)]
#[cfg(feature = "server")]
mod tests {
    use ra2a::agent::{AgentBuilder, NoopAgentHandler};
    use std::net::SocketAddr;

    #[tokio::test]
    async fn basic_execution() {
        let agent_builder = AgentBuilder::new(NoopAgentHandler)
            .with_name("test")
            .with_json_rpc_server("127.0.0.1:50123".parse::<SocketAddr>().unwrap());
        #[cfg(feature = "grpc")]
        let agent_builder =
            agent_builder.with_grpc_server("127.0.0.1:50124".parse::<SocketAddr>().unwrap());
        let agent = agent_builder.build().expect("failed to build agent");

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let serve = agent
            .serve_with_shutdown(async { rx.await.expect("unexpected server shutdown signal") });
        let handle = tokio::task::spawn(serve);

        tx.send(()).unwrap();

        let _ = handle.await.unwrap();
    }
}
