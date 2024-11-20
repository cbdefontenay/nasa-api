#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn NeptuneComponent() -> Element {
    let title = use_signal(String::new);
    let description = use_signal(String::new);
    let mut description_sun = use_signal(String::new);
    let header = "Neptune.";
    let description_text = "Neptune is the eighth, and most distant planet from the Sun.";
    const _: Asset = asset!("./assets/mars.css");
    const NEPTUNE: Asset = asset!("./assets/neptune.webp");

    use_effect(move || {
        let mut title = title.clone();
        let mut description = description.clone();

        spawn(async move {
            let url = "https://eyes.nasa.gov/apps/solar-system/descriptions/neptune.json";

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
        div { class: "relative w-full h-[90vh] flex items-center pl-12 overflow-hidden opacity-0 animate-fadeIn animate-slideUp duration-1000",
            img {
                class: "absolute top-0 left-0 w-full h-full object-cover z-[-1] opacity-0 animate-fadeIn animate-duration-[1200ms]",
                src: "{NEPTUNE}",
            }

            div { class: "z-10 max-w-[800px] text-white bg-black bg-opacity-50 p-5 rounded-lg transform translate-y-[50px] opacity-0 animate-fadeIn animate-slideUp animate-duration-[1500ms]",

                h1 { class: "text-5xl font-bold mb-4", "{header}" }

                p { class: "italic text-xl leading-6", "{description_text}" }
                div { class: "flex flex-row justify-around items-center mt-5 mb-5 ml-4",
                    p { class: "text-white text-2xl mx-5",
                        span { class: "text-[#288bff] text-4xl", "Distance from the Sun: " }
                        "2.778.949.092 km"
                    }
                    p { class: "text-white text-2xl mx-5",
                        span { class: "text-[#288bff] text-4xl", "Planet type: " }
                        "Ice Giant"
                    }
                }
            }
        }

        div { class: "sun-infos-wrapper",
            h1 { class: "text-4xl", "{title}" }
            p { class: "text-lg text-gray-300", "{description}" }
            p { class: "text-lg text-gray-300", "{description_sun}" }
        }
    }
}