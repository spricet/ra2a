use crate::agent::A2ADelegate;
use crate::core::Transport;
use crate::server::A2AServerError;
use std::net::SocketAddr;
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone)]
pub struct A2AServer {
    delegate: A2ADelegate,
    #[cfg(feature = "grpc")]
    grpc: Option<crate::server::grpc::A2AGrpcServer>,
    jsonrpc: Option<crate::server::jsonrpc::A2AJsonRpcServer>,
}

impl A2AServer {
    pub fn new(delegate: A2ADelegate) -> Self {
        A2AServer {
            delegate,
            #[cfg(feature = "grpc")]
            grpc: None,
            jsonrpc: None,
        }
    }

    pub fn with_jsonrpc(mut self, addr: SocketAddr) -> Self {
        self.jsonrpc = Some(crate::server::jsonrpc::A2AJsonRpcServer::new(
            addr,
            self.delegate.clone(),
        ));
        self
    }

    #[cfg(feature = "grpc")]
    pub fn with_grpc(mut self, addr: SocketAddr) -> Self {
        self.grpc = Some(crate::server::grpc::A2AGrpcServer::new(
            addr,
            self.delegate.clone(),
        ));
        self
    }

    pub fn enabled_transports(&self) -> Vec<Transport> {
        let mut transports = Vec::new();
        #[cfg(feature = "grpc")]
        if self.grpc.is_some() {
            transports.push(Transport::Grpc);
        }
        if self.jsonrpc.is_some() {
            transports.push(Transport::JsonRpc);
        }
        transports
    }

    pub async fn serve_with_shutdown<F: Future<Output=()>>(
        self,
        signal: F,
    ) -> Result<(), A2AServerError> {
        let (tx, _rx) = tokio::sync::broadcast::channel(1);
        tokio::select! {
            res = self.serve_all(&tx) => res,
            _ = signal => {
                let _ = tx.send(());
                Ok(())
            }
        }
    }

    async fn serve_all(&self, tx: &Sender<()>) -> Result<(), A2AServerError> {
        let futs = self
            .enabled_transports()
            .into_iter()
            .map(|t| self.serve_transport(t, tx));
        futures::future::try_join_all(futs).await.map(|_| ())
    }

    async fn serve_grpc(&self, tx: &Sender<()>) -> Result<(), A2AServerError> {
        #[cfg(feature = "grpc")]
        if let Some(grpc) = &self.grpc {
            let mut rx = tx.subscribe();
            grpc.serve(async { rx.recv().await.unwrap_or_default() })
                .await?;
        }
        Ok(())
    }

    async fn serve_jsonrpc(&self, tx: &Sender<()>) -> Result<(), A2AServerError> {
        if let Some(jsonrpc) = &self.jsonrpc {
            let mut rx = tx.subscribe();
            jsonrpc
                .serve(async { rx.recv().await.unwrap_or_default() })
                .await?;
        }
        Ok(())
    }

    async fn serve_transport(
        &self,
        transport: Transport,
        tx: &Sender<()>,
    ) -> Result<(), A2AServerError> {
        match transport {
            Transport::Grpc => self.serve_grpc(tx).await,
            Transport::JsonRpc => self.serve_jsonrpc(tx).await,
        }
    }
}
