use std::collections::HashMap;

use anyhow::anyhow;
use chrono::{NaiveDate, TimeDelta};
use tokio::sync::mpsc::UnboundedSender;

use super::{WeekdayData, WeekdayState};
use crate::{
    fetch::{fetch_weather_range, fetch_geocode, fetch_weather, fetch_namedays},
    tui::event::{AppEvent, Event}, utils::date_range
};

#[derive(Debug)]
pub struct State {
    pub place: String,
    pub weekday_data: HashMap<NaiveDate, WeekdayState>
}

impl State {
    pub fn new(place: String) -> Self {
        return Self {
            place: place,
            weekday_data: HashMap::new()
        };
    }

    pub fn get_weather_state(self: &mut Self, date: NaiveDate) -> &WeekdayState {
        if !self.weekday_data.contains_key(&date) {
            self.weekday_data.insert(date, WeekdayState::Loading);
        }
        return self.weekday_data.get(&date).unwrap();
    }

    pub async fn update(self: &mut Self, send: UnboundedSender<Event>, start: NaiveDate, end: NaiveDate) -> anyhow::Result<()> {
        let best_code = fetch_geocode(self.place.clone()).await?
            .get(0).ok_or(anyhow!("Invalid geocodes amount"))?.clone();

        for date in date_range(start, end) {
            let value = best_code.clone();
            let send = send.clone();
            tokio::spawn(async move {
                let forecast_res = fetch_weather(value.clone(), date).await;
                let namedays_res = fetch_namedays(date).await;

                let event = match (forecast_res, namedays_res) {
                    (Ok(forecast), Ok(namedays)) => AppEvent::WeatherLoaded {
                        date, data: WeekdayData {
                            forecast: (*forecast).clone(), namedays: (*namedays).clone()
                        }
                    },
                    (Ok(x), Err(_y)) => {
                        AppEvent::WeatherLoaded { date,
                            data: WeekdayData {
                                forecast: (*x).clone(), namedays: vec![String::from("Unavailable")]
                            }
                        }
                    },
                    (Err(_x), Ok(_y)) => {
                        AppEvent::WeatherError { date, err_msg: String::from("Unavailable") }
                    },
                    (Err(_x), Err(_y)) => {
                        AppEvent::WeatherError { date, err_msg: String::from("Unavailable") }
                    }
                };

                let _ = send.send(Event::App(event));
            });
        }
        return Ok(());
    }

    pub async fn warm_caches(self: &Self, date: NaiveDate) -> anyhow::Result<()> {
        let place = self.place.clone();
        tokio::spawn(async move {
            let geocodes = if let Ok(x) = fetch_geocode(place).await { x } else {
                return;
            };
            let best_code = if let Some(x) = geocodes.get(0) { x.clone() } else { return };
            let _ = fetch_weather_range(
                best_code, date - TimeDelta::days(60), date + TimeDelta::days(14)
            ).await;
        });
        return Ok(());
    }
}
