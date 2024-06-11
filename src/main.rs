#![allow(non_snake_case)]

use std::collections::HashSet;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    wasm_logger::init(wasm_logger::Config::default());
    get_countries();
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx!(
        GeoChart {}
    )
}
#[derive(Debug)]
struct RestaurantInfo {
    country_name:String,
    visit_date:u32
}


fn build_restaurant_info(country_name: String, visit_date:u32) -> RestaurantInfo {
    RestaurantInfo {
        visit_date,
        country_name
    }
}

//MEMO:もっといい方法でデータ登録したい
//TODO:データベースでの管理に変える
fn get_restaurant_info() ->Vec<RestaurantInfo> {
    let v = vec![
        (2019_0000,"Japan"),
        (2020_0000,"Ireland"),
        (2020_0000,"Mexico"),
        (2020_0000,"Turkey"),
        (2020_0000,"Indonesia"),
        (2020_0000,"Morocco"),
        (2020_0604,"Japan"),
        (2021_0320,"Spain"),
        (2021_0320,"China"),
        (2021_0407,"US"),
        (2021_0407,"KP"),
        (2021_0610,"China"),
        (2021_0610,"Italy"),
        (2021_1204,"Slovenia"),
        (2021_1204,"Lebanon"),
        (2022_0703,"Germany"),
        (2022_1007,"Russia" ),
        (2022_1113,"India" ),
        (2022_1218,"New Zealand"),
        (2022_1218,"Jamaica" ),
        (2023_0225,"VietNam"),
        (2023_0225,"Thailand"),
        (2023_0422,"Portugal"),
        (2023_0625,"Chile"),
        (2023_0923,"Israel"),
        (2023_0923,"Italia"),
        (2023_1015,"Denmark"),
        (2023_1119,"France"),
        (2023_1211,"Pakistan"),
        (2023_1211,"Greece"),
        (2024_0423,"Finland"),
        (2024_0423,"Poland")
    ];

    let mut restaurants = vec![];
    for (date, country_name) in v {
        restaurants.push(build_restaurant_info(country_name.to_string(),date));
    }
    restaurants
}

fn get_country_count() -> usize{
    let restaurants = get_restaurant_info();
    let mut countries:HashSet<String> = HashSet::new();

    for restaurant in restaurants {
        countries.insert(restaurant.country_name);
    }
    countries.len()
}
/// https://developers.google.com/chart/interactive/docs/gallery/geochart
fn get_countries()->String {
    let restaurants = get_restaurant_info();

    let mut array_string = String::new();
    array_string+="\t['Country', 'Date'],\n";

    for restaurant in restaurants {
        let tmp = format!("\t['{}',{}],\n",restaurant.country_name.clone(),restaurant.visit_date);
        array_string+=&tmp;
    }

    array_string=(&array_string[0..array_string.len()-2]).to_string();

    println!("{}",array_string);

    let s = r#"
    google.charts.load('current', {
        'packages':['geochart'],
    });

    function selectHandler(reg) {
        alert('あなたがクリックした国の国名コードは' + reg.region + 'です。');
    }

    google.charts.setOnLoadCallback(drawRegionsMap);
    function drawRegionsMap() {
        var data = google.visualization.arrayToDataTable(["#.to_string()
        + format!("{}",array_string).as_str()
        + r#"
        ]);
        var options = {
            geochartVersion:11,
            backgroundColor:'#81d4fa'
        };
        var chart = new google.visualization.GeoChart(document.getElementById('regions_div'));
        google.visualization.events.addListener(chart, 'regionClick', selectHandler);
        chart.draw(data, options);
    }"#;
    s
}

//TODO:ええ感じのレイアウトにする
#[component]
fn GeoChart() -> Element {
    let countries = get_countries();    
    
    let dummy = use_signal(||"");
    let regions = r#"<div id="regions_div" style="width: 1600px; height: 1200px;"></div>"#;

    
    const  MAX_COUNTRY_NUM:usize = 196;
    let country_count = get_country_count();
    //let percentage = ((country_count as f64 / MAX_COUNTRY_NUM as f64) * 1000.0).round() / 10.0;
    let s_percentage = format!("{:.1}",(country_count as f64 / MAX_COUNTRY_NUM as f64) * 100.0);
    
    rsx! {
        script {
            r#type: "text/javascript",
            src: "https://www.gstatic.com/charts/loader.js"
        }
        script { r#type: "text/javascript", "{countries}" }
        div { dangerous_inner_html: "{regions}" }
        h1 { "異国飯地図" }
        h2 { "今までに{country_count}カ国の異国料理を食べたよ🌎" }
        h2 { "進捗率:{s_percentage}%" }
        h1 {
            div {
                progress { max: MAX_COUNTRY_NUM.to_string(), value: 30 }
                "  {country_count} / 196カ国"
            }
        }
        //FIXME:signalを2回連続して呼ばないと地図が描画されない
        text { "{dummy}" }
        text { "{dummy}" }
    }
}