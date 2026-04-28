use chrono::NaiveDate;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Line, Text}, style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget}
};

use crate::{
    state::WeekdayState, tui::widgets::InfoType
};

pub struct Weekday<'a> {
    pub date: NaiveDate,
    pub is_selected: bool,
    pub is_today: bool,
    pub addt_info_type: InfoType,
    pub weather: &'a WeekdayState,
    pub weekday: String
}

impl<'a> Widget for Weekday<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = match self.weather {
            WeekdayState::Loading => {
                Text::from(vec![
                    Line::from("Loading..."),
                ])
            },
            WeekdayState::Ready(data) => {
                let forecast = &data.forecast;
                match self.addt_info_type {
                    InfoType::TempOverSun => {
                        Text::from(vec![
                            Line::from(""),
                            Line::from(forecast.wmo_code.to_string()),
                            Line::from(""),
                            Line::from("Temp [min-max]"),
                            Line::from(
                                format!("{} - {} {}",
                                    forecast.min_temp(),
                                    forecast.max_temp(),
                                    forecast.units.temperature
                                )
                            ),
                            Line::from(""),
                            Line::from("Sunrise-Sunset"),
                            Line::from(format!("{} - {}", forecast.sunrise.format("%H:%M"), forecast.sunset.format("%H:%M")))
                        ])
                    },
                    InfoType::RainSnow => {
                        Text::from(vec![
                            Line::from(""),
                            Line::from("Rain [first-last]"),
                            if let (Some(first), Some(last)) = (forecast.first_rain(), forecast.last_rain()) {
                                Line::from(format!("{} - {}", first.format("%H:%M"), last.format("%H:%M")))
                            } else {
                                Line::from("No rainfall")
                            },
                            Line::from(""),
                            Line::from(""),
                            Line::from("Snow [first-last]"),
                            if let (Some(first), Some(last)) = (forecast.first_snow(), forecast.last_snow()) {
                                Line::from(format!("{} - {}", first.format("%H:%M"), last.format("%H:%M")))
                            } else {
                                Line::from("No snowfall")
                            },
                            Line::from(""),
                        ])
                    },
                    InfoType::WindUV => {
                        Text::from(vec![
                            Line::from(""),
                            Line::from("Wind [min-max]"),
                            Line::from(
                                format!("{} - {} {}",
                                    forecast.min_wind_speed(),
                                    forecast.max_wind_speed(),
                                    forecast.units.wind_speed
                                )
                            ),
                            Line::from(""),
                            Line::from(""),
                            Line::from("UV Index"),
                            Line::from(format!("{}", forecast.uv_index))
                        ])
                    },
                    InfoType::Namedays => {
                        let mut lines = vec![
                            Line::from(""),
                            Line::from("Namedays"),
                            Line::from(""),
                        ];
                        lines.extend(data.namedays.iter().map(|name| Line::from(name.clone())));
                        Text::from(lines)
                    }
                }
            },
            WeekdayState::Err(err_msg) => {
                Text::from(vec![
                    Line::from("Error:"),
                    Line::from(err_msg.as_str())
                ])
            }
        };

        let block = Block::default()
            .title(format!(" {} {} ", self.date, self.weekday))
            .title_alignment(Alignment::Center)
            .title_style(Style::default().not_dim())
            .borders(Borders::ALL)
            .border_style(if self.is_selected && self.is_today {
                Style::default().fg(Color::Cyan)
            } else if self.is_selected {
                Style::default()
            } else if self.is_today {
                Style::default().fg(Color::Cyan).dim()
            } else {
                Style::default().dim()
            });

        Paragraph::new(content)
            .block(block)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
