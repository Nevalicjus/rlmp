use ratatui::{
    widgets::{Axis, Chart}, style::Color
};

use super::{ChartForecast, default_dataset, default_x_axis};
use crate::models::Forecast;

pub struct Temp {
    forecast: Forecast,
    temp_data: Vec<(f64, f64)>
}

impl From<Forecast> for Temp {
    fn from(value: Forecast) -> Self {
        let ylen = value.temperature.len();
        let ys: Vec<f64> = (0..ylen).map(|x| x as f64 / ylen as f64).collect::<Vec<f64>>();
        let temp_data: Vec<(f64, f64)> = value.temperature
            .iter().zip(ys.iter())
            .map(|(temp, i)| (*i, *temp as f64))
            .collect();
        return Self {
            forecast: value,
            temp_data: temp_data
        };
    }
}

impl<'a> ChartForecast<'a, Chart<'a>> for Temp {
    fn chart(self: &'a Self) -> Chart<'a> {
        let dataset = default_dataset()
            .style(Color::LightRed)
            .data(&self.temp_data);

        let x_axis = default_x_axis();

        let mut y_i = self.forecast.min_temp() as f64 - 5.0;
        let mut y_labels: Vec<String> = Vec::new();
        while y_i <= self.forecast.max_temp() as f64 + 5.0 {
            y_labels.push(format!("{:.2}", y_i));
            y_i += 1.0;
        }

        let y_axis = Axis::default()
            .title(format!("Temp [{}]", self.forecast.units.temperature))
            .bounds([self.forecast.min_temp() as f64 - 5.0, self.forecast.max_temp() as f64 + 5.0])
            .labels(y_labels);

        return Chart::new(vec![dataset]).x_axis(x_axis).y_axis(y_axis);
    }
}
