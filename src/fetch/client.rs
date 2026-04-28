use std::sync::OnceLock;

use reqwest::Client;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn client() -> &'static Client {
    return CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent(format!("rlmp-bot/{}", env!("CARGO_PKG_VERSION")))
            .build().unwrap()
    });
}
