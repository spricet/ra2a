use crate::agent::{A2ADelegate, AgentBuilderError, AgentHandler};
use crate::core::{A2AError, Transport};
use crate::server::{A2AServer, A2AServerError};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct Agent<A: AgentHandler + 'static> {
    name: String,
    handler: Arc<A>,
    server: A2AServer,
}

impl<A: AgentHandler + 'static> Agent<A> {
    /// Starts the agent server with the configured transports that responds to requests in the A2A protocol.
    pub async fn start_server(&self) -> Result<AgentServerHandle, A2AError> {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let server = self.server.clone();

        let mut local_addrs = HashMap::new();
        let (grpc_listener, jrpc_listener) = server.bind_all().await?;
        let mut transports = vec![];
        if let Some(listener) = grpc_listener {
            local_addrs.insert(
                Transport::Grpc,
                listener.local_addr().map_err(A2AServerError::from)?,
            );
            transports.push((Transport::Grpc, listener));
        }
        if let Some(listener) = jrpc_listener {
            local_addrs.insert(
                Transport::JsonRpc,
                listener.local_addr().map_err(A2AServerError::from)?,
            );
            transports.push((Transport::JsonRpc, listener));
        }

        let handle: JoinHandle<Result<(), A2AError>> = tokio::spawn(async move {
            // Treat either "sent ()" or "sender dropped" as a shutdown signal
            let shutdown = async move {
                let _ = rx.await;
            };

            server
                .serve_with_shutdown(transports, shutdown)
                .await
                .map_err(A2AError::from)
        });

        Ok(AgentServerHandle {
            tx: Some(tx),
            handle: Some(handle),
            local_addrs,
        })
    }

    /// Returns the supported transports for the agent.
    pub fn supported_transports(&self) -> Vec<Transport> {
        self.server.enabled_transports()
    }
}

#[derive(Debug)]
pub struct AgentServerHandle {
    tx: Option<tokio::sync::oneshot::Sender<()>>,
    handle: Option<JoinHandle<Result<(), A2AError>>>,
    local_addrs: HashMap<Transport, SocketAddr>,
}

impl AgentServerHandle {
    /// Ask the server to stop (graceful).
    pub async fn shutdown(mut self) -> Result<(), A2AError> {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(()); // ignore if receiver already gone
        }
        self.join().await
    }

    /// Wait for the server task to finish and get its result.
    pub async fn join(mut self) -> Result<(), A2AError> {
        self.handle
            .take()
            .unwrap()
            .await
            .unwrap_or_else(|e| Err(A2AError::from(A2AServerError::Join(e))))
    }

    pub fn local_addr(&self, transport: Transport) -> Option<SocketAddr> {
        self.local_addrs.get(&transport).cloned()
    }

    pub fn local_addrs(&self) -> Vec<(Transport, SocketAddr)> {
        self.local_addrs.iter().map(|(k, v)| (*k, *v)).collect()
    }
}

impl Drop for AgentServerHandle {
    fn drop(&mut self) {
        // Try to stop gracefully; if user forgets to call shutdown/join, we still clean up.
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(());
        }
        if let Some(h) = self.handle.take() {
            h.abort(); // best-effort cancellation if it's still running
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AgentBuilder<A: AgentHandler + 'static> {
    pub handler: Arc<A>,
    pub name: Option<String>,
    pub json_rpc_socket: Option<SocketAddr>,
    #[cfg(feature = "grpc")]
    pub grpc_socket: Option<SocketAddr>,
}

impl<A: AgentHandler + 'static> AgentBuilder<A> {
    pub fn new(handler: A) -> Self {
        Self {
            handler: Arc::new(handler),
            name: None,
            json_rpc_socket: None,
            #[cfg(feature = "grpc")]
            grpc_socket: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_json_rpc_server(mut self, addr: SocketAddr) -> Self {
        self.json_rpc_socket = Some(addr);
        self
    }

    #[cfg(feature = "grpc")]
    pub fn with_grpc_server(mut self, addr: SocketAddr) -> Self {
        self.grpc_socket = Some(addr);
        self
    }

    pub fn build(self) -> Result<Agent<A>, AgentBuilderError> {
        let name = match self.name {
            Some(name) => name,
            None => return Err(AgentBuilderError::MissingName),
        };

        let delegate = A2ADelegate::new(self.handler.clone());
        let mut server = A2AServer::new(delegate);
        if let Some(addr) = self.json_rpc_socket {
            server = server.with_jsonrpc(addr);
        }
        #[cfg(feature = "grpc")]
        if let Some(addr) = self.grpc_socket {
            server = server.with_grpc(addr);
        }

        Ok(Agent {
            name,
            handler: self.handler,
            server,
        })
    }
}
