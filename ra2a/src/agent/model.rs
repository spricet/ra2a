use crate::agent::{A2ADelegate, AgentBuilderError, AgentHandler};
use crate::core::{A2AError, Transport};
use crate::server::{A2AServer, A2AServerError};
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
    pub async fn serve_with_shutdown<F: Future<Output = ()>>(
        self,
        signal: F,
    ) -> Result<(), crate::server::A2AServerError> {
        self.server.serve_with_shutdown(signal).await
    }

    pub fn start_server(&self) -> AgentServerHandle {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let server = self.server.clone();

        let handle: JoinHandle<Result<(), A2AError>> = tokio::spawn(async move {
            // Treat either "sent ()" or "sender dropped" as a shutdown signal
            let shutdown = async move {
                let _ = rx.await;
            };
            server
                .serve_with_shutdown(shutdown)
                .await
                .map_err(A2AError::from)
        });

        AgentServerHandle {
            tx: Some(tx),
            handle: Some(handle),
        }
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
            grpc_socket: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_json_rpc_server(mut self, addr: impl Into<SocketAddr>) -> Self {
        self.json_rpc_socket = Some(addr.into());
        self
    }

    #[cfg(feature = "grpc")]
    pub fn with_grpc_server(mut self, addr: impl Into<SocketAddr>) -> Self {
        self.grpc_socket = Some(addr.into());
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
