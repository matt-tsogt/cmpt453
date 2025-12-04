// src/grpc_tests/server_stream_test.rs

use std::time::{Duration, Instant};
use tokio::task::JoinHandle;
use tonic::Request;

use crate::scenario::{Scenario, NUM_WORKERS};
use crate::routes::grpc_endpoint;

use main::service::{
    SimpleRequest,
    simple_service_client::SimpleServiceClient,
};

pub async fn server_stream_test(
    s: &Scenario,
) -> Result<Duration, Box<dyn std::error::Error>> {
    let mut client = SimpleServiceClient::connect(grpc_endpoint()).await?;

    let payload = if s.payload_size_bytes > 0 {
        Some("x".repeat(s.payload_size_bytes))
    } else {
        None
    };

    let requests_per_worker = s.requests_per_worker;

    let start = Instant::now();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for worker_id in 0..NUM_WORKERS {
        let mut client_clone = client.clone();
        let payload_clone = payload.clone();

        let handle = tokio::spawn(async move {
            for i in 0..requests_per_worker {
                let base_msg = format!("worker {} - request {}", worker_id, i);

                let full_msg = match &payload_clone {
                    Some(p) => format!("{base_msg} - {p}"),
                    None => base_msg,
                };

                let request = Request::new(SimpleRequest { message: full_msg });

                if let Ok(response) = client_clone.simple_server_stream(request).await {
                    let mut stream = response.into_inner();
                    while let Ok(Some(_msg)) = stream.message().await {}
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
