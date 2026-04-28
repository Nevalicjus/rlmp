use clap::Parser;

mod basic;
mod cli;
mod cache;
mod fetch;
mod models;
mod state;
mod tui;
mod utils;

use crate::{
    cli::Cli, basic::show, tui::run_app
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.no_tui {
        show(cli.place, Some(cli.selected_date), !cli.no_namedays).await?;
    } else {
        run_app(cli.place, Some(cli.selected_date)).await?;
    }

    return Ok(());
}
