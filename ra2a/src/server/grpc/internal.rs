use crate::core::message::{SendMessageRequest, SendMessageResponse};
use crate::core::{A2A, GRPC_SEND_MESSAGE_PATH, GRPC_SERVICE_NAME};
use crate::server::grpc::A2AGrpcDelegate;
use http::{Request as HttpRequest, Response as HttpResponse};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use tonic::body::Body;
use tonic::codegen::Service;
use tonic::{
    Request, Response, Status,
    codec::CompressionEncoding,
    server::{Grpc, NamedService, UnaryService},
};
use tonic_prost::ProstCodec;

const DELEGATE: A2AGrpcDelegate = A2AGrpcDelegate;

#[derive(Debug, Clone, Default)]
pub struct A2AGrpc;

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
        Box::pin(async move {
            match req.uri().path() {
                GRPC_SEND_MESSAGE_PATH => {
                    let mut grpc =
                        Grpc::new(ProstCodec::<SendMessageResponse, SendMessageRequest>::default())
                            .accept_compressed(CompressionEncoding::Gzip)
                            .send_compressed(CompressionEncoding::Gzip)
                            .max_decoding_message_size(4 * 1024 * 1024)
                            .max_encoding_message_size(4 * 1024 * 1024);
                    let svc = SendMessage::default();
                    let res = grpc.unary(svc, req).await;
                    Ok(res)
                }
                _ => Ok(Status::unimplemented("unknown method").into_http()),
            }
        })
    }
}

type BoxFut<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

#[derive(Debug, Clone, Default)]
pub struct SendMessage;
impl UnaryService<SendMessageRequest> for SendMessage {
    type Response = SendMessageResponse;
    type Future = BoxFut<Result<Response<Self::Response>, Status>>;

    fn call(&mut self, request: Request<SendMessageRequest>) -> Self::Future {
        let req = request.into_inner();
        Box::pin(async move {
            let res = DELEGATE.send_message(req).await;
            match res {
                Ok(response) => Ok(Response::new(response)),
                Err(e) => Err(Status::internal(e.to_string())),
            }
        })
    }
}
