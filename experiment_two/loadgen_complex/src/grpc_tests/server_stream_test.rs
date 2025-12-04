// src/grpc_tests/server_stream_test.rs

use std::time::{Duration, Instant};
use tokio::task::JoinHandle;
use tonic::Request;

use crate::scenario::{Scenario, NUM_WORKERS};
use crate::routes::grpc_endpoint;
use crate::types::make_complex_request;

use main::service::{
    ComplexRequest,
    complex_service_client::ComplexServiceClient,
};

pub async fn server_stream_test(
    s: &Scenario,
) -> Result<Duration, Box<dyn std::error::Error>> {
    let mut client = ComplexServiceClient::connect(grpc_endpoint()).await?;

    let num_readings = s.payload_size_bytes.max(1);
    let requests_per_worker = s.requests_per_worker;

    let start = Instant::now();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for worker_id in 0..NUM_WORKERS {
        let mut client_clone = client.clone();
        let scenario_name = s.name.to_string();

        let handle = tokio::spawn(async move {
            for i in 0..requests_per_worker {
                let label = format!("{}-worker-{}-req-{}", scenario_name, worker_id, i);
                let req: ComplexRequest = make_complex_request(&label, num_readings);

                let request = Request::new(req);

                if let Ok(response) = client_clone.complex_server_stream(request).await {
                    let mut stream = response.into_inner();
                    while let Ok(Some(_msg)) = stream.message().await {
                        // just drain the stream
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(start.elapsed())
}
