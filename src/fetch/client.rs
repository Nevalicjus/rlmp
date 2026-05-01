use std::{
    sync::OnceLock, time::Duration
};

use reqwest::Client;

static CLIENT: OnceLock<Client> = OnceLock::new();

macro_rules! retry_for_host {
    ($host:expr) => {
        reqwest::retry::for_host($host)
            .max_retries_per_request(5)
            .classify_fn(|req_rep| {
                if req_rep.status() == Some(reqwest::StatusCode::SERVICE_UNAVAILABLE) {
                    req_rep.retryable()
                } else {
                    req_rep.success()
                }
            })
    };
}

pub fn client() -> &'static Client {
    return CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent(format!("rlmp-bot/{}", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(5))
            .retry(retry_for_host!("geocoding-api.open-meteo.com"))
            .retry(retry_for_host!("api.open-meteo.com"))
            .retry(retry_for_host!("pl.wikipedia.org"))
            .build().unwrap()
    });
}
