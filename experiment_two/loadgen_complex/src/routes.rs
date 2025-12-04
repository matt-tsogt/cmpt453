pub const GRPC_ENDPOINT: &str = "http://0.0.0.0:8081";
pub const REST_ENDPOINT: &str = "http://0.0.0.0:9091";

pub fn grpc_endpoint() -> String {
    std::env::var("GRPC_ENDPOINT")
        .unwrap_or_else(|_| GRPC_ENDPOINT.to_string())
}

pub fn rest_endpoint() -> String {
    std::env::var("REST_ENDPOINT")
        .unwrap_or_else(|_| REST_ENDPOINT.to_string())
}