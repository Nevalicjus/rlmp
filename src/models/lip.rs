use std::fmt::{self, Display};

use anyhow::anyhow;
use serde::Deserialize;

use crate::models::Latlon;
use crate::fetch::fetch_geocode;

#[derive(Clone, Debug, Deserialize)]
pub struct LocationInfo {
    pub country: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>
}

impl LocationInfo {
    pub fn usable(self: &Self) -> bool {
        return self.city.is_some() || (self.latitude.is_some() && self.longitude.is_some());
    }

    pub fn requires_geocoding(self: &Self) -> bool {
        return self.city.is_some() && (self.latitude.is_none() || self.longitude.is_none());
    }

    pub fn as_latlon(self: &Self) -> anyhow::Result<Latlon> {
        if let (Some(lat), Some(lon)) = (self.latitude, self.longitude) {
            return Ok(Latlon { lat, lon });
        }
        return Err(anyhow!("lat/lon unavailable"));
    }

    pub async fn try_geocode(self: &mut Self) -> anyhow::Result<()> {
        if !self.requires_geocoding() { return Ok(()); }
        if self.city.is_none() { return Err(anyhow!("Invalid location data")); }
        let best_code = fetch_geocode(self.city.clone().unwrap()).await?
            .get(0).ok_or(anyhow!("Invalid geocodes amount"))?.clone();
        self.latitude = Some(best_code.latitude);
        self.longitude = Some(best_code.longitude);
        return Ok(());
    }
}

impl Default for LocationInfo {
    fn default() -> Self {
        return Self { city: None, country: None, latitude: None, longitude: None };
    }
}

impl Display for LocationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hr = match (self.city.clone(), self.country.clone()) {
            (Some(city), Some(country)) => Some(format!("{}, {}", city, country)),
            (Some(city), _) => Some(format!("{}", city)),
            (_, _) => None
        };
        if let Some(hr) = hr {
            return write!(f, "{}", hr);
        }
        let ll = match (self.latitude, self.longitude) {
            (Some(lat), Some(lon)) => Some(format!("{},{}", lat, lon)),
            (_, _) => None
        };
        if let Some(ll) = ll {
            return write!(f, "{}", ll);
        } else {
            return write!(f, "?unknown");
        }
    }
}
