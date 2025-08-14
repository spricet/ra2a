use thiserror::Error;

#[derive(Debug, Error)]
pub enum A2AJsonRpcClientError {
    #[error("Json RPC client error")]
    JsonRpc(#[from] jsonrpsee::core::ClientError),
}
