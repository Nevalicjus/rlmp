mod client;
mod weather;
mod wikipedia;
mod lip;

pub use weather::{fetch_geocode, fetch_weather, fetch_weather_range};
pub use wikipedia::fetch_namedays;
pub use lip::fetch_lip;
