use tonic::transport::Server;

use main::service::simple_service_server::SimpleServiceServer;

mod rpc_calls;
use rpc_calls::SimpleServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "0.0.0.0:8080".parse()?;
    println!("gRPC server is listening on: {}", address);
    let service = SimpleServiceImpl;

    Server::builder()
        .add_service(SimpleServiceServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}
