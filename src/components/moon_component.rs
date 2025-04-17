#![allow(non_snake_case)]

use crate::components::moon_image_carousel;
use dioxus::prelude::*;
use serde_json::Value;

#[component]
pub fn MoonComponent() -> Element {
    let title = use_signal(String::new);
    let description = use_signal(String::new);
    let mut description_moon = use_signal(String::new);
    let header = "The Moon.";
    let description_text = "From lighting up our skies to maintaining a geological record of our solar system’s history, Earth’s closest celestial neighbor plays a pivotal role in the study of our planet and our solar system. ";
    const MOON: Asset = asset!("/assets/images/moons.webp");

    use_effect(move || {
        let mut title = title.clone();
        let mut description = description.clone();

        spawn(async move {
            let url = "https://eyes.nasa.gov/apps/solar-system/descriptions/moon.json";

            match reqwest::get(url).await {
                Ok(response) => match response.json::<Value>().await {
                    Ok(json) => {
                        let title_value = json["title"]
                            .as_str()
                            .unwrap_or("No Title found for the Moon.");
                        let description_value = json["description"]["blurb"]
                            .as_str()
                            .unwrap_or("No Description found for Neptune.");
                        let description_more = json["description"]["more"]
                            .as_str()
                            .unwrap_or("No Description 'More' found.");
                        title.set(title_value.to_string());
                        description.set(description_value.to_string());
                        description_moon.set(description_more.to_string());
                    }
                    Err(e) => eprintln!("Error parsing JSON: {}", e),
                },
                Err(e) => eprintln!("Error fetching data: {e}"),
            }
        });
        (|| ())()
    });

    rsx! {
        div { class: "relative w-full h-[90vh] flex items-center pl-12 overflow-hidden",
            img {
                class: "absolute top-0 left-0 w-full h-full object-cover z-[-1] ",
                src: "{MOON}",
            }

            div { class: "z-10 max-w-[800px] text-white bg-black bg-opacity-50 p-5 rounded-lg transform translate-y-[50px]",

                h1 { class: "text-5xl font-bold mb-4", "{header}" }

                p { class: "italic text-xl leading-6", "{description_text}" }
                div { class: "flex flex-row justify-around items-center mt-5 mb-5 ml-4",
                    p { class: "text-white text-2xl mx-5",
                        span { class: "text-[#288bff] text-4xl", "Next full Moon: " }
                        "May 12, 2025"
                    }
                    p { class: "text-white text-2xl mx-5",
                        span { class: "text-[#288bff] text-4xl", "Current Phase (UTC): " }
                        "Waning Gibbous"
                    }
                }
            }
        }

        div { class: "flex flex-col justify-center items-center bg-black/70 text-white p-10 mb-5 rounded-lg shadow-md max-w-3xl w-full mx-auto mt-12 opacity-0 animate-fadeIn animate-slideUp text-justify",
            h1 { class: "text-4xl", "{title}" }
            p { class: "text-lg text-gray-300", "{description}" }
            p { class: "text-lg text-gray-300", "{description_moon}" }
        }
        div { class: "flex flex-col justify-center-mt-10 md:mt-14 items-center w-full h-full",
            moon_image_carousel::MoonImageCarousel {}
        }
    }
}
