use std::{
    collections::HashMap, sync::Arc
};

use anyhow::anyhow;
use chrono::{NaiveDate, TimeDelta};
use reqwest::Client;

const GEOCODING_API_BASE: &'static str = "https://geocoding-api.open-meteo.com/v1/search";
const WEATHER_API_BASE: &'static str = "https://api.open-meteo.com/v1/forecast";

use crate::{
    cache, models::{
        Forecast, ForecastRequest, ForecastResponse,
        GeocodeItem, GeocodeRequest, GeocodeResponse,
        Lonlat
    }, utils::{date_range, longest_runs}
};
use super::client::client;

pub async fn fetch_geocode(name: impl Into<String>) -> anyhow::Result<Arc<Vec<GeocodeItem>>> {
    let name = name.into();

    if let Some(x) = {
        let cache = cache::Cache::global();
        cache.geocode_cache.get(&name).cloned()
    } {
        return Ok(x);
    }

    let res = Arc::new(
        _geocode(client(), name.clone()).await
        .map_err(|_| anyhow!("Couldn't geocode place"))?
    );

    let mut cache = cache::Cache::global();
    cache.geocode_cache.insert(name, res.clone());
    return Ok(res);
}

pub async fn fetch_weather(geocode: GeocodeItem, date: NaiveDate) -> anyhow::Result<Arc<Forecast>> {
    return Ok(_fetch_weather_ranged(geocode, date, date).await?
        .get(&date).ok_or(anyhow!("Couldn't fetch weather"))?.clone());
}

pub async fn fetch_weather_range(
    geocode: GeocodeItem, start_date: NaiveDate, end_date: NaiveDate
) -> anyhow::Result<HashMap<NaiveDate, Arc<Forecast>>> {
    return Ok(_fetch_weather_ranged(geocode, start_date, end_date).await?);
}

async fn _fetch_weather_ranged(
    geocode: GeocodeItem, start_date: NaiveDate, end_date: NaiveDate
) -> anyhow::Result<HashMap<NaiveDate, Arc<Forecast>>> {
    if start_date != end_date {
        let uncached_days: Vec<NaiveDate> = {
            let cache = cache::Cache::global();
            let mut uncached_days: Vec<NaiveDate> = Vec::new();
            let lonlat = Lonlat::new(geocode.latitude, geocode.longitude);
            for date in date_range(start_date, end_date) {
                if let None = cache.weather_cache.get(&(lonlat, date)) {
                    uncached_days.push(date);
                }
            }
            uncached_days
        };
        if uncached_days.len() != 0 {
            let uncached_runs = longest_runs(&uncached_days, |d1, d2| *d1 + TimeDelta::days(1) == *d2);
            for (start_date, end_date) in uncached_runs {
                let _ = _weather(client(), geocode.clone(), start_date, end_date).await?;
            }
        }
    }

    let mut ans: HashMap<NaiveDate, Arc<Forecast>> = HashMap::new();

    for date in date_range(start_date, end_date) {
        ans.insert(date, _fetch_weather_cached(geocode.clone(), date).await?);
    }

    return Ok(ans);
}

async fn _fetch_weather_cached(geocode: GeocodeItem, date: NaiveDate) -> anyhow::Result<Arc<Forecast>> {
    let lonlat = Lonlat::new(geocode.latitude, geocode.longitude);
    if let Some(x) = {
        let cache = cache::Cache::global();
        cache.weather_cache.get(&(lonlat, date)).cloned()
    } {
        return Ok(x);
    }

    let res = Arc::new(
        _weather(client(), geocode, date, date).await?.to_forecasts()?
        .get(&date).ok_or(anyhow!("Requested date not found in fetched"))?.clone()
    );

    let mut cache = cache::Cache::global();
    cache.weather_cache.insert((lonlat, date), res.clone());
    return Ok(res);
}

async fn _geocode(client: &Client, name: impl Into<String>) -> anyhow::Result<Vec<GeocodeItem>> {
    let r = client.get(GEOCODING_API_BASE)
        .query(&GeocodeRequest::new(name.into()))
        .send().await?.error_for_status()?;
    return Ok(r.json::<GeocodeResponse>().await?.results);
}

async fn _weather(
    client: &Client, geocode: GeocodeItem, start: NaiveDate, end: NaiveDate
) -> anyhow::Result<ForecastResponse> {
    let fr = ForecastRequest::new(geocode.latitude, geocode.longitude, start, end);
    let r = client.get(WEATHER_API_BASE)
        .query(&fr.as_query())
        .send().await?.error_for_status()?;
    return Ok(r.json::<ForecastResponse>().await?);
}
