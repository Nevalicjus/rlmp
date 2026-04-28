use ratatui::symbols::Marker;
use ratatui::widgets::{Axis, Dataset, GraphType, Widget};

mod temp;
mod wind;
mod rain;
mod snow;

pub use temp::Temp;
pub use wind::Wind;
pub use rain::Rain;
pub use snow::Snow;

static HOUR_LABELS: [&'static str; 9] = ["00:00", "04:00", "08:00", "10:00", "12:00", "14:00", "16:00", "20:00", "24:00"];
static HOUR_LABELS_ALL: [&'static str; 24] = [
    "00:00", "01:00", "02:00", "03:00", "04:00",
    "05:00", "06:00", "07:00", "08:00", "09:00",
    "10:00", "11:00", "12:00", "13:00", "14:00",
    "15:00", "16:00", "17:00", "18:00", "19:00",
    "20:00", "21:00", "22:00", "23:00"
];

pub fn labels_short(labels: &[&'static str]) -> Vec<String> {
    return labels.iter().map(|x| x.replace(":00", ""))
        .collect::<Vec<String>>();
}

pub trait ChartForecast<'a, T: Widget> {
    fn chart(self: &'a Self) -> T;
}

fn default_dataset() -> Dataset<'static> {
    return Dataset::default()
        .marker(Marker::Octant)
        .graph_type(GraphType::Line);
}

fn default_x_axis() -> Axis<'static> {
    return Axis::default()
        .title("Time")
        .bounds([0.0, 1.0])
        .labels(HOUR_LABELS);
}
