pub mod complex_ping;
pub mod complex_client_stream;
pub mod complex_server_stream;
pub mod complex_stream;

use std::pin::Pin;
use futures_core::Stream;
use tonic::{Request, Response, Status, Streaming};

use main::service::{
    ComplexRequest,
    ComplexResponse,
    complex_service_server::ComplexService,
};

pub type ServerStream =
    Pin<Box<dyn Stream<Item = Result<ComplexResponse, Status>> + Send + 'static>>;

pub struct ComplexServiceImpl;

#[tonic::async_trait]
impl ComplexService for ComplexServiceImpl {
    type ComplexServerStreamStream = ServerStream;
    type ComplexStreamStream = ServerStream;

    async fn complex_ping(
        &self,
        request: Request<ComplexRequest>,
    ) -> Result<Response<ComplexResponse>, Status> {
        complex_ping::handle(request).await
    }

    async fn complex_client_stream(
        &self,
        request: Request<Streaming<ComplexRequest>>,
    ) -> Result<Response<ComplexResponse>, Status> {
        complex_client_stream::handle(request).await
    }

    async fn complex_server_stream(
        &self,
        request: Request<ComplexRequest>,
    ) -> Result<Response<Self::ComplexServerStreamStream>, Status> {
        complex_server_stream::handle(request).await
    }

    async fn complex_stream(
        &self,
        request: Request<Streaming<ComplexRequest>>,
    ) -> Result<Response<Self::ComplexStreamStream>, Status> {
        complex_stream::handle(request).await
    }
}
