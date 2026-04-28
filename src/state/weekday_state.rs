use super::WeekdayData;

#[derive(Clone, Debug)]
pub enum WeekdayState {
    Loading,
    Ready(WeekdayData),
    Err(String)
}
