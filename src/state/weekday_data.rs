use crate::models::Forecast;

#[derive(Clone, Debug)]
pub struct WeekdayData {
    pub forecast: Forecast,
    pub namedays: Vec<String>
}
