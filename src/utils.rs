use std::{
    cmp::Reverse,
    collections::HashMap,
    sync::LazyLock
};

use chrono::{Datelike, Days, Local, NaiveDate, NaiveDateTime, TimeDelta};
use serde::Deserialize;

pub static NAMES: LazyLock<HashMap<String, u32>> = LazyLock::new(|| {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/names.txt"))
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts = line.split(";").collect::<Vec<&str>>();
            (String::from(parts[0]), parts[1].parse::<u32>().expect("invalid imiona format"))
        })
        .collect::<HashMap<String, u32>>()
});

pub static WEEKDAYS: [(&str, &str); 7] = [
    ("monday",    "poniedziałek"),
    ("tuesday",   "wtorek"),
    ("wednesday", "środa"),
    ("thursday",  "czwartek"),
    ("friday",    "piątek"),
    ("saturday",  "sobota"),
    ("sunday",    "niedziela"),
];

pub static MONTHS: [(&str, &str); 12] = [
    ("january",   "stycznia"),
    ("february",  "lutego"),
    ("march",     "marca"),
    ("april",     "kwietnia"),
    ("may",       "maja"),
    ("june",      "czerwca"),
    ("july",      "lipca"),
    ("august",    "sierpnia"),
    ("september", "września"),
    ("october",   "października"),
    ("november",  "listopada"),
    ("december",  "grudnia"),
];

pub fn get_name(name: impl Into<String>) -> Option<u32> {
    return NAMES.get(&name.into().to_lowercase()).map(|x| *x);
}

pub fn weekday_short(idx: usize) -> String {
    return format!(
        "{}{}", &WEEKDAYS[idx].0[0..1].to_uppercase(), &WEEKDAYS[idx].0[1..3]
    );
}

pub fn date_range(start: NaiveDate, end: NaiveDate) -> impl Iterator<Item = NaiveDate> {
    return std::iter::successors(Some(start), move |d| {
        let next = *d + TimeDelta::days(1);
        (next <= end).then_some(next)
    });
}

pub fn longest_runs<T: Clone + Ord>(v: &Vec<T>, is_next: fn(&T, &T) -> bool) -> Vec<(T, T)> {
    let mut ans: Vec<(T, T)> = Vec::new();

    let mut v = v.clone();
    v.sort();

    let mut start = v[0].clone();
    let mut prev = v[0].clone();

    for el in v.iter().skip(1) {
        if is_next(&prev, &el) {
            prev = el.clone();
        } else {
            ans.push((start, prev));
            start = el.clone();
            prev = el.clone();
        }
    }
    ans.push((start, prev));
    return ans;
}

pub fn min_f32(v: &Vec<f32>) -> f32 {
    let mut sel: f32 = v[0];
    for el in v { if el < &sel { sel = *el } }
    return sel;
}

pub fn max_f32(v: &Vec<f32>) -> f32 {
    let mut sel: f32 = v[0];
    for el in v { if el > &sel { sel = *el } }
    return sel;
}

pub fn filter_names_popularity(names: Vec<String>) -> Vec<String> {
    let mut names: Vec<String> = names.into_iter()
        .filter(|name| get_name(name).is_some())
        .collect();
    names.sort_by_key(|n| Reverse(get_name(n).unwrap()));
    names = names.into_iter().take(3).collect();
    names.sort();
    return names;
}

pub fn now() -> NaiveDateTime {
    return Local::now().naive_local();
}

pub fn today() -> NaiveDate {
    return now().date();
}

pub fn start_of_week(date: NaiveDate) -> NaiveDate {
    return date - TimeDelta::days(date.weekday().num_days_from_monday() as i64);
}

pub fn end_of_week(date: NaiveDate) -> NaiveDate {
    return start_of_week(date).checked_add_days(Days::new(7)).unwrap();
}

pub fn deserialize_vec_datetime<'de, D>(deserializer: D) -> Result<Vec<NaiveDateTime>, D::Error>
where D: serde::Deserializer<'de> {
    return Vec::<String>::deserialize(deserializer)?
        .into_iter()
        .map(|s| {
            NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M").map_err(serde::de::Error::custom)
        })
        .collect();
}
