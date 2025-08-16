use crate::agent::{A2ADelegate, AgentBuilderError, AgentHandler};
use crate::server::A2AServer;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Agent<A: AgentHandler + 'static> {
    name: String,
    handler: Arc<A>,
    server: A2AServer,
}

impl<A: AgentHandler + 'static> Agent<A> {
    pub async fn serve_with_shutdown<F: Future<Output = ()>>(
        self,
        signal: F,
    ) -> Result<(), crate::server::A2AServerError> {
        self.server.serve_with_shutdown(signal).await
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
