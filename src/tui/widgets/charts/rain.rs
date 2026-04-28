use ratatui::{
    style::Color,
    widgets::{Bar, BarChart, Block, Borders, Padding}
};

use super::{ChartForecast, labels_short, HOUR_LABELS_ALL};
use crate::models::Forecast;

pub struct Rain {
    forecast: Forecast,
    rain_data: Vec<Bar<'static>>
}

impl From<Forecast> for Rain {
    fn from(value: Forecast) -> Self {
        let rain_data: Vec<Bar> = value.rain
            .iter().zip(labels_short(&HOUR_LABELS_ALL).iter())
            .map(|(rain, label)|
                Bar::default()
                    .label(format!("{}", label))
                    .value(
                        // we artifically increase rainfall to always show value
                        if *rain > 0.0 { *rain as u64 + 5 } else { 0 }
                    )
                    .text_value(format!("{:.1}", rain))
                    .style(Color::Cyan)
            )
            .collect();
        return Self {
            forecast: value,
            rain_data: rain_data
        };
    }
}

impl<'a> ChartForecast<'a, BarChart<'a>> for Rain {
    fn chart(self: &'a Self) -> BarChart<'a> {
        return BarChart::vertical(self.rain_data.clone())
            .block(
                Block::default()
                    .title(format!("Rain [{}]", self.forecast.units.rain))
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(10))
            )
            .bar_width(3)
            .bar_gap(2)
            // we could use a .max_rain() of this day,
            // but then between days rescaling looks weird
            .max(50)
            .bar_style(Color::Cyan)
            .value_style(Color::White);
    }
}
