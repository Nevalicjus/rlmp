use reqwest::Client;

use super::client::client;

use crate::models::LocationInfo;

static LIP_API: &'static str = "https://lip.nevi.fyi/lookup";

pub async fn fetch_lip() -> anyhow::Result<LocationInfo> {
    return Ok(_fetch_lip(client()).await?);
}

async fn _fetch_lip(client: &Client) -> anyhow::Result<LocationInfo> {
    let r = client.get(format!("{}", LIP_API))
        .send().await?.error_for_status()?;

    return Ok(r.json::<LocationInfo>().await?);
}
