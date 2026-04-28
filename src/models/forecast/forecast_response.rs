use std::collections::HashMap;

use anyhow::anyhow;
use chrono::NaiveDate;
use serde::Deserialize;

use super::{Hourly, HourlyUnits, Daily, Forecast, WMOCode, Units};

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub hourly_units: HourlyUnits,
    pub hourly: Hourly,
    pub daily: Daily
}

impl ForecastResponse {
    pub fn to_forecasts(self: &Self) -> anyhow::Result<HashMap<NaiveDate, Forecast>> {
        let mut ans: HashMap<NaiveDate, Forecast> = HashMap::new();
        let dates: Vec<NaiveDate> = self.daily.time.clone();
        for (idd, date) in dates.iter().enumerate() {
            let date = *date;
            let (start_i, end_i) = (idd * 24, idd * 24 + 24);

            let temps = self.hourly.temperature.get(start_i..end_i).ok_or(anyhow!("Invalid weather data"))?.to_vec();
            let rain = self.hourly.rain.get(start_i..end_i).ok_or(anyhow!("Invalid weather data"))?.to_vec();
            let snow = self.hourly.snowfall.get(start_i..end_i).ok_or(anyhow!("Invalid weather data"))?.to_vec();
            let wind = self.hourly.wind_speed.get(start_i..end_i).ok_or(anyhow!("Invalid weather data"))?.to_vec();

            let sunrise = self.daily.sunrise.get(idd).ok_or(anyhow!("Invalid weather data"))?.time();
            let sunset = self.daily.sunset.get(idd).ok_or(anyhow!("Invalid weather data"))?.time();
            let uv_index = self.daily.uv_index.get(idd).ok_or(anyhow!("Invalid weather data"))?;

            let wmo_code = self.daily.wmo_codes.get(idd).ok_or(anyhow!("Invalid WMO1 data"))?;
            let wmo_code = WMOCode::try_from(*wmo_code).map_err(|x| anyhow!(x))?;

            ans.insert(date, Forecast {
                date: date,
                temperature: temps,
                rain: rain, snowfall: snow,
                wind_speed: wind,
                sunrise: sunrise, sunset: sunset,
                uv_index: *uv_index, units: Units {
                    temperature: self.hourly_units.temperature.clone(),
                    rain: self.hourly_units.rain.clone(),
                    snowfall: self.hourly_units.snowfall.clone(),
                    wind_speed: self.hourly_units.wind_speed.clone()
                },
                wmo_code: wmo_code
            });
        }
        return Ok(ans);
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_weather_parsing() {
        let example_forecast = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/example_forecast.json"));
        let parsed = serde_json::from_str::<ForecastResponse>(example_forecast);
        assert!(parsed.is_ok(), "example forecast should parse");
    }
}
