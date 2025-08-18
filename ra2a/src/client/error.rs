use crate::client::jsonrpc::A2AJsonRpcClientError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum A2AClientError {
    #[cfg(feature = "grpc")]
    #[error("grpc")]
    Grpc(#[from] crate::client::grpc::A2AGrpcClientError),

    #[error("json-rpc")]
    JsonRpc(#[from] A2AJsonRpcClientError),
}
