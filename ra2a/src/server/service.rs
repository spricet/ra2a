use crate::agent::A2ADelegate;
use crate::core::Transport;
use crate::server::A2AServerError;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone)]
pub struct A2AServer {
    delegate: A2ADelegate,
    #[cfg(feature = "grpc")]
    grpc: Option<crate::server::grpc::A2AGrpcServer>,
    jsonrpc: Option<crate::server::jsonrpc::A2AJsonRpcServer>,
    local_addrs: Arc<Mutex<HashMap<Transport, SocketAddr>>>,
}

impl A2AServer {
    pub fn new(delegate: A2ADelegate) -> Self {
        A2AServer {
            delegate,
            #[cfg(feature = "grpc")]
            grpc: None,
            jsonrpc: None,
            local_addrs: Arc::new(Mutex::new(HashMap::new())),
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

    pub async fn local_addr(&self, transport: Transport) -> Option<SocketAddr> {
        self.local_addrs.lock().await.get(&transport).cloned()
    }

    pub async fn bind_all(
        &self,
    ) -> Result<(Option<TcpListener>, Option<TcpListener>), A2AServerError> {
        #[cfg(not(feature = "grpc"))]
        let grpc_listener: Option<TcpListener> = None;
        #[cfg(feature = "grpc")]
        let grpc_listener = match &self.grpc {
            Some(grpc) => Some(grpc.bind().await?),
            None => None,
        };
        let jrpc_listener = match &self.jsonrpc {
            Some(jrpc) => Some(jrpc.bind().await?),
            None => None,
        };
        Ok((grpc_listener, jrpc_listener))
    }

    pub async fn serve_with_shutdown<F: Future<Output = ()>>(
        &self,
        listeners: Vec<(Transport, TcpListener)>,
        signal: F,
    ) -> Result<(), A2AServerError> {
        let (tx, _rx) = tokio::sync::broadcast::channel(1);
        tokio::select! {
            res = self.serve_all(listeners, &tx) => res,
            _ = signal => {
                let _ = tx.send(());
                Ok(())
            }
        }
    }

    async fn serve_all(
        &self,
        listeners: Vec<(Transport, TcpListener)>,
        tx: &Sender<()>,
    ) -> Result<(), A2AServerError> {
        let futs = listeners
            .into_iter()
            .map(|(t, listener)| self.serve_transport(t, listener, tx));
        futures::future::try_join_all(futs).await.map(|_| ())
    }

    async fn serve_grpc(
        &self,
        listener: TcpListener,
        tx: &Sender<()>,
    ) -> Result<(), A2AServerError> {
        #[cfg(feature = "grpc")]
        if let Some(grpc) = &self.grpc {
            self.local_addrs
                .lock()
                .await
                .insert(Transport::Grpc, listener.local_addr()?);
            let mut rx = tx.subscribe();
            grpc.serve(async { rx.recv().await.unwrap_or_default() }, listener)
                .await?;
        }
        Ok(())
    }

    async fn serve_jsonrpc(
        &self,
        listener: TcpListener,
        tx: &Sender<()>,
    ) -> Result<(), A2AServerError> {
        if let Some(jsonrpc) = &self.jsonrpc {
            self.local_addrs
                .lock()
                .await
                .insert(Transport::JsonRpc, listener.local_addr()?);
            let mut rx = tx.subscribe();
            jsonrpc
                .serve(async { rx.recv().await.unwrap_or_default() }, listener)
                .await?;
        }
        Ok(())
    }

    async fn serve_transport(
        &self,
        transport: Transport,
        listener: TcpListener,
        tx: &Sender<()>,
    ) -> Result<(), A2AServerError> {
        match transport {
            Transport::Grpc => self.serve_grpc(listener, tx).await,
            Transport::JsonRpc => self.serve_jsonrpc(listener, tx).await,
        }
    }
}
