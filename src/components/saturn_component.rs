#![allow(non_snake_case)]

use dioxus::prelude::*;
use manganis::*;
use serde_json::Value;

#[component]
pub fn SaturnComponent() -> Element {
    let title = use_signal(String::new);
    let description = use_signal(String::new);
    let mut description_saturn = use_signal(String::new);
    let header = "Saturn";
    let description_text = "Saturn is the sixth planet from the Sun, and the second largest in the solar system. It’s surrounded by beautiful rings.";
    const SATURN: ImageAsset = mg!(image("./assets/saturn2.webp"));
    const _: &str = mg!(file("./assets/saturn.css"));

    use_effect(move || {
        let mut title = title.clone();
        let mut description = description.clone();

        spawn(async move {
            let url_saturn = "https://eyes.nasa.gov/apps/solar-system/descriptions/saturn.json";

            match reqwest::get(url_saturn).await {
                Ok(response) => {
                    match response.json::<Value>().await {
                        Ok(json) => {
                            let title_value = json["title"].as_str().unwrap_or("Sorry, no 'Title' found.");
                            let description_value = json["description"]["blurb"].as_str().unwrap_or("No Description found.");
                            let description_more = json["description"]["more"].as_str().unwrap_or("No Description found.");
                            title.set(title_value.to_string());
                            description.set(description_value.to_string());
                            description_saturn.set(description_more.to_string());
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
        div { class: "saturn-component-wrapper",

            img {
                class: "saturn-background",
                src: "{SATURN}",
                alt: "Saturn"
            }

            div {
                class: "saturn-content",

                h1 {
                    class: "saturn-title",
                    "{header}"
                }

                p {
                    class: "saturn-description",
                    "{description_text}"
                }
                div {
                    class: "saturn-mission-wrapper",
                    p {
                        class: "saturn-mission",
                        span {
                            class: "saturn-mission-number", "Length of year" }
                        "10 759"
                    }
                    p {
                        class: "saturn-mission",
                        span {
                            class: "saturn-mission-number", "Planet Type" }
                        "Gas Giant"
                    }
                }
            }
        }

        div {
            class: "saturn-infos-wrapper",
            h1 {
                "{title}"
            }
            p {
                class: "saturn-description",
                "{description}"
            }
            p {
                class: "saturn-description-more",
                "{description_saturn}"
            }
        }
    }
}