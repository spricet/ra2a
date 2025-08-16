# ra2a

Rust SDK for the Agent To Agent protocol (A2A)

## Basic Usage

```rust
let agent = AgentBuilder::new(YourAgentHandler)
    .with_name("my-agent")
    .with_json_rpc_server("[::]:0".parse()?)
    .with_grpc_server("[::]:0".parse()?)
    .build()?;

let server = agent.start_server().await?;
if let Some(addr) = server.local_addr(Transport::Grpc) {
    println ! ("Running GRPC server on {}", addr);
}

server.join().await?;

```