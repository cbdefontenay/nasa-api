#![allow(non_snake_case)]

use dioxus::prelude::*;
use manganis::mg;
use serde_json::Value;

#[component]
pub fn MarsExplanationComponent() -> Element {
    let explanation = use_signal(String::new);
    let description = use_signal(String::new);
    let distance = use_signal(String::new);
    let radius = use_signal(String::new);
    let volume = use_signal(String::new);
    let density = use_signal(String::new);
    let mass = use_signal(String::new);
    let area = use_signal(String::new);
    let gravity = use_signal(String::new);
    let eccentricity = use_signal(String::new);
    let inclination = use_signal(String::new);
    let velocity = use_signal(String::new);
    let circumference = use_signal(String::new);

    let header = "The planet Mars and its secrets revealed.";

    use_effect(move || {
        let mut explanation = explanation.clone();
        let mut description = description.clone();
        let mut distance = distance.clone();
        let mut radius = radius.clone();
        let mut volume = volume.clone();
        let mut density = density.clone();
        let mut mass = mass.clone();
        let mut area = area.clone();
        let mut gravity = gravity.clone();
        let mut eccentricity = eccentricity.clone();
        let mut inclination = inclination.clone();
        let mut velocity = velocity.clone();
        let mut circumference = circumference.clone();

        spawn(async move {
            let url = "https://eyes.nasa.gov/apps/solar-system/descriptions/mars.json";

            match reqwest::get(url).await {
                Ok(response) => {
                    match response.json::<Value>().await {
                        Ok(json) => {
                            let explanation_value = json["description"]["more"].as_str().unwrap_or("No Title found in the JSON.");
                            let description_value = json["description"]["blurb"].as_str().unwrap_or("No Description found.");
                            explanation.set(explanation_value.to_string());
                            description.set(description_value.to_string());

                            let distance_value = json["data"]["distance"].as_i64().map_or("N/A".to_string(), |v| v.to_string());
                            let radius_value = json["data"]["radius"].as_f64().map_or("N/A".to_string(), |v| v.to_string());
                            let volume_value = json["data"]["volume"].as_f64().map_or("N/A".to_string(), |v| v.to_string());
                            let density_value = json["data"]["density"].as_f64().map_or("N/A".to_string(), |v| v.to_string());
                            let mass_value = json["data"]["mass"].as_f64().map_or("N/A".to_string(), |v| v.to_string());
                            let area_value = json["data"]["area"].as_f64().map_or("N/A".to_string(), |v| v.to_string());
                            let gravity_value = json["data"]["gravity"].as_f64().map_or("N/A".to_string(), |v| v.to_string());
                            let eccentricity_value = json["data"]["eccentricity"].as_f64().map_or("N/A".to_string(), |v| v.to_string());
                            let inclination_value = json["data"]["inclination"].as_f64().map_or("N/A".to_string(), |v| v.to_string());
                            let velocity_value = json["data"]["velocity"].as_i64().map_or("N/A".to_string(), |v| v.to_string());
                            let circumference_value = json["data"]["circumference"].as_f64().map_or("N/A".to_string(), |v| v.to_string());

                            distance.set(distance_value);
                            radius.set(radius_value);
                            volume.set(volume_value);
                            density.set(density_value);
                            mass.set(mass_value);
                            area.set(area_value);
                            gravity.set(gravity_value);
                            eccentricity.set(eccentricity_value);
                            inclination.set(inclination_value);
                            velocity.set(velocity_value);
                            circumference.set(circumference_value);
                        }
                        Err(e) => eprintln!("Error parsing JSON: {}", e),
                    }
                }
                Err(e) => eprintln!("Error fetching data: {}", e),
            }
        });
        (|| ())()
    });

     const _: &str = mg!(file("./assets/mars.css"));

    rsx! {
        div { class: "mars-explanation-container",

            div { class: "mars-data-section",
                h2 { class: "mars-data-header", "Mars Data" }
                ul { class: "mars-data-list",
                    li { class: "data-item", "Distance: {distance}" }
                    li { class: "data-item", "Radius: {radius}" }
                    li { class: "data-item", "Volume: {volume}" }
                    li { class: "data-item", "Density: {density}" }
                    li { class: "data-item", "Mass: {mass}" }
                    li { class: "data-item", "Area: {area}" }
                    li { class: "data-item", "Gravity: {gravity}" }
                    li { class: "data-item", "Eccentricity: {eccentricity}" }
                    li { class: "data-item", "Inclination: {inclination}" }
                    li { class: "data-item", "Velocity: {velocity}" }
                    li { class: "data-item", "Circumference: {circumference}" }
                }
            }

            div { class: "mars-description-section",
                h1 { class: "mars-description-header", "{header}" }
                div { class: "mars-description-text",
                    p { class: "description", "{description}" }
                    p { class: "explanation", "{explanation}" }
                }
            }
        }
    }
}
