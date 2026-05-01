use chrono::{NaiveDate, TimeDelta};
use clap::Parser;

use crate::utils::now;

#[derive(Debug, Parser)]
#[command(version, about = "Get weather information in the terminal")]
pub struct Cli {
    #[arg(help = "Place to fetch weather for [default: approx. location]")]
    pub place: Option<String>,

    #[arg(help = "Starting date", value_parser = selected_date_validator, default_value_t = now().date())]
    pub selected_date: NaiveDate,

    #[arg(help = "Don't show namedays", long = "no-namedays", default_value_t = false)]
    pub no_namedays: bool,

    #[arg(help = "Don't show tui", long = "no-tui", default_value_t = false)]
    pub no_tui: bool,
}

fn selected_date_validator(s: &str) -> anyhow::Result<NaiveDate, String> {
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| String::from("Invalid date format. Has to be YYYY-mm-dd"))?;
    if date > now().date() + TimeDelta::days(16) {
        return Err(String::from("Date too far into the future"));
    } else {
        return Ok(date);
    }
}
