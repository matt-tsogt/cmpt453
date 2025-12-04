use std::time::{Duration, Instant};
use tokio::task::JoinHandle;
use reqwest;

use crate::scenario::{Scenario, NUM_WORKERS};
use crate::routes::rest_endpoint;
use crate::types::{SimpleRequestJson, SimpleResponseJson};


pub async fn rest_test(s: &Scenario) -> Result<Duration, Box<dyn std::error::Error>> {
    println!("Running REST SIMPLE scenario: {}", s.name);

    let client = reqwest::Client::new();

    let payload = if s.payload_size_bytes > 0 {
        Some("x".repeat(s.payload_size_bytes))
    } else {
        None
    };

    let scenario_name = s.name;
    let requests_per_worker = s.requests_per_worker;

    let base_endpoint = rest_endpoint();
    let endpoint = format!("{}/simple", base_endpoint.trim_end_matches('/'));

    let start = Instant::now();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for worker_id in 0..NUM_WORKERS {
        let client_clone = client.clone();
        let payload_clone = payload.clone();
        let scenario_name = scenario_name;
        let requests_per_worker = requests_per_worker;
        let endpoint = endpoint.clone();

        let handle = tokio::spawn(async move {
            for i in 0..requests_per_worker {
                let base_msg = format!("worker {} - request {}", worker_id, i);

                let full_msg = match &payload_clone {
                    Some(p) => format!("{base_msg} - {p}"),
                    None => base_msg,
                };

                let req_body = SimpleRequestJson {
                    message: full_msg,
                };

                let resp = client_clone.post(&endpoint).json(&req_body).send().await;

                match resp {
                    Ok(r) => {
                        let json_result: Result<SimpleResponseJson, _> = r.json().await;
                        if let Err(e) = json_result {
                            eprintln!(
                                "[REST SIMPLE worker {}] failed to decode JSON in scenario '{}': {}",
                                worker_id, scenario_name, e
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "[REST SIMPLE worker {}] request {} failed in scenario '{}': {}",
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
