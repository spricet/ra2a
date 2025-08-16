use crate::agent::A2ADelegate;
use crate::server::A2AServerError;
use crate::server::grpc::A2AGrpc;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tonic::codegen::tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

#[derive(Debug, Clone)]
pub struct A2AGrpcServer {
    bind_addr: SocketAddr,
    delegate: A2ADelegate,
}

impl A2AGrpcServer {
    pub fn new(bind_addr: SocketAddr, delegate: A2ADelegate) -> Self {
        Self {
            bind_addr,
            delegate,
        }
    }

    pub async fn bind(&self) -> Result<TcpListener, A2AServerError> {
        TcpListener::bind(self.bind_addr)
            .await
            .map_err(A2AServerError::from)
    }

    pub async fn serve<F: Future<Output = ()>>(
        &self,
        signal: F,
        listener: TcpListener,
    ) -> Result<(), A2AServerError> {
        Server::builder()
            .add_service(A2AGrpc {
                delegate: self.delegate.clone(),
            })
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), signal)
            .await?;
        Ok(())
    }
}
