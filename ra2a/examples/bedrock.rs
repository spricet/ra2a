use ra2a::agent::{A2AAgentError, AgentBuilder, AgentHandler};
use ra2a::core::message::{Message, SendMessageResponsePayload};
use ra2a::core::task::Task;
use ra2a::core::util::Object;

#[derive(Debug, Clone)]
pub struct MyAgentHandler {
    // client: aws_sdk_bedrockruntime::Client
}

impl MyAgentHandler {
    async fn send_message(&self, _message: &Message) -> anyhow::Result<()> {
        todo!()
    }
}

#[async_trait::async_trait]
impl AgentHandler for MyAgentHandler {
    async fn handle_message(
        &self,
        message: Message,
        _metadata: Option<Object>,
        _task: Task,
    ) -> Result<SendMessageResponsePayload, A2AAgentError> {
        Ok(SendMessageResponsePayload::Message(message))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let agent = AgentBuilder::new(MyAgentHandler {})
        .with_name("my-agent")
        .with_json_rpc_server("[::]:0".parse()?)
        .with_grpc_server("[::]:0".parse()?)
        .build()?;

    let handle = agent.start_server().await?;
    for (transport, addr) in handle.local_addrs().into_iter() {
        println!("Running {transport} server on {addr}");
    }

    handle.join().await?;
    Ok(())
}
