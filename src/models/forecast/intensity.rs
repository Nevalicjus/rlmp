use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Intensity {
    Light,
    Moderate,
    Heavy
}

impl Display for Intensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Intensity::Light => "light",
            Intensity::Moderate => "moderate",
            Intensity::Heavy => "heavy"
        };
        write!(f, "{}", s)
    }
}
