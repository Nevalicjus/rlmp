use std::collections::HashMap;

use anyhow::anyhow;
use chrono::{NaiveDate, TimeDelta};
use tokio::sync::mpsc::UnboundedSender;

use super::{WeekdayData, WeekdayState};
use crate::{
    fetch::{fetch_weather_range, fetch_weather, fetch_namedays},
    models::LocationInfo,
    tui::event::{AppEvent, Event}, utils::date_range
};

#[derive(Debug)]
pub struct State {
    pub loc: LocationInfo,
    pub weekday_data: HashMap<NaiveDate, WeekdayState>
}

impl State {
    pub fn new(loc: LocationInfo) -> Self {
        return Self {
            loc: loc,
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
        if !self.loc.usable() { Err(anyhow!("Location data unusable"))? }
        self.loc.try_geocode().await?;

        for date in date_range(start, end) {
            let latlon = self.loc.as_latlon()?;
            let send = send.clone();
            tokio::spawn(async move {
                let forecast_res = fetch_weather(latlon.clone(), date).await;
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
        let mut loc: LocationInfo = self.loc.clone();
        tokio::spawn(async move {
            if let Err(_) = loc.try_geocode().await { return; }
            let latlon = if let Ok(latlon) = loc.as_latlon() { latlon } else { return; };
            let _ = fetch_weather_range(
                latlon, date - TimeDelta::days(60), date + TimeDelta::days(14)
            ).await;
        });
        return Ok(());
    }
}
