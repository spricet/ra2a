use jsonrpsee::server::RegisterMethodError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum A2AServerError {
    #[cfg(feature = "grpc")]
    #[error("Grpc transport")]
    Grpc(#[from] tonic::transport::Error),

    #[error("Io")]
    Io(#[from] std::io::Error),

    #[error("Register method")]
    RegisterMethod(#[from] RegisterMethodError),

    #[error("Task join error")]
    Join(#[from] tokio::task::JoinError),
}

#[derive(Debug, Error)]
pub enum A2AServerBuilderError {
    #[error("No servers provided")]
    EmptyServers,
}
