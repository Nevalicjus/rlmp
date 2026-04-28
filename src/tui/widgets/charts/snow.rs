use ratatui::{
    style::Color,
    widgets::{Bar, BarChart, Block, Borders, Padding}
};

use super::{ChartForecast, labels_short, HOUR_LABELS_ALL};
use crate::models::Forecast;

pub struct Snow {
    forecast: Forecast,
    snow_data: Vec<Bar<'static>>
}

impl From<Forecast> for Snow {
    fn from(value: Forecast) -> Self {
        let snow_data: Vec<Bar> = value.snowfall
            .iter().zip(labels_short(&HOUR_LABELS_ALL).iter())
            .map(|(snow, label)|
                Bar::default()
                    .label(format!("{}", label))
                    .value(
                        // we artifically increase snowfall to always show value
                        if *snow > 0.0 { *snow as u64 + 5 } else { 0 }
                    )
                    .text_value(format!("{:.1}", snow))
                    .style(Color::White)
            )
            .collect();
        return Self {
            forecast: value,
            snow_data: snow_data
        };
    }
}

impl<'a> ChartForecast<'a, BarChart<'a>> for Snow {
    fn chart(self: &'a Self) -> BarChart<'a> {
        return BarChart::vertical(self.snow_data.clone())
            .block(
                Block::default()
                    .title(format!("Snow [{}]", self.forecast.units.snowfall))
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(10))
            )
            .bar_width(3)
            .bar_gap(2)
            // we could use a .max_rain() of this day,
            // but then between days rescaling looks weird
            .max(50)
            .bar_style(Color::White)
            .value_style(Color::White);
    }
}
