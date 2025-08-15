use crate::server::grpc::A2AGrpc;
use tonic::transport::Server;

#[derive(Debug, Clone)]
pub struct A2AGrpcServer {}

impl A2AGrpcServer {
    pub async fn serve<F: Future<Output=()>>(&self, signal: F) -> Result<(), String> {
        Server::builder()
            .add_service(A2AGrpc)
            .serve_with_shutdown("0.0.0.0:50051".parse().unwrap(), signal)
            .await
            .unwrap();
        Ok(())
    }
}
