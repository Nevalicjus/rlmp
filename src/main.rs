use anyhow::anyhow;
use clap::Parser;

mod basic;
mod cache;
mod cli;
mod fetch;
mod models;
mod state;
mod tui;
mod utils;

use crate::{
    basic::show, cli::Cli, fetch::fetch_lip, models::LocationInfo, tui::run_app
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let lip = if let Some(place) = cli.place {
        LocationInfo { city: Some(place), ..Default::default() }
    } else {
        fetch_lip().await
            .map_err(|_| anyhow!("Couldn't autofetch location data."))?
    };

    if cli.no_tui {
        show(lip, Some(cli.selected_date), !cli.no_namedays).await?;
    } else {
        run_app(lip, Some(cli.selected_date)).await?;
    }

    return Ok(());
}
