#![allow(non_snake_case)]

use dioxus::prelude::*;
use manganis::*;
use serde_json::Value;

#[component]
pub fn SunComponent() -> Element {
    let title = use_signal(String::new);
    let description = use_signal(String::new);
    let header = "Explore the Sun like never before";
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
        div {
            class: "relative h-screen w-full overflow-hidden",

            img {
                src: "{SUN}",
                class: "absolute inset-0 h-full w-full object-cover"
            }

            div {
                class: "absolute inset-0 w-full h-full md:h-screen flex flex-col items-start justify-center pl-8 pr-4",

                h1 {
                    class: "text-white text-3xl font-bold mb-4 font-amsterdam ml-10",
                    "{header}"
                }
            }
        }
    }
}
