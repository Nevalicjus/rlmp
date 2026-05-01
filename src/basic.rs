use chrono::NaiveDate;

use crate::{
    cache, fetch::{fetch_namedays, fetch_weather_range},
    models::LocationInfo,
    utils::{date_range, end_of_week, start_of_week, today, weekday_short}
};

pub async fn show(loc: LocationInfo, selected_date: Option<NaiveDate>, show_namedays: bool) -> anyhow::Result<()> {
    let run = run_app(loc, selected_date, show_namedays).await;
    if let Err(x) = run {
        println!("Error: {}", x);
    }
    cache::Cache::global().save_persist()?;
    return Ok(());
}

async fn run_app(mut loc: LocationInfo, selected_date: Option<NaiveDate>, show_namedays: bool) -> anyhow::Result<()> {
    let (start, end) = if let Some(date) = selected_date {
        (start_of_week(date), end_of_week(date))
    } else {
        let today = today();
        (start_of_week(today), end_of_week(today))
    };

    loc.try_geocode().await?;
    let latlon = loc.as_latlon()?;

    let weather = fetch_weather_range(latlon, start, end).await?;

    println!("- {} {}", loc, "-".repeat(50 - loc.to_string().len() - 3));
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

    return Ok(());
}
