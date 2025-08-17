# ra2a

Rust SDK for the Agent To Agent protocol (A2A)

## Basic Usage

```rust
let agent = AgentBuilder::new(YourAgentHandler)
    .with_name("my-agent")
    .with_json_rpc_server("[::]:0".parse()?)
    .with_grpc_server("[::]:0".parse()?)
    .build()?;

let handle = agent.start_server().await?;
for (transport, addr) in handle.local_addrs().into_iter() {
    println!("Running {transport} server on {addr}");
}

handle.join().await?;

```