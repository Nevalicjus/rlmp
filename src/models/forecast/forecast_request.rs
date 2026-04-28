
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ForecastRequest<'a> {
    pub latitude: f32,
    pub longitude: f32,
    pub daily: Vec<&'a str>,
    pub hourly: Vec<&'a str>,
    pub timezone: &'a str,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl<'a> ForecastRequest<'a> {
    pub fn new(latitude: f32, longitude: f32, start: NaiveDate, end: NaiveDate) -> Self {
        return ForecastRequest {
            latitude, longitude,
            daily: vec!["sunrise","sunset","uv_index_max,weather_code"],
            hourly: vec!["temperature_2m","rain","snowfall","wind_speed_10m"],
            timezone: "GMT",
            start_date: start, end_date: end
        }
    }

    pub fn as_query(self: &Self) -> Vec<(&str, String)> {
        return [
            ("latitude", self.latitude.to_string()),
            ("longitude", self.longitude.to_string()),
            ("daily", self.daily.join(",")),
            ("hourly", self.hourly.join(",")),
            ("timezone", self.timezone.to_string()),
            ("start_date", self.start_date.to_string()),
            ("end_date", self.end_date.to_string())
        ].to_vec();
    }
}
