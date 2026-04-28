use serde::Deserialize;
use chrono::{NaiveDate, NaiveDateTime};

use crate::utils::deserialize_vec_datetime;

#[derive(Debug, Deserialize)]
pub struct Daily {
    pub time: Vec<NaiveDate>,
    #[serde(deserialize_with = "deserialize_vec_datetime")]
    pub sunrise: Vec<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_vec_datetime")]
    pub sunset: Vec<NaiveDateTime>,
    #[serde(rename = "uv_index_max")]
    pub uv_index: Vec<f32>,
    #[serde(rename = "weather_code")]
    pub wmo_codes: Vec<i32>
}
