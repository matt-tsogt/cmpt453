use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    body::Bytes,
    response::IntoResponse,
    routing::post,
    Router,
};
use tokio::net::TcpListener;

async fn ping(body: Bytes) -> impl IntoResponse {
    let msg = String::from_utf8_lossy(&body);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let response = format!("Pong: {}, timestamp={}", msg, now);
    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app: Router<()> = Router::new().route("/ping", post(ping));

    let addr = "0.0.0.0:9090";
    println!("REST server listening on http://{}/ping", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
