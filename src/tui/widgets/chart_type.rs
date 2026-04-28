#[derive(Copy, Clone, Debug)]
pub enum ChartType {
    Temp,
    Rain,
    Snow,
    Wind
}

impl ChartType {
    pub fn next(self: &Self) -> Self {
        return match self {
            ChartType::Temp => ChartType::Rain,
            ChartType::Rain => ChartType::Snow,
            ChartType::Snow => ChartType::Wind,
            ChartType::Wind => ChartType::Temp
        };
    }

    pub fn prev(self: &Self) -> Self {
        return match self {
            ChartType::Temp => ChartType::Wind,
            ChartType::Rain => ChartType::Temp,
            ChartType::Snow => ChartType::Rain,
            ChartType::Wind => ChartType::Snow
        }
    }
}
