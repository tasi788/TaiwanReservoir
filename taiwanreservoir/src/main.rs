use reqwest;

struct Reservoir {
    name: String,
    cap_available: f64,
    statistic_time_start: i32,
    statistic_time_end: i32,
    rain_fall: f64,
    inflow: f64,
    outflow: f64,
    water_level_diff: f64,
    record_time: i32,
    cap_level: f64,
    current_cap: f64,
    current_cap_percent: f64,
}

struct ASP {
    eventtarget: String,
    eventargument: String,
    lastfocus: String,
    viewstate: String,
    viewstategenerator: String,
}

trait ReservoirTrait {
    fn get_asp(&self) -> ASP;
    fn get_realtime(&self) -> Reservoir;
    fn get_history(&self) -> Vec<Reservoir>;


    fn url(&self) -> String {
        String::from("http://fhy.wra.gov.tw/ReservoirPage_2011/StorageCapacity.aspx")
    }
}

fn main() {
    println!("Hello, world!");
}
