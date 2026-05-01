use std::{
    fmt::{self, Display},
    hash::{Hash, Hasher}
};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Latlon {
    pub lat: f32,
    pub lon: f32
}

impl Display for Latlon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{};{}", self.lat, self.lon);
    }
}

impl Hash for Latlon {
    fn hash<H: Hasher>(self: &Self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl PartialEq for Latlon {
    fn eq(self: &Self, other: &Self) -> bool {
        return self.to_string() == other.to_string();
    }
}

impl Eq for Latlon {}
