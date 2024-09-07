#![allow(non_snake_case)]
pub mod components;
pub mod consts;
pub mod enums;
pub mod structs;

use dioxus::prelude::*;
use enums::Route;
use serde_json::Value;
use std::{collections::HashSet, ptr::null};
use structs::RestaurantInfo;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    get_countries();
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn build_restaurant_info(country_name: String, visit_date: u32, url: String) -> RestaurantInfo {
    RestaurantInfo {
        visit_date,
        country_name,
        url,
    }
}

pub fn parse_json(input_url: String, mut return_url: String) {
    if input_url.is_empty() {
        tracing::info!("url is null");
    }

    spawn(async move {
        let resp = reqwest::get(input_url.to_string())
            .await
            .unwrap()
            .text()
            .await;

        match resp {
            Ok(data) => {
                let json = data.as_str();
                //tracing::info!("{json}");
                let serialized: Value = serde_json::from_str(json).unwrap();
                let flags = &serialized[0]["flags"]["png"];
                //tracing::info!("{flags}");
                //tracing::info!("{serialized:#?}");
                return_url = flags.to_string();
            }
            Err(err) => {
                tracing::info!("ng {err}");
                panic!()
            }
        }
    });
}

//MEMO:もっといい方法でデータ登録したい
//TODO:データベースでの管理に変える
fn get_restaurant_info() -> Vec<RestaurantInfo> {
    let v = vec![
        (2019_0000, "JP", ""),
        (
            2020_0000,
            "IE",
            "https://tabelog.com/kyoto/A2601/A260202/26005894",
        ),
        (
            2020_0000,
            "MX",
            "https://tabelog.com/kyoto/A2601/A260202/26008970/",
        ),
        (
            2020_0000,
            "TR",
            "https://tabelog.com/kyoto/A2601/A260202/26002357/",
        ),
        (
            2020_0000,
            "ID",
            "https://tabelog.com/kyoto/A2601/A260202/26013974/",
        ),
        (
            2020_0000,
            "MA",
            "https://tabelog.com/kyoto/A2601/A260201/26017146/",
        ),
        (
            2020_0604,
            "JP",
            "https://tabelog.com/kyoto/A2601/A260101/26004181/",
        ),
        (
            2021_0320,
            "ES",
            "https://tabelog.com/kyoto/A2601/A260202/26027532/",
        ),
        (
            2021_0320,
            "CN",
            "https://tabelog.com/kyoto/A2601/A260201/26002804/",
        ),
        (
            2021_0407,
            "US",
            "https://tabelog.com/kyoto/A2601/A260304/26029551/",
        ),
        (
            2021_0407,
            "KR",
            "https://tabelog.com/kyoto/A2601/A260301/26021466/",
        ),
        (
            2021_0610,
            "CN",
            "https://tabelog.com/shiga/A2502/A250201/25001136",
        ),
        (
            2021_0610,
            "IT",
            "https://tabelog.com/shiga/A2502/A250201/25000137/",
        ),
        (
            2021_1204,
            "SI",
            "https://tabelog.com/kyoto/A2601/A260402/26005632/",
        ),
        (
            2021_1204,
            "LB",
            "https://tabelog.com/kyoto/A2601/A260201/26035998/",
        ),
        (
            2022_0703,
            "DE",
            "https://tabelog.com/kyoto/A2601/A260201/26021597",
        ),
        (
            2022_1007,
            "RU",
            "https://tabelog.com/kyoto/A2601/A260503/26025717",
        ),
        (
            2022_1113,
            "IN",
            "https://tabelog.com/kyoto/A2601/A260203/26002103",
        ),
        (
            2022_1218,
            "NZ",
            "https://tabelog.com/hyogo/A2803/A280303/28002375",
        ),
        (
            2022_1218,
            "JM",
            "https://tabelog.com/hyogo/A2801/A280101/28000418/",
        ),
        (
            2023_0225,
            "VN",
            "https://tabelog.com/kyoto/A2601/A260201/26019324/",
        ),
        (
            2023_0225,
            "TH",
            "https://tabelog.com/kyoto/A2601/A260304/26003047/",
        ),
        (
            2023_0422,
            "PT",
            "https://tabelog.com/kyoto/A2601/A260302/26032201/",
        ),
        (
            2023_0625,
            "CL",
            "https://tabelog.com/hyogo/A2801/A280101/28000386/",
        ),
        (
            2023_0923,
            "IL",
            "https://tabelog.com/kyoto/A2601/A260202/26034349/",
        ),
        (
            2023_1015,
            "DK",
            "https://tabelog.com/kyoto/A2601/A260604/26036045/",
        ),
        (
            2023_1119,
            "FR",
            "https://tabelog.com/kyoto/A2601/A260202/26005029/",
        ),
        (
            2023_1211,
            "PK",
            "https://tabelog.com/osaka/A2701/A270407/27072992/",
        ),
        (
            2023_1211,
            "GR",
            "https://tabelog.com/osaka/A2701/A270108/27011869/",
        ),
        (
            2024_0423,
            "FI",
            "https://tabelog.com/kyoto/A2601/A260501/26032573/",
        ),
        (
            2024_0423,
            "PL",
            "https://tabelog.com/kyoto/A2601/A260302/26036734/",
        ),
        (
            2024_0706,
            "TZ",
            "https://tabelog.com/kyoto/A2601/A260601/26027006/",
        ),
        (
            2024_0929,
            "EE",
            "https://tabelog.com/osaka/A2701/A270101/27141717/",
        ),
        (
            2024_0929,
            "MY",
            "https://tabelog.com/osaka/A2701/A270103/27100262/",
        ),
    ];

    let mut restaurants = vec![];
    for (date, country_name, url) in v {
        restaurants.push(build_restaurant_info(
            country_name.to_string(),
            date,
            url.to_string(),
        ));
    }
    restaurants
}

pub fn get_country_count() -> usize {
    let restaurants = get_restaurant_info();
    let mut countries: HashSet<String> = HashSet::new();

    for restaurant in restaurants {
        countries.insert(restaurant.country_name);
    }
    countries.len()
}

/// https://developers.google.com/chart/interactive/docs/gallery/geochart
pub fn get_countries() -> String {
    let restaurants = get_restaurant_info();

    let mut array_string = String::new();
    array_string += "\t['Country', 'Date'],\n";

    for restaurant in restaurants {
        let tmp = format!(
            "\t['{}',{}],\n",
            restaurant.country_name.clone(),
            restaurant.visit_date
        );
        array_string += &tmp;
    }

    array_string = (&array_string[0..array_string.len() - 2]).to_string();

    println!("{}", array_string);

    //alert('あなたがクリックした国の国名コードは' + reg.region + 'です。');

    let s = r#"
    google.charts.load('current', {
        'packages':['geochart'],
    });

    google.charts.setOnLoadCallback(drawRegionsMap);

    function selectHandler(reg) {
        console.log(reg.region)
    }
    
    function drawRegionsMap() {
        var data = google.visualization.arrayToDataTable(["#
        .to_string()
        + format!("{}", array_string).as_str()
        + r#"
        ]);
        var options = {
            backgroundColor:'#81d4fa'
        };
        var chart = new google.visualization.GeoChart(document.getElementById('regions_div'));
        google.visualization.events.addListener(chart, 'regionClick', selectHandler);
        chart.draw(data, options);
    }"#;
    s
}
