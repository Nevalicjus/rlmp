use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Lonlat {
    pub lat: f32,
    pub lon: f32
}

impl Lonlat {
    pub fn new(lat: f32, lon: f32) -> Self {
        return Self { lat, lon };
    }

    pub fn to_string(self: &Self) -> String {
        return format!("{};{}", self.lat, self.lon);
    }
}

impl Hash for Lonlat {
    fn hash<H: Hasher>(self: &Self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl PartialEq for Lonlat {
    fn eq(self: &Self, other: &Self) -> bool {
        return self.to_string() == other.to_string();
    }
}

impl Eq for Lonlat {}
