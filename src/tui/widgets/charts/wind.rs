use ratatui::{
    widgets::{Axis, Chart}, style::Color
};

use super::{ChartForecast, default_dataset, default_x_axis};
use crate::models::Forecast;

pub struct Wind {
    forecast: Forecast,
    wind_data: Vec<(f64, f64)>
}

impl From<Forecast> for Wind {
    fn from(value: Forecast) -> Self {
        let ylen = value.temperature.len();
        let ys: Vec<f64> = (0..ylen).map(|x| x as f64 / ylen as f64).collect::<Vec<f64>>();
        let wind_data: Vec<(f64, f64)> = value.wind_speed
            .iter().zip(ys.iter())
            .map(|(wind, i)| (*i, *wind as f64))
            .collect();
        return Self {
            forecast: value,
            wind_data: wind_data
        };
    }
}

impl<'a> ChartForecast<'a, Chart<'a>> for Wind {
    fn chart(self: &'a Self) -> Chart<'a> {
        let dataset = default_dataset()
            .style(Color::Gray)
            .data(&self.wind_data);

        let x_axis = default_x_axis();

        let mut y_i = 0.0;
        let mut y_labels: Vec<String> = Vec::new();
        while y_i <= self.forecast.max_wind_speed() as f64 + 5.0 {
            y_labels.push(format!("{:.2}", y_i));
            y_i += 2.0;
        }

        let y_axis = Axis::default()
            .title(format!("Wind [{}]", self.forecast.units.wind_speed))
            .bounds([0.0, self.forecast.max_wind_speed() as f64 + 5.0])
            .labels(y_labels);

        return Chart::new(vec![dataset]).x_axis(x_axis).y_axis(y_axis);
    }
}
