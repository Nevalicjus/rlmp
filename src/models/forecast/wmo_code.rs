use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::Intensity;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
/// Minimally adapted with merges and unification of intensity naming \
/// <https://open-meteo.com/en/docs#weather_variable_documentation>
pub enum WMOCode {
    Clear,
    PartlyCloudy,
    Overcast,
    Fog,
    Drizzle(Intensity),
    FreezingDrizzle(Intensity),
    Rain(Intensity),
    FreezingRain(Intensity),
    Snow(Intensity),
    SnowGrains,
    Thunderstorm
}

impl TryFrom<i32> for WMOCode {
    type Error = String;

    fn try_from(value: i32) -> anyhow::Result<Self, Self::Error> {
        let code = match value {
            0 => WMOCode::Clear,
            1 | 2 => WMOCode::PartlyCloudy,
            3 => WMOCode::Overcast,
            45 | 48 => WMOCode::Fog,
            51 => WMOCode::Drizzle(Intensity::Light),
            53 => WMOCode::Drizzle(Intensity::Moderate),
            55 => WMOCode::Drizzle(Intensity::Heavy),
            56 => WMOCode::FreezingDrizzle(Intensity::Light),
            57 => WMOCode::FreezingDrizzle(Intensity::Heavy),
            61 | 80 => WMOCode::Rain(Intensity::Light),
            63 | 81 => WMOCode::Rain(Intensity::Moderate),
            65 | 82 => WMOCode::Rain(Intensity::Heavy),
            66 => WMOCode::FreezingRain(Intensity::Light),
            67 => WMOCode::FreezingRain(Intensity::Heavy),
            71 | 85 => WMOCode::Snow(Intensity::Light),
            73 | 86 => WMOCode::Snow(Intensity::Moderate),
            75 => WMOCode::Snow(Intensity::Heavy),
            77 => WMOCode::SnowGrains,
            95 | 96 | 99 => WMOCode::Thunderstorm,
            _ => WMOCode::Clear
        };
        return if code == WMOCode::Clear && value != 0 {
            Err(format!("Invalid WMO code: {}", value))
        } else {
            Ok(code)
        }
    }
}

impl Display for WMOCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            WMOCode::Clear => "clear",
            WMOCode::PartlyCloudy => "partly cloudy",
            WMOCode::Overcast => "overcast",
            WMOCode::Fog => "fog",
            WMOCode::Drizzle ( intensity ) => &format!("{} {}", intensity, "drizzle"),
            WMOCode::FreezingDrizzle ( intensity ) => &format!("{} {}", intensity, "f. drizzle"),
            WMOCode::Rain ( intensity ) => &format!("{} {}", intensity, "rain"),
            WMOCode::FreezingRain ( intensity ) => &format!("{} {}", intensity, "f. rain"),
            WMOCode::Snow ( intensity ) => &format!("{} {}", intensity, "snow"),
            WMOCode::SnowGrains => "snow grains",
            WMOCode::Thunderstorm => "thunderstorm"
        };
        write!(f, "{}", s)
    }
}
