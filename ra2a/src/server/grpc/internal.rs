use crate::core::message::{SendMessageRequest, SendMessageResponse};
use crate::core::{
    A2A, A2AError, A2AProtocolError, GRPC_GET_TASK_PATH, GRPC_SEND_MESSAGE_PATH, GRPC_SERVICE_NAME,
};
use http::{Request as HttpRequest, Response as HttpResponse};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::agent::A2ADelegate;
use crate::core::task::{GetTaskGrpcRequest, Task};
use tonic::body::Body;
use tonic::codec::CompressionEncoding;
use tonic::codegen::Service;
use tonic::{
    Request, Response, Status,
    server::{Grpc, NamedService, UnaryService},
};
use tonic_prost::ProstCodec;

#[derive(Debug, Clone)]
pub struct A2AGrpc {
    pub(crate) delegate: A2ADelegate,
}

impl NamedService for A2AGrpc {
    const NAME: &'static str = GRPC_SERVICE_NAME;
}

impl Service<HttpRequest<Body>> for A2AGrpc {
    type Response = HttpResponse<Body>;
    type Error = Infallible;
    type Future = BoxFut<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: HttpRequest<Body>) -> Self::Future {
        let delegate = self.delegate.clone();
        Box::pin(async move {
            match req.uri().path() {
                GRPC_SEND_MESSAGE_PATH => {
                    let mut grpc =
                        Grpc::new(ProstCodec::<SendMessageResponse, SendMessageRequest>::default())
                            .accept_compressed(CompressionEncoding::Gzip)
                            .send_compressed(CompressionEncoding::Gzip)
                            .max_decoding_message_size(4 * 1024 * 1024)
                            .max_encoding_message_size(4 * 1024 * 1024);
                    let svc = SendMessage { delegate };
                    let res = grpc.unary(svc, req).await;
                    Ok(res)
                }
                GRPC_GET_TASK_PATH => {
                    // todo clean up this and expose tuning
                    let mut grpc = Grpc::new(ProstCodec::<Task, GetTaskGrpcRequest>::default())
                        .accept_compressed(CompressionEncoding::Gzip)
                        .send_compressed(CompressionEncoding::Gzip)
                        .max_decoding_message_size(4 * 1024 * 1024)
                        .max_encoding_message_size(4 * 1024 * 1024);
                    let svc = GetTask { delegate };
                    let res = grpc.unary(svc, req).await;
                    Ok(res)
                }
                _ => Ok(Status::unimplemented("unknown method").into_http()),
            }
        })
    }
}

type BoxFut<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

#[derive(Debug, Clone)]
pub struct SendMessage {
    delegate: A2ADelegate,
}

impl UnaryService<SendMessageRequest> for SendMessage {
    type Response = SendMessageResponse;
    type Future = BoxFut<Result<Response<Self::Response>, Status>>;

    fn call(&mut self, request: Request<SendMessageRequest>) -> Self::Future {
        let req = request.into_inner();
        let delegate = self.delegate.clone();
        Box::pin(async move {
            let res = delegate.send_message(req).await;
            match res {
                Ok(response) => Ok(Response::new(response)),
                Err(e) => Err(Status::internal(e.to_string())),
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetTask {
    delegate: A2ADelegate,
}

impl UnaryService<GetTaskGrpcRequest> for GetTask {
    type Response = Task;
    type Future = BoxFut<Result<Response<Self::Response>, Status>>;

    fn call(&mut self, request: Request<GetTaskGrpcRequest>) -> Self::Future {
        let req = request.into_inner();
        let delegate = self.delegate.clone();
        Box::pin(async move {
            let res = delegate.get_task(req.into()).await;
            match res {
                Ok(response) => Ok(Response::new(response)),
                Err(A2AError::Protocol(A2AProtocolError::TaskNotFound { id, code })) => {
                    let mut status = Status::not_found(format!("Task not found: {}", id));
                    status.metadata_mut().insert("code", code.into());
                    Err(status)
                }
                Err(e) => Err(Status::internal(e.to_string())),
            }
        })
    }
}
