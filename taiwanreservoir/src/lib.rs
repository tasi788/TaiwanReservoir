pub struct Reservoir {}

impl Reservoir {}

#[derive(Debug)]
pub struct ReservoirData {
    pub name: String,
    pub cap_available: f64,
    pub statistic_time_start: i32,
    pub statistic_time_end: i32,
    pub rain_fall: f64,
    pub inflow: f64,
    pub outflow: f64,
    pub water_level_diff: f64,
    pub record_time: i32,
    pub cap_level: f64,
    pub current_cap: f64,
    pub current_cap_percent: f64,
}