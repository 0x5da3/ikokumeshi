use std::collections::{hash_map, HashMap};

use super::*;
use crate::consts;
use consts::GEO_FUNCTION;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx!(GeoChart {})
}

#[component]
fn Flags() -> Element {
    let countries = get_restaurant_info();

    let mut img_urls: HashMap<String, (String, String)> = std::collections::HashMap::new();
    let mut img_urls = use_signal(|| img_urls);

    let mut count = 0;
    for c in countries {
        let country_name = c.country_name;
        let url = format!("https://restcountries.com/v3.1/alpha/{}", country_name);
        if img_urls.read().get(&country_name).is_none() {
            spawn(async move {
                let resp = reqwest::get(url.to_string()).await.unwrap().text().await;
                match resp {
                    Ok(data) => {
                        let serialized: Value = serde_json::from_str(data.as_str()).unwrap();
                        let flags = &serialized[0]["flags"]["png"].to_string()[1..];
                        let len = flags.len();
                        let flags = &flags[..len - 1].to_string();
                        img_urls
                            .write()
                            .insert(country_name.to_string(), (flags.clone(), c.url.to_string()));
                    }
                    Err(err) => {
                        tracing::info!("ng {err}");
                    }
                }
            });
        }
    }

    rsx!(
        div { class: "card-container",
            for (code , (flag_url , restrant_url)) in img_urls.read().iter() {
                Flag { code, flag_url, restrant_url }
            }
        }
    )
}

#[component]
fn Flag(code: String, flag_url: String, restrant_url: String) -> Element {
    rsx!(
        div { class: "country-card",
            h3 { class: "country-name", "{code}" }
            img { src: "{flag_url}", class: "country-image" }
            br {}
            a { href: "{restrant_url}", class: "store-link", "link" }
        }
    )
}

//TODO:ええ感じのレイアウトにする
#[component]
fn GeoChart() -> Element {
    let countries = get_countries();
    let regions = r#"<div id="regions_div" style="width: 1200px; height: 600px;"></div>"#;

    const MAX_COUNTRY_NUM: usize = 196;
    let country_count = get_country_count();

    //let percentage = ((country_count as f64 / MAX_COUNTRY_NUM as f64) * 1000.0).round() / 10.0;
    let s_percentage = format!(
        "{:.1}",
        (country_count as f64 / MAX_COUNTRY_NUM as f64) * 100.0
    );

    //    tracing::info!("{img_urls:#?}");

    //https://www.gstatic.com/charts/loader.js
    rsx! {
        script { r#type: "text/javascript", {GEO_FUNCTION} }
        script { r#type: "text/javascript", "{countries}" }
        link { rel: "stylesheet", href: "main.css" }
        div { dangerous_inner_html: "{regions}" }
        form {
            div {
                onsubmit: move |e| { tracing::info!("{e:?}") },
                id: "regions_div"
            }
        }
        h1 { "異国飯地図" }
        h2 { "今までに{country_count}カ国の異国料理を食べたよ🌎" }
        h2 { "進捗率:{s_percentage}%" }
        h1 {
            div {
                progress { max: MAX_COUNTRY_NUM.to_string(), value: 30 }
                "  {country_count} / 196カ国"
            }
        }
        Flags {}
    }
}
