use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex, MutexGuard}
};

use anyhow::anyhow;
use chrono::NaiveDate;
use serde_json;

use crate::models::{Latlon, Forecast, GeocodeItem};
use super::persistent::PersistentCache;

static CACHE_DIR: &'static str = ".cache/rlmp/";
static CACHE_FILE: &'static str = "persistent.cache";

static GLOBAL_CACHE: LazyLock<Mutex<Cache>> = LazyLock::new(|| {
    Mutex::new(Cache::new())
});

#[derive(Debug)]
pub struct Cache {
    pub geocode_cache: HashMap<String, Arc<Vec<GeocodeItem>>>,
    pub name_cache: HashMap<(usize, usize), Arc<Vec<String>>>,
    pub weather_cache: HashMap<(Latlon, NaiveDate), Arc<Forecast>>
}

impl Cache {
    pub fn new() -> Self {
        if let Ok(x) = Self::try_from_persist() {
            return x;
        } else {
            return Cache::default();
        }
    }

    fn try_from_persist() -> anyhow::Result<Self> {
        let mut path = std::env::home_dir().ok_or(anyhow!("Home directory not found"))?;
        path.push(CACHE_DIR);
        if !std::fs::exists(path.clone()).is_ok_and(|x| x == true) {
            std::fs::create_dir_all(path.clone())?;
            return Err(anyhow!("Cache dir didn't exist"));
        }
        path.push(CACHE_FILE);
        if !std::fs::exists(path.clone()).is_ok_and(|x| x == true) {
            return Err(anyhow!("Cache file didn't exist"));
        }
        let stringifed = std::fs::read_to_string(path)?;

        let persist = serde_json::from_str::<PersistentCache>(&stringifed)?;
        return Ok(persist.into_cache()?);
    }

    pub fn save_persist(self: &Self) -> anyhow::Result<()> {
        let mut path = std::env::home_dir().ok_or(anyhow!("Home directory not found"))?;
        path.push(CACHE_DIR);
        if !std::fs::exists(path.clone()).is_ok_and(|x| x == true) {
            std::fs::create_dir_all(path.clone())?;
        }
        path.push(CACHE_FILE);
        let persist = PersistentCache::from_cache(self);
        let stringified = serde_json::to_string::<PersistentCache>(&persist)?;
        return Ok(std::fs::write(path, stringified)?);
    }

    pub fn global() -> MutexGuard<'static, Cache> {
        return GLOBAL_CACHE.lock().unwrap();
    }
}

impl Default for Cache {
    fn default() -> Self {
        return Cache {
            geocode_cache: HashMap::new(),
            name_cache: HashMap::new(),
            weather_cache: HashMap::new()
        };
    }
}
