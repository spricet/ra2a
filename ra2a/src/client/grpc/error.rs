use http::uri::InvalidUri;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum A2AGrpcClientError {
    #[error("Invalid URI")]
    InvalidUri(#[from] InvalidUri),

    #[cfg(feature = "grpc")]
    #[error("Transport")]
    Transport(#[from] tonic::transport::Error),

    #[error("Status")]
    Status(#[from] tonic::Status),
}
