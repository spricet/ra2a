#[derive(Debug, Clone)]
pub enum A2AClient {
    #[cfg(feature = "grpc")]
    Grpc(crate::client::grpc::A2AGrpcClient),
}
