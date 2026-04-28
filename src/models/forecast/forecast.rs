use serde::{Deserialize, Serialize};
use chrono::{NaiveTime, NaiveDate};

use crate::utils::{min_f32, max_f32};
use super::{WMOCode, Units};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub date: NaiveDate,
    pub temperature: Vec<f32>,
    pub rain: Vec<f32>,
    pub snowfall: Vec<f32>,
    pub wind_speed: Vec<f32>,
    pub sunrise: NaiveTime,
    pub sunset: NaiveTime,
    pub uv_index: f32,
    pub wmo_code: WMOCode,
    pub units: Units
}

impl Forecast {
    pub fn min_temp(self: &Self) -> f32 {
        return min_f32(&self.temperature);
    }

    pub fn max_temp(self: &Self) -> f32 {
        return max_f32(&self.temperature);
    }

    pub fn min_wind_speed(self: &Self) -> f32 {
        return min_f32(&self.wind_speed);
    }

    pub fn max_wind_speed(self: &Self) -> f32 {
        return max_f32(&self.wind_speed);
    }

    pub fn first_rain(self: &Self) -> Option<NaiveTime> {
        for (i, rain) in self.rain.iter().enumerate() {
            if rain > &0.0f32 {
                return Some(NaiveTime::from_hms_opt(i as u32, 0, 0).unwrap());
            }
        }
        return None;
    }

    pub fn last_rain(self: &Self) -> Option<NaiveTime> {
        for (i, rain) in self.rain.iter().enumerate().rev() {
            if rain > &0.0f32 {
                return Some(NaiveTime::from_hms_opt(i as u32, 0, 0).unwrap());
            }
        }
        return None;
    }

    pub fn first_snow(self: &Self) -> Option<NaiveTime> {
        for (i, snow) in self.snowfall.iter().enumerate() {
            if snow > &0.0f32 {
                return Some(NaiveTime::from_hms_opt(i as u32, 0, 0).unwrap());
            }
        }
        return None;
    }

    pub fn last_snow(self: &Self) -> Option<NaiveTime> {
        for (i, snow) in self.snowfall.iter().enumerate().rev() {
            if snow > &0.0f32 {
                return Some(NaiveTime::from_hms_opt(i as u32, 0, 0).unwrap());
            }
        }
        return None;
    }
}
