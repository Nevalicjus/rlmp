use anyhow::anyhow;
use chrono::NaiveDate;

use crate::{
    cache, fetch::{fetch_geocode, fetch_namedays, fetch_weather_range},
    utils::{date_range, end_of_week, start_of_week, today, weekday_short}
};

pub async fn show(place: String, selected_date: Option<NaiveDate>, show_namedays: bool) -> anyhow::Result<()> {
    let run = run_app(place, selected_date, show_namedays).await;
    if let Err(x) = run {
        println!("Error: {}", x);
    }
    return Ok(());
}

async fn run_app(place: String, selected_date: Option<NaiveDate>, show_namedays: bool) -> anyhow::Result<()> {
    let (start, end) = if let Some(date) = selected_date {
        (start_of_week(date), end_of_week(date))
    } else {
        let today = today();
        (start_of_week(today), end_of_week(today))
    };

    let best_code = fetch_geocode(place.clone()).await?
        .get(0).ok_or(anyhow!("Invalid geocodes amount"))?.clone();

    let weather = fetch_weather_range(best_code, start, end).await?;

    println!("{} {} {}", "-".repeat(10), place, "-".repeat(50 - place.len() - 12));
    for (idd, date) in date_range(start, end).enumerate().take(7) {
        let forecast = weather.get(&date).unwrap();
        let weekday = weekday_short(idd);
        println!("{} {} {} {}", "-".repeat(17), date, weekday, "-".repeat(17));
        println!(
            "      Overview:              {}",
            forecast.wmo_code);
        println!(
            "      Temp [min-max]:        {} - {} {}",
            forecast.min_temp(), forecast.max_temp(), forecast.units.temperature
        );
        println!(
            "      Sunrise - Sunset:      {} - {}",
            forecast.sunrise.format("%H:%M"), forecast.sunset.format("%H:%M")
        );
        if let (Some(first), Some(last)) = (forecast.first_rain(), forecast.last_rain()) {
            println!(
                "      Rain [first-last]:      {} - {}",
                first.format("%H:%M"), last.format("%H:%M")
            );
        } else {
            println!("      No rainfall");
        }
        if let (Some(first), Some(last)) = (forecast.first_snow(), forecast.last_snow()) {
            println!(
                "      Snow [first-last]:      {} - {}",
                first.format("%H:%M"), last.format("%H:%M")
            );
        } else {
            println!("      No snowfall");
        }
        println!(
            "      Wind [min-max]:        {} - {} {}",
            forecast.min_wind_speed(), forecast.max_wind_speed(), forecast.units.wind_speed
        );
        println!(
            "      UV Index:              {}",
            forecast.uv_index
        );
        if show_namedays {
            let namedays = fetch_namedays(date).await?;
            println!(
                "      Namedays:  {}",
                namedays.join(", ")
            );
        }

        println!("{}", "-".repeat(50));
    }
    println!("{}", "-".repeat(50));

    let cache = cache::Cache::global();
    cache.save_persist()?;

    return Ok(());
}
