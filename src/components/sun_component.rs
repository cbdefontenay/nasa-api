#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn SunComponent() -> Element {
    let title = use_signal(String::new);
    let description = use_signal(String::new);
    let mut description_sun = use_signal(String::new);
    let header = "Our star: the Sun.";
    let description_text = "The Sun is somehow unreachable, but here is a glimpse of it...";
    const _: Asset = asset!("./assets/sun.css");
    const SUN: Asset = asset!("./assets/sun.webp");

    use_effect(move || {
        let mut title = title.clone();
        let mut description = description.clone();

        spawn(async move {
            let url = "https://eyes.nasa.gov/apps/solar-system/descriptions/sun.json";

            match reqwest::get(url).await {
                Ok(response) => {
                    match response.json::<Value>().await {
                        Ok(json) => {
                            let title_value = json["title"].as_str().unwrap_or("No Title found.");
                            let description_value = json["description"]["blurb"].as_str().unwrap_or("No Description found.");
                            let description_more = json["description"]["more"].as_str().unwrap_or("No Description 'More' found.");
                            title.set(title_value.to_string());
                            description.set(description_value.to_string());
                            description_sun.set(description_more.to_string());
                        }
                        Err(e) => eprintln!("Error parsing JSON: {}", e)
                    }
                }
                Err(e) => eprintln!("Error fetching data: {e}")
            }
        });
        (|| ())()
    });

    rsx! {
        div { class: "sun-component-wrapper",

            img { class: "sun-background", src: "{SUN}" }

            div { class: "sun-content",

                h1 { class: "sun-title", "{header}" }

                p { class: "sun-description", "{description_text}" }
                div { class: "sun-mission-wrapper",
                    p { class: "sun-mission",
                        span { class: "sun-mission-number", "18" }
                        " Active Missions"
                    }
                    p { class: "sun-mission",
                        span { class: "sun-mission-number", "13" }
                        " Upcoming Missions"
                    }
                }
            }
        }

        div { class: "sun-infos-wrapper",
            h1 { "{title}" }
            p { class: "sun-description", "{description}" }
            p { class: "sun-description-more", "{description_sun}" }
        }
    }
}