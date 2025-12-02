use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct RestPingRequest {
    message: String,
}

#[derive(Serialize)]
struct RestPingResponse {
    message: String,
    timestamp: i64,
}

async fn ping(Json(payload): Json<RestPingRequest>) -> Json<RestPingResponse> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let reply = RestPingResponse {
        message: format!("Pong: {}", payload.message),
        timestamp: now,
    };

    Json(reply)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app: Router<()> = Router::new().route("/ping", post(ping));

    let addr = "0.0.0.0:10010";
    println!("REST-JSON server listening on http://{}/ping", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
