use crate::core::{A2A, JSONRPC_SEND_MESSAGE_METHOD};
use crate::server::delegate::A2AGrpcDelegate;
use jsonrpsee::server::Server;
use jsonrpsee::types::ErrorObjectOwned;
use jsonrpsee::RpcModule;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct A2AJsonRpcServer {}

impl A2AJsonRpcServer {
    pub async fn serve<F: Future<Output=()>>(&self, signal: F) -> Result<(), String> {
        let addr = "127.0.0.1:50123".parse::<SocketAddr>().unwrap();
        let server = Server::builder().build(addr).await.unwrap();

        let mut module = RpcModule::new(());
        module
            .register_async_method(JSONRPC_SEND_MESSAGE_METHOD, |params, _ctx, _| async move {
                let request = params.parse().unwrap();
                let res = A2AGrpcDelegate
                    .send_message(request)
                    .await
                    .unwrap();
                Ok::<_, ErrorObjectOwned>(res)
            })
            .unwrap();
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
