use std::sync::Arc;

use anyhow::anyhow;
use chrono::{Datelike, NaiveDate};
use reqwest::{Client, multipart::Form};

use crate::{
    cache, models::PageResponse,
    utils::{MONTHS, filter_names_popularity}
};
use super::client::client;

const PAGE_API: &'static str = "https://pl.wikipedia.org/api/rest_v1/page/html";
const HTML2WIKI_API: &'static str = "https://pl.wikipedia.org/api/rest_v1/transform/html/to/wikitext";

pub async fn fetch_namedays(date: NaiveDate) -> anyhow::Result<Arc<Vec<String>>> {
    let (month, day) = (date.month0() as usize, date.day() as usize);
    let page_key = format!("{}_{}", day, MONTHS.get(month).ok_or(anyhow!("Invalid month"))?.1);

    if let Some(x) = {
        let cache = cache::Cache::global();
        cache.name_cache.get(&(month, day)).cloned()
    } {
        return Ok(x);
    }

    let html = _get_html(client(), page_key.clone()).await?;
    let wikitext = _html_to_wikitext(client(), html, page_key.clone()).await?;
    let res = Arc::new(filter_names_popularity(PageResponse { source: wikitext }.get_names()?));

    let mut cache = cache::Cache::global();
    cache.name_cache.insert((month, day), res.clone());
    return Ok(res);
}

async fn _get_html(client: &Client, page_key: String) -> anyhow::Result<String> {
    let r = client.get(format!("{}/{}?redirect=true", PAGE_API, page_key))
        .send().await?.error_for_status()?;
    return Ok(r.text().await?);
}

async fn _html_to_wikitext(client: &Client, html: String, page_key: String) -> anyhow::Result<String> {
    let form = Form::new()
        .text("html", html)
        .text("scrub_wikitext", "true");
    let r = client.post(format!("{}/{}", HTML2WIKI_API, page_key))
        .multipart(form)
        .send().await?.error_for_status()?;
    return Ok(r.text().await?);
}
