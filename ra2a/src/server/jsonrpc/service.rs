use crate::agent::A2ADelegate;
use crate::core::{A2A, JSONRPC_SEND_MESSAGE_METHOD};
use crate::server::A2AServerError;
use jsonrpsee::RpcModule;
use jsonrpsee::server::Server;
use jsonrpsee::types::ErrorObjectOwned;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
pub struct A2AJsonRpcServer {
    bind_addr: SocketAddr,
    delegate: A2ADelegate,
}

impl A2AJsonRpcServer {
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
        // hand off to jsonrpsee as std listener
        let std_listener = listener.into_std()?;
        // ensure non-blocking for hyper/jsonrpsee
        std_listener.set_nonblocking(true)?;
        let server = Server::builder().build_from_tcp(std_listener)?;

        let mut module = RpcModule::new(self.delegate.clone());
        module.register_async_method(JSONRPC_SEND_MESSAGE_METHOD, |params, ctx, _| async move {
            let request = params.parse()?;
            let res = ctx.send_message(request).await.unwrap(); // todo handle these errors!!!
            Ok::<_, ErrorObjectOwned>(res)
        })?;
        let handle = server.start(module);

        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                let _ = handle.stop();
            }
            _ = signal => {
                let _ = handle.stop();
            }
            _ = handle.clone().stopped() => {} // server finished on its own
        }

        Ok(())
    }
}
