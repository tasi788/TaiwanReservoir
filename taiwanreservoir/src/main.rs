use std::time::Duration;
use reqwest;
use scraper::{Html, Selector};

struct Reservoir {}
struct ReservoirData {
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
    fn get_asp(&self) -> Result<ASP, reqwest::Error>;
    fn get_realtime(&self) -> Reservoir;
    fn get_history(&self) -> Vec<Reservoir>;

    fn client(&self) -> reqwest::blocking::Client {
        // reqwest::blocking::Client::new()
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(3))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")
            .build()
            .unwrap()
    }

    fn url(&self) -> String {
        String::from("https://fhy.wra.gov.tw/ReservoirPage_2011/StorageCapacity.aspx")
    }
}

impl Reservoir { }

impl ReservoirTrait for Reservoir {
    fn get_asp(&self) -> Result<ASP, reqwest::Error>{
        let result = self.client().get(self.url()).send();

        let document = Html::parse_document(&result?.text()?);
        // let viewstate_s = Selector::parse(r#"input#__VIEWSTATE"#).unwrap();
        let viewstate = document.select(&Selector::parse(r#"input#__VIEWSTATE"#).unwrap())
            .next()
            .and_then(|e| e.value().attr("value"))
            .unwrap();
        let viewstategenerator = document.select(&Selector::parse(r#"input#__VIEWSTATEGENERATOR"#).unwrap())
            .next()
            .and_then(|e| e.value().attr("value"))
            .unwrap();
        // let viewstate = document.select(&selector).next()?.value().attr("value")?;
        println!("{:?}", viewstate);
        Ok(
            ASP {
            eventtarget: String::from("ctl00$cphMain$cboSearch"),
            eventargument: String::from(""),
            lastfocus: String::from(""),
            viewstate: String::from(viewstate),
            viewstategenerator: String::from(viewstategenerator),
        })

        // match reqwest::blocking::get(self.url()) {
        //     Ok(resp) => {
        //         let document = Html::parse_document(&resp.text().unwrap());
        //         let selector = Selector::parse(r#"input#__VIEWSTATE"#).unwrap();
        //         let input = document.select(&selector).next().unwrap();
        //         println!("{:?}", input.value().attr("value"));
        //     }
        //     Err(e) => {
        //         println!("{:?}", e);
        //         // panic!("error")
        //     }
        // }



    }

    fn get_realtime(&self) -> Reservoir {
        todo!()
    }

    fn get_history(&self) -> Vec<Reservoir> {
        todo!()
    }
}

fn main() {
    let c = Reservoir {};
    c.get_asp().expect("發生錯誤");
}
