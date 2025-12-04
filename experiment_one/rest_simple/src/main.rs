use std::time::{SystemTime, UNIX_EPOCH};
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct SimpleRequestJson {
    message: String,
}

#[derive(Serialize)]
struct SimpleResponseJson {
    message: String,
    timestamp: i64,
}

async fn simple_handler(Json(req): Json<SimpleRequestJson>) -> Json<SimpleResponseJson> {
    Json(SimpleResponseJson {
        message: req.message,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    })
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/simple", post(simple_handler));

    let addr = "0.0.0.0:9090";
    println!("REST server listening on: {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
