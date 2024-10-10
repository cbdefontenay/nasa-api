#![allow(non_snake_case)]

use dioxus::prelude::*;
use manganis::*;
use serde_json::Value;

#[component]
pub fn SunComponent() -> Element {
    let title = use_signal(String::new);
    let description = use_signal(String::new);
    let header = "Our star: the Sun.";
    let description_text = "The Sun is somehow unreachable, but here is a glimpse of it...";
    const _: &str = mg!(file("./assets/sun.css"));
    const SUN: ImageAsset = mg!(image("./assets/sun.webp"));

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
                            title.set(title_value.to_string());
                            description.set(description_value.to_string());
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

            img {
                class: "sun-background",
                src: "{SUN}"
            }

            div { class: "sun-content",

                h1 { class: "sun-title",
                    "{header}"
                }

                p { class: "sun-description",
                    "{description_text}"
                }
            }
        }
    }
}