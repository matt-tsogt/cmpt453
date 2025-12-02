use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tonic::Request;

use main::service::{
    PingRequest,
    service_client::ServiceClient,
};

const NUM_WORKERS: u32 = 10;

struct Scenario {
    name: &'static str,
    requests_per_worker: u32,
    payload_size_bytes: usize, 
}

#[derive(Serialize)]
struct RestPingRequest {
    message: String,
}

#[derive(Deserialize)]
struct RestPingResponse {
    message: String,
    timestamp: i64,
}

// Defaults are for local runs; Docker will override via env vars.
fn grpc_endpoint() -> String {
    std::env::var("GRPC_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string())
}

fn rest_endpoint() -> String {
    std::env::var("REST_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:9090/ping".to_string())
}

fn rest_json_endpoint() -> String {
    std::env::var("REST_JSON_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:10010/ping".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Scenarios: 3 loads × 3 payload sizes
    let scenarios = [
        // Small loads
        Scenario {
            name: "Small load, small payload",
            requests_per_worker: 100,
            payload_size_bytes: 100,
        },
        Scenario {
            name: "Small load, medium payload",
            requests_per_worker: 100,
            payload_size_bytes: 1_000,
        },
        Scenario {
            name: "Small load, large payload",
            requests_per_worker: 100,
            payload_size_bytes: 10_000,
        },

        // Medium loads
        Scenario {
            name: "Medium load, small payload",
            requests_per_worker: 500,
            payload_size_bytes: 100,
        },
        Scenario {
            name: "Medium load, medium payload",
            requests_per_worker: 500,
            payload_size_bytes: 1_000,
        },
        Scenario {
            name: "Medium load, big payload",
            requests_per_worker: 500,
            payload_size_bytes: 10_000,
        },

        // Large loads
        Scenario {
            name: "Large load, small payload",
            requests_per_worker: 1_000,
            payload_size_bytes: 100,
        },
        Scenario {
            name: "Large load, medium payload",
            requests_per_worker: 1_000,
            payload_size_bytes: 1_000,
        },
        Scenario {
            name: "Large load, large payload",
            requests_per_worker: 1_000,
            payload_size_bytes: 10_000,
        },
    ];

    println!("Running load test for the REST server");
    for scenario in scenarios.iter() {
        let elapsed_time = run_scenario_rest(scenario).await?;
        print_stats("REST", scenario, elapsed_time);
    }

    println!("Running load test for the REST-JSON server");
    for scenario in scenarios.iter() {
        let elapsed_time = run_scenario_rest_json(scenario).await?;
        print_stats("REST-JSON", scenario, elapsed_time);
    }

    println!("Running load test for the gRPC server");
    for scenario in scenarios.iter() {
        let elapsed_time = run_scenario_grpc(scenario).await?;
        print_stats("gRPC", scenario, elapsed_time);
    }

    Ok(())
}

fn print_stats(label: &str, scenario: &Scenario, elapsed: Duration) {
    let total_requests = NUM_WORKERS * scenario.requests_per_worker;
    let secs = elapsed.as_secs_f64();
    let throughput = total_requests as f64 / secs;

    println!("---- {} Scenario: {} ----", label, scenario.name);
    println!("  Workers:              {}", NUM_WORKERS);
    println!("  Requests per worker:  {}", scenario.requests_per_worker);
    println!("  Total requests:       {}", total_requests);
    println!("  Payload size (bytes): {}", scenario.payload_size_bytes);
    println!("  Time taken:           {:.3?}", elapsed);
    println!("  Throughput:           {:.2} req/s\n", throughput);
}

// ======================= gRPC version =======================

async fn run_scenario_grpc(s: &Scenario) -> Result<Duration, Box<dyn std::error::Error>> {
    println!("Running gRPC scenario: {}", s.name);

    let client = ServiceClient::connect(grpc_endpoint()).await?;

    let payload = if s.payload_size_bytes > 0 {
        Some("x".repeat(s.payload_size_bytes))
    } else {
        None
    };

    let scenario_name = s.name;
    let requests_per_worker = s.requests_per_worker;

    let start = Instant::now();
    let mut handles = Vec::new();

    for worker_id in 0..NUM_WORKERS {
        let mut client_clone = client.clone();
        let payload_clone = payload.clone();
        let scenario_name = scenario_name;
        let requests_per_worker = requests_per_worker;

        let handle = tokio::spawn(async move {
            for i in 0..requests_per_worker {
                let base_msg = format!("worker {} - request {}", worker_id, i);

                let full_msg = match &payload_clone {
                    Some(p) => format!("{base_msg} - {p}"),
                    None => base_msg,
                };

                let request = Request::new(PingRequest { message: full_msg });

                if let Err(e) = client_clone.ping(request).await {
                    eprintln!(
                        "[gRPC worker {}] request {} failed in scenario '{}': {}",
                        worker_id, i, scenario_name, e
                    );
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

// ======================= REST (raw body) =======================

async fn run_scenario_rest(s: &Scenario) -> Result<Duration, Box<dyn std::error::Error>> {
    println!("Running REST scenario: {}", s.name);

    let client = reqwest::Client::new();

    let payload = if s.payload_size_bytes > 0 {
        Some("x".repeat(s.payload_size_bytes))
    } else {
        None
    };

    let scenario_name = s.name;
    let requests_per_worker = s.requests_per_worker;

    let start = Instant::now();
    let mut handles = Vec::new();

    for worker_id in 0..NUM_WORKERS {
        let client_clone = client.clone();
        let payload_clone = payload.clone();
        let scenario_name = scenario_name;
        let requests_per_worker = requests_per_worker;

        let handle = tokio::spawn(async move {
            for i in 0..requests_per_worker {
                let base_msg = format!("worker {} - request {}", worker_id, i);

                let full_msg = match &payload_clone {
                    Some(p) => format!("{base_msg} - {p}"),
                    None => base_msg,
                };

                let resp = client_clone
                    .post(rest_endpoint())
                    .body(full_msg)
                    .send()
                    .await;

                if let Err(e) = resp {
                    eprintln!(
                        "[REST worker {}] request {} failed in scenario '{}': {}",
                        worker_id, i, scenario_name, e
                    );
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

// ======================= REST-JSON =======================

async fn run_scenario_rest_json(s: &Scenario) -> Result<Duration, Box<dyn std::error::Error>> {
    println!("Running REST-JSON scenario: {}", s.name);

    let client = reqwest::Client::new();

    let payload = if s.payload_size_bytes > 0 {
        Some("x".repeat(s.payload_size_bytes))
    } else {
        None
    };

    let scenario_name = s.name;
    let requests_per_worker = s.requests_per_worker;

    let start = Instant::now();
    let mut handles = Vec::new();

    for worker_id in 0..NUM_WORKERS {
        let client_clone = client.clone();
        let payload_clone = payload.clone();
        let scenario_name = scenario_name;
        let requests_per_worker = requests_per_worker;

        let handle = tokio::spawn(async move {
            for i in 0..requests_per_worker {
                let base_msg = format!("worker {} - request {}", worker_id, i);

                let full_msg = match &payload_clone {
                    Some(p) => format!("{base_msg} - {p}"),
                    None => base_msg,
                };

                let req_body = RestPingRequest {
                    message: full_msg,
                };

                let resp = client_clone
                    .post(rest_json_endpoint())
                    .json(&req_body)
                    .send()
                    .await;

                match resp {
                    Ok(r) => {
                        let json_result: Result<RestPingResponse, _> = r.json().await;
                        if let Err(e) = json_result {
                            eprintln!(
                                "[REST-JSON worker {}] failed to decode JSON in scenario '{}': {}",
                                worker_id, scenario_name, e
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "[REST-JSON worker {}] request {} failed in scenario '{}': {}",
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
