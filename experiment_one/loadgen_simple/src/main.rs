mod grpc_tests;
mod rest_test;
mod routes;
mod scenario;
mod types;

use scenario::{SCENARIOS, print_stats};
use rest_test::rest_test;
use grpc_tests::{unary_test, stream_test, client_stream_test, server_stream_test};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SIMPLE payload tests ===");

    for scenario in SCENARIOS.iter() {
        let t = rest_test(scenario).await?;
        print_stats("REST SIMPLE", scenario, t);

        let t = unary_test(scenario).await?;
        print_stats("gRPC SIMPLE UNARY", scenario, t);

        let t = client_stream_test(scenario).await?;
        print_stats("gRPC SIMPLE CLIENT STREAM", scenario, t);

        let t = server_stream_test(scenario).await?;
        print_stats("gRPC SIMPLE SERVER STREAM", scenario, t);

        let t = stream_test(scenario).await?;
        print_stats("gRPC SIMPLE BIDI STREAM", scenario, t);
    }

    Ok(())
}
