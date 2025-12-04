// src/grpc_tests/client_stream_test.rs

use std::time::{Duration, Instant};
use tokio::task::JoinHandle;
use async_stream::stream;

use crate::scenario::{Scenario, NUM_WORKERS};
use crate::routes::grpc_endpoint;

use main::service::{
    SimpleRequest,
    simple_service_client::SimpleServiceClient,
};

pub async fn client_stream_test(
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
            let outbound = stream! {
                for i in 0..requests_per_worker {
                    let base_msg = format!("worker {} - request {}", worker_id, i);

                    let full_msg = match &payload_clone {
                        Some(p) => format!("{base_msg} - {p}"),
                        None => base_msg,
                    };

                    yield SimpleRequest { message: full_msg };
                }
            };

            let _ = client_clone.simple_client_stream(outbound).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(start.elapsed())
}
