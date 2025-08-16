#[cfg(test)]
#[cfg(feature = "agent")]
mod tests {
    use ra2a::agent::{AgentBuilder, NoopAgentHandler};
    use std::net::SocketAddr;

    #[tokio::test]
    async fn basic_execution() {
        let agent_builder = AgentBuilder::new(NoopAgentHandler)
            .with_name("test")
            .with_json_rpc_server("127.0.0.1:0".parse::<SocketAddr>().unwrap());
        #[cfg(feature = "grpc")]
        let agent_builder =
            agent_builder.with_grpc_server("127.0.0.1:50128".parse::<SocketAddr>().unwrap());
        let agent = agent_builder.build().expect("failed to build agent");

        let handle = agent.start_server().await.expect("failed to start server");

        handle.shutdown().await.unwrap()
    }
}
