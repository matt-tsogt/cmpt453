use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use main::service::{
    Contact,
    Owner,
    Coordinates,
    Building,
    ComplexResponse,
};

pub fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn complex_response(num_readings: usize) -> ComplexResponse {
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
    tags.insert("env".to_string(), "experiment".to_string());

    ComplexResponse {
        project_id: "proj_78432".to_string(),
        name: "Smart Building Automation".to_string(),
        created_at: "2025-12-03T18:52:00Z".to_string(),
        owner: Some(owner),
        building: Some(building),
        readings,
        tags,
    }
}

