use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use taiwanreservoir as lib;
use chrono::prelude::*;
use chrono::offset::LocalResult;
use chrono_tz::Asia::Taipei;
use serde_json;


// struct Reservoir {}


#[derive(Serialize, Deserialize, Debug)]
struct ASP {
    #[serde(rename = "__EVENTTARGET")]
    eventtarget: String,
    #[serde(rename = "__EVENTARGUMENT")]
    eventargument: String,
    #[serde(rename = "__LASTFOCUS")]
    lastfocus: String,
    #[serde(rename = "__VIEWSTATE")]
    viewstate: String,
    #[serde(rename = "__VIEWSTATEGENERATOR")]
    viewstategenerator: String,
    #[serde(rename = "ctl00$cphMain$ucDate$cboYear")]
    cboyear: String,
    #[serde(rename = "ctl00$cphMain$ucDate$cboMonth")]
    cbomonth: String,
    #[serde(rename = "ctl00$cphMain$ucDate$cboDay")]
    cboday: String,
    #[serde(rename = "ctl00$cphMain$cboSearch")]
    cbosearch: String,
    #[serde(rename = "__ASYNCPOST")]
    asyncpost: bool,
    #[serde(rename = "ctl00$ctl02")]
    ctl02: String,
    #[serde(rename = "input#ctl00_ctl02_HiddenField")]
    hiddenfield: String,
}

impl ASP {
    fn new(viewstate: &str, viewstategenerator: &str, hiddenfield: &str) -> ASP {
        ASP {
            eventtarget: String::from("ctl00$cphMain$cboSearch"),
            eventargument: String::from(""),
            lastfocus: String::from(""),
            viewstate: String::from(viewstate),
            viewstategenerator: String::from(viewstategenerator),
            cboyear: String::from(""),
            cbomonth: String::from(""),
            cboday: String::from(""),
            cbosearch: String::from(""),
            asyncpost: true,
            ctl02: String::from("ctl00$cphMain$ctl00|ctl00$cphMain$cboSearch"),
            hiddenfield: String::from(hiddenfield),
        }
    }
}

trait ReservoirTrait {
    fn get_asp(&self) -> Result<ASP, reqwest::Error>;
    fn get_realtime(&self);
    fn get_history(&self) -> Vec<lib::Reservoir>;

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
    fn parse_to(&self, html: &str);
}

impl ReservoirTrait for lib::Reservoir {
    fn get_asp(&self) -> Result<ASP, reqwest::Error> {
        let result = self.client().get(self.url()).send();

        let document = Html::parse_document(&result?.text()?);
        // let viewstate_s = Selector::parse(r#"input#__VIEWSTATE"#).unwrap();
        let viewstate = document
            .select(&Selector::parse(r#"input#__VIEWSTATE"#).unwrap())
            .next()
            .and_then(|e| e.value().attr("value"))
            .unwrap();
        let viewstategenerator = document
            .select(&Selector::parse(r#"input#__VIEWSTATEGENERATOR"#).unwrap())
            .next()
            .and_then(|e| e.value().attr("value"))
            .unwrap();
        let hiddenfield = document
            .select(&Selector::parse(r#"input#ctl00_ctl02_HiddenField"#).unwrap())
            .next()
            .and_then(|e| e.value().attr("value"))
            .unwrap();
        Ok(ASP::new(viewstate, viewstategenerator, hiddenfield))

    }

    fn parse_to(&self, html: &str) {
        let document = Html::parse_document(html);
        let table_selector = Selector::parse(r#"table#ctl00_cphMain_gvList.list.nowrap tr:nth-child(n+3):not(:last-child)"#).unwrap();
        let table = document.select(&table_selector).into_iter();
        for x in table { //  for x in table[2..]
            let mut row: Vec<_> = x.text().collect();
            row.retain(|&x| x != "\n\t\t\t");
            println!("{:?}", row);
            let d = lib::ReservoirData{
                name: row[0].to_string(),
                cap_available: 0.0,
                statistic_time_start: 0,
                statistic_time_end: 0,
                rain_fall: 0.0,
                inflow: 0.0,
                outflow: 0.0,
                water_level_diff: 0.0,
                record_time: 0,
                cap_level: 0.0,
                current_cap: 0.0,
                current_cap_percent: 0.0,
            };
            println!("{:?}", d)
        }
        // println!("{:?}", table.inner_html());

    }
    fn get_realtime(&self) {
        let mut asp = self.get_asp().unwrap();
        // 轉換為台北時區
        let now = Utc::now().with_timezone(&Taipei);
        asp.cboday = now.day().to_string();
        asp.cbomonth = now.month().to_string();
        asp.cboyear = now.year().to_string();
        let json_ = serde_json::to_value(&asp).unwrap();
        let mut params: HashMap<String, serde_json::Value> = serde_json::from_value(json_).unwrap();
        let result = self.client().post(self.url())
            .form(&params)
            .send()
            .unwrap()
            .text()
            .unwrap();
        return self.parse_to(&result);
    }

    fn get_history(&self) -> Vec<lib::Reservoir> {
        todo!()
    }
}

fn main() {
    let c = lib::Reservoir {};
    // let result = c.get_asp().expect("發生錯誤");
    let result = c.get_realtime();
    // println!("{:?}", result);
}
