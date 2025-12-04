use std::time::Duration;

pub const NUM_WORKERS: u32 = 10;

pub struct Scenario {
    pub name: &'static str,
    pub requests_per_worker: u32,
    pub payload_size_bytes: usize,
}

pub const SCENARIOS: [Scenario; 9] = [
    // SMALL LOADS
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

pub fn print_stats(label: &str, scenario: &Scenario, elapsed: Duration) {
    let total_requests = NUM_WORKERS * scenario.requests_per_worker;
    let secs = elapsed.as_secs_f64();
    let throughput = total_requests as f64 / secs;

    println!("---- {} :: {} ----", label, scenario.name);
    println!("  Workers:              {}", NUM_WORKERS);
    println!("  Requests per worker:  {}", scenario.requests_per_worker);
    println!("  Total requests:       {}", total_requests);
    println!("  Payload size (bytes): {}", scenario.payload_size_bytes);
    println!("  Time taken:           {:.3?}", elapsed);
    println!("  Throughput:           {:.2} req/s\n", throughput);
}
