use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Units {
    pub temperature: String,
    pub rain: String,
    pub snowfall: String,
    pub wind_speed: String
}
