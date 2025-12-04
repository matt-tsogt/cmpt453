use std::time::{Duration, Instant};
use tokio::task::JoinHandle;

use crate::scenario::{Scenario, NUM_WORKERS};
use crate::routes::rest_endpoint;
use crate::types::make_complex_request_json;

pub async fn rest_complex_test(
    s: &Scenario,
) -> Result<Duration, Box<dyn std::error::Error>> {
    println!("Running REST COMPLEX scenario: {}", s.name);

    let client = reqwest::Client::new();

    let num_readings = s.payload_size_bytes.max(1);
    let scenario_name = s.name.to_string();
    let requests_per_worker = s.requests_per_worker;

    let base_endpoint = rest_endpoint();
    let endpoint = format!("{}/complex", base_endpoint.trim_end_matches('/'));

    let start = Instant::now();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for worker_id in 0..NUM_WORKERS {
        let client_clone = client.clone();
        let endpoint = endpoint.clone();
        let scenario_name = scenario_name.clone();

        let handle = tokio::spawn(async move {
            for i in 0..requests_per_worker {
                let label = format!("{}-worker-{}-req-{}", scenario_name, worker_id, i);
                let req_body = make_complex_request_json(&label, num_readings);

                let resp = client_clone.post(&endpoint).json(&req_body).send().await;

                match resp {
                    Ok(r) => {
                        // 🔧 don't try to decode into a struct; just drain the body
                        if let Err(e) = r.bytes().await {
                            eprintln!(
                                "[REST COMPLEX worker {}] failed to read body in scenario '{}': {}",
                                worker_id, scenario_name, e
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "[REST COMPLEX worker {}] request {} failed in scenario '{}': {}",
                            worker_id, i, scenario_name, e
                        );
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
