use std::{
    collections::HashMap,
    sync::Arc
};

use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use crate::models::{GeocodeItem, Forecast, Lonlat};
use crate::cache::Cache;

#[derive(Deserialize, Serialize)]
pub struct PersistentCache {
    pub geocode_cache: HashMap<String, Vec<GeocodeItem>>,
    /// Key in format `usize;usize`
    pub name_cache: HashMap<String, Vec<String>>,
    /// Key in format `f32;f32;NaiveDate`
    pub weather_cache: HashMap<String, Forecast>
}

impl PersistentCache {
    pub fn into_cache(self: Self) -> anyhow::Result<Cache> {
        let mut name_cache: HashMap<(usize, usize), Arc<Vec<String>>> = HashMap::new();
        for (key, names) in self.name_cache {
            let parts = key.split(";").collect::<Vec<&str>>();
            let (month, day) = (parts[0].parse::<usize>()?, parts[1].parse::<usize>()?);
            name_cache.insert((month, day), Arc::new(names));
        }
        let mut weather_cache: HashMap<(Lonlat, NaiveDate), Arc<Forecast>> = HashMap::new();
        for (key, forecast) in self.weather_cache {
            let parts = key.split(";").collect::<Vec<&str>>();
            let (lat, lon, date) = (parts[0].parse::<f32>()?, parts[1].parse::<f32>()?, parts[2].parse::<NaiveDate>()?);
            weather_cache.insert((Lonlat { lat, lon }, date), Arc::new(forecast));
        }

        return Ok(Cache {
            geocode_cache: self.geocode_cache.iter()
                .map(|(place, geocodes)| (place.clone(), Arc::new(geocodes.clone())))
                .collect::<HashMap<String, Arc<Vec<GeocodeItem>>>>(),
            name_cache: name_cache,
            weather_cache: weather_cache
        });
    }

    pub fn from_cache(cache: &Cache) -> PersistentCache {
        return PersistentCache {
            geocode_cache: cache.geocode_cache.iter()
                .map(|(place, geocodes)| (place.clone(), (**geocodes).clone()))
                .collect::<HashMap<String, Vec<GeocodeItem>>>(),
            name_cache: cache.name_cache.iter()
                .map(|((month, day), names)| {
                    (format!("{};{}", month, day), (**names).clone())
                })
                .collect::<HashMap<String, Vec<String>>>(),
            weather_cache: cache.weather_cache.iter()
                .map(|((Lonlat { lat, lon }, date), forecast)| {
                    (format!("{};{};{}", lat, lon, date), (**forecast).clone())
                })
                .collect::<HashMap<String, Forecast>>()
        }
    }
}
