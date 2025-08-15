use crate::server::A2AServerError;
use crate::server::grpc::A2AGrpc;
use std::net::SocketAddr;
use tonic::transport::Server;

#[derive(Debug, Clone)]
pub struct A2AGrpcServer {
    addr: SocketAddr,
}

impl A2AGrpcServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn serve<F: Future<Output = ()>>(&self, signal: F) -> Result<(), A2AServerError> {
        Server::builder()
            .add_service(A2AGrpc)
            .serve_with_shutdown(self.addr, signal)
            .await?;
        Ok(())
    }
}
