use tonic::transport::Server;
use main::service::complex_service_server::ComplexServiceServer;

mod util;
mod rpc_calls;

use rpc_calls::ComplexServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8081".parse()?;   
    println!("gRPC COMPLEX server is listening on: {}", addr);

    let service = ComplexServiceImpl;

    Server::builder()
        .add_service(ComplexServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
