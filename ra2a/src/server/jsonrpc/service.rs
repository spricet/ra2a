use crate::core::{A2A, JSONRPC_SEND_MESSAGE_METHOD};
use crate::server::delegate::A2ADelegate;
use crate::server::A2AServerError;
use jsonrpsee::server::Server;
use jsonrpsee::types::ErrorObjectOwned;
use jsonrpsee::RpcModule;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct A2AJsonRpcServer {
    addr: SocketAddr,
    delegate: A2ADelegate,
}

impl A2AJsonRpcServer {
    pub fn new(addr: SocketAddr, delegate: A2ADelegate) -> Self {
        Self { addr, delegate }
    }

    pub async fn serve<F: Future<Output=()>>(&self, signal: F) -> Result<(), A2AServerError> {
        let server = Server::builder().build(self.addr).await?;

        let mut module = RpcModule::new(self.delegate.clone());
        module.register_async_method(
            JSONRPC_SEND_MESSAGE_METHOD,
            |params, ctx, _| async move {
                let request = params.parse()?;
                let res = ctx.send_message(request).await.unwrap(); // todo handle these errors!!!
                Ok::<_, ErrorObjectOwned>(res)
            },
        )?;
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
