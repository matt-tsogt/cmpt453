use tonic::{transport::Server, Request, Response, Status};
use std::time::{SystemTime, UNIX_EPOCH};

use main::service::{
    PingRequest,
    PingResponse,
    service_server::Service,
    service_server::ServiceServer,
};


#[derive(Default)]
pub struct ServiceImpl;

#[tonic::async_trait]
impl Service for ServiceImpl {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {

        let reply = PingResponse {
            message: format!("Pong: {}", request.into_inner().message),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "0.0.0.0:8080".parse()?;

    println!("gRPC server starting on: {}", address);

    let service = ServiceImpl::default();

    Server::builder()
        .add_service(ServiceServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}
