use jsonrpsee::server::RegisterMethodError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum A2AServerError {
    #[cfg(feature = "grpc")]
    #[error("Grpc transport")]
    Grpc(#[from] tonic::transport::Error),

    #[error("io")]
    Io(#[from] std::io::Error),

    #[error("register method")]
    RegisterMethod(#[from] RegisterMethodError),
}

#[derive(Debug, Error)]
pub enum A2AServerBuilderError {
    #[error("No servers provided")]
    EmptyServers,
}
