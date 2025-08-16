use thiserror::Error;

#[derive(Debug, Error)]
pub enum A2AAgentError {}

#[derive(Debug, Error)]
pub enum AgentBuilderError {
    #[error("Name is required")]
    MissingName,
}
