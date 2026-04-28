use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HourlyUnits {
    #[serde(rename = "temperature_2m")]
    pub temperature: String,
    pub rain: String,
    pub snowfall: String,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed: String
}

#[derive(Debug, Deserialize)]
pub struct Hourly {
    #[serde(rename = "temperature_2m")]
    pub temperature: Vec<f32>,
    pub rain: Vec<f32>,
    pub snowfall: Vec<f32>,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed: Vec<f32>
}
