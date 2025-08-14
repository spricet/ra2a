#[derive(Debug, Clone)]
pub enum A2AServer {
    #[cfg(feature = "grpc")]
    Grpc(crate::server::grpc::A2AGrpcServer),
}
