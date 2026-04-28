use ratatui::{
    buffer::Buffer, layout::Rect,
    text::{Line, Text}, widgets::Widget
};

use super::{
    ChartType, charts::{ChartForecast, Temp, Rain, Snow, Wind}
};
use crate::state::WeekdayState;

pub struct Today<'a> {
    pub weather: &'a WeekdayState,
    pub chart_type: ChartType
}

impl<'a> Widget for Today<'a> {
    fn render(self: Self, area: Rect, buf: &mut Buffer) {
        match self.weather {
            WeekdayState::Loading => {
                let text = Text::from(format!("Loading..."));
                text.centered().render(area, buf);
            },
            WeekdayState::Err(err_msg) => {
                let text = Text::from(vec![
                    Line::from("Couldn't load weather"),
                    Line::from(format!("Error: {}", err_msg))
                ]);
                text.centered().render(area, buf);
            },
            WeekdayState::Ready(data) => {
                let forecast = &data.forecast;
                match self.chart_type {
                    ChartType::Rain => {
                        let chart = Rain::from(forecast.clone());
                        chart.chart().render(area, buf);
                    },
                    ChartType::Snow => {
                        let chart = Snow::from(forecast.clone());
                        chart.chart().render(area, buf);
                    },
                    ChartType::Temp => {
                        let chart = Temp::from(forecast.clone());
                        chart.chart().render(area, buf);
                    },
                    ChartType::Wind => {
                        let chart = Wind::from(forecast.clone());
                        chart.chart().render(area, buf);
                    }
                }
            }
        }
    }
}
