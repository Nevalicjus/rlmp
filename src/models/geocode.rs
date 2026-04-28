use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GeocodeRequest {
    pub name: String,
    pub count: i32,
    pub format: String
}

impl GeocodeRequest {
    pub fn new(name: String) -> Self {
        return GeocodeRequest {
            name: name, count: 10, format: String::from("json")
        };
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeocodeItem {
    pub id: u32,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub elevation: f32,
    pub country_code: String,
    pub timezone: String,
    pub population: Option<u32>
}

#[derive(Debug, Deserialize)]
pub struct GeocodeResponse {
    pub results: Vec<GeocodeItem>
}
