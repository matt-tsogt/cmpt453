use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use main::service::{
    Contact,
    Owner,
    Coordinates,
    Building,
    ComplexRequest,
};

//
// SIMPLE REST JSON TYPES
//

#[derive(Serialize)]
pub struct SimpleRequestJson {
    pub message: String,
}

#[derive(Deserialize)]
pub struct SimpleResponseJson {
    pub message: String,
    pub timestamp: i64,
}

//
// COMPLEX REST JSON TYPES
//  Request and response share the same shape
//

#[derive(Serialize, Deserialize, Clone)]
pub struct ContactJson {
    pub email: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OwnerJson {
    pub id: u32,
    pub name: String,
    pub contact: ContactJson,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CoordinatesJson {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BuildingJson {
    pub name: String,
    pub city: String,
    pub coordinates: CoordinatesJson,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ComplexRequestJson {
    pub project_id: String,
    pub name: String,
    pub created_at: String,
    pub owner: OwnerJson,
    pub building: BuildingJson,
    pub readings: Vec<f64>,
    pub tags: HashMap<String, String>,
}

// Response uses the exact same JSON shape as the request
pub type ComplexResponseJson = ComplexRequestJson;

/// Build a ComplexRequestJson for REST.
pub fn make_complex_request_json(label: &str, num_readings: usize) -> ComplexRequestJson {
    let owner = OwnerJson {
        id: 219,
        name: "Grid".to_string(),
        contact: ContactJson {
            email: "grid@example.com".to_string(),
            phone: "+1-306-555-9000".to_string(),
        },
    };

    let building = BuildingJson {
        name: "Engineering Complex".to_string(),
        city: "Saskatoon".to_string(),
        coordinates: CoordinatesJson {
            lat: 52.1332,
            lng: -106.6700,
        },
    };

    let readings: Vec<f64> = (0..num_readings).map(|i| i as f64).collect();

    let mut tags = HashMap::new();
    tags.insert("project".to_string(), "smart-building".to_string());
    tags.insert("scenario".to_string(), label.to_string());

    ComplexRequestJson {
        project_id: format!("proj_{}", label.replace(' ', "_")),
        name: "Smart Building Automation".to_string(),
        created_at: "2025-12-03T18:52:00Z".to_string(),
        owner,
        building,
        readings,
        tags,
    }
}


pub fn make_complex_request(label: &str, num_readings: usize) -> ComplexRequest {
    let contact = Contact {
        email: "grid@example.com".to_string(),
        phone: "+1-306-555-9000".to_string(),
    };

    let owner = Owner {
        id: 219,
        name: "Grid".to_string(),
        contact: Some(contact),
    };

    let coordinates = Coordinates {
        lat: 52.1332,
        lng: -106.6700,
    };

    let building = Building {
        name: "Engineering Complex".to_string(),
        city: "Saskatoon".to_string(),
        coordinates: Some(coordinates),
    };

    let readings: Vec<f64> = (0..num_readings).map(|i| i as f64).collect();

    let mut tags = HashMap::new();
    tags.insert("project".to_string(), "smart-building".to_string());
    tags.insert("scenario".to_string(), label.to_string());

    ComplexRequest {
        project_id: format!("proj_{}", label.replace(' ', "_")),
        name: "Smart Building Automation".to_string(),
        created_at: "2025-12-03T18:52:00Z".to_string(),
        owner: Some(owner),
        building: Some(building),
        readings,
        tags,
    }
}
