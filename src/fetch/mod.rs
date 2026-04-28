mod client;
mod weather;
mod wikipedia;

pub use weather::{fetch_geocode, fetch_weather, fetch_weather_range};
pub use wikipedia::fetch_namedays;
