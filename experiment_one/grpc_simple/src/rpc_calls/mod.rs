pub mod simple_ping;
pub mod simple_client_stream;
pub mod simple_server_stream;
pub mod simple_stream;
pub mod util;

use std::pin::Pin;
use futures_core::Stream;
use tonic::{Request, Response, Status, Streaming};

use main::service::{
    SimpleRequest,
    SimpleResponse,
    simple_service_server::SimpleService,
};

pub struct SimpleServiceImpl;

pub type ServerStream = Pin<Box<dyn Stream<Item = Result<SimpleResponse, Status>> + Send + 'static>>;

pub type BiStream = ServerStream;

#[tonic::async_trait]
impl SimpleService for SimpleServiceImpl {
    
    type SimpleServerStreamStream = ServerStream;
    type SimpleStreamStream = BiStream;

    async fn simple_ping(&self, request: Request<SimpleRequest>) -> Result<Response<SimpleResponse>, Status> {
        simple_ping::handle(request).await
    }

    async fn simple_client_stream(&self, request: Request<Streaming<SimpleRequest>>) -> Result<Response<SimpleResponse>, Status> {
        simple_client_stream::handle(request).await
    }

    async fn simple_server_stream(&self, request: Request<SimpleRequest>) -> Result<Response<Self::SimpleServerStreamStream>, Status> {
        simple_server_stream::handle(request).await
    }

    async fn simple_stream(&self, request: Request<Streaming<SimpleRequest>>) -> Result<Response<Self::SimpleStreamStream>, Status> {
        simple_stream::handle(request).await
    }
}
