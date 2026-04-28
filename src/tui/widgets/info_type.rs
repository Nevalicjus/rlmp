#[derive(Copy, Clone, Debug)]
pub enum InfoType {
    /// Temperature, Overview, Sunrise-Sunset
    TempOverSun,
    /// Rain first-last, Snow first-last
    RainSnow,
    /// Wind, UV Index
    WindUV,
    /// Namedays
    Namedays
}

impl InfoType {
    pub fn next(self: &Self) -> Self {
        return match self {
            InfoType::TempOverSun => InfoType::RainSnow,
            InfoType::RainSnow => InfoType::WindUV,
            InfoType::WindUV => InfoType::Namedays,
            InfoType::Namedays => InfoType::TempOverSun
        };
    }
}
