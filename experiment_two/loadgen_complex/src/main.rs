mod grpc_tests;
mod rest_test;
mod routes;
mod scenario;
mod types;

use scenario::{SCENARIOS, print_stats};
use rest_test::rest_complex_test;
use grpc_tests::{unary_test, client_stream_test, server_stream_test, stream_test};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== COMPLEX payload tests ===");

    for scenario in SCENARIOS.iter() {
        let t = rest_complex_test(scenario).await?;
        print_stats("REST COMPLEX", scenario, t);

        let t = unary_test(scenario).await?;
        print_stats("gRPC COMPLEX UNARY", scenario, t);

        let t = client_stream_test(scenario).await?;
        print_stats("gRPC COMPLEX CLIENT STREAM", scenario, t);

        let t = server_stream_test(scenario).await?;
        print_stats("gRPC COMPLEX SERVER STREAM", scenario, t);

        let t = stream_test(scenario).await?;
        print_stats("gRPC COMPLEX BIDI STREAM", scenario, t);
    }

    Ok(())
}
