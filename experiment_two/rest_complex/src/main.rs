use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use std::collections::HashMap;

// ----- Local JSON types for REST COMPLEX -----

#[derive(Serialize, Deserialize, Clone)]
struct ContactJson {
    email: String,
    phone: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct OwnerJson {
    id: u32,
    name: String,
    contact: ContactJson,
}

#[derive(Serialize, Deserialize, Clone)]
struct CoordinatesJson {
    lat: f64,
    lng: f64,
}

#[derive(Serialize, Deserialize, Clone)]
struct BuildingJson {
    name: String,
    city: String,
    coordinates: CoordinatesJson,
}

#[derive(Serialize, Deserialize, Clone)]
struct ComplexJson {
    project_id: String,
    name: String,
    created_at: String,
    owner: OwnerJson,
    building: BuildingJson,
    readings: Vec<f64>,
    tags: HashMap<String, String>,
}

// request and response have the same shape
type ComplexRequestJson = ComplexJson;
type ComplexResponseJson = ComplexJson;

// ----- Handler & app -----

async fn complex_handler(
    Json(req): Json<ComplexRequestJson>,
) -> Json<ComplexResponseJson> {
    // Dummy: echo back the body, maybe tweak a tag if you like
    Json(req)
}

fn app() -> Router {
    Router::new().route("/complex", post(complex_handler))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app();

    let addr = "0.0.0.0:9091";
    println!("REST complex server listening on: {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
