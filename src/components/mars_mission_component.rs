#![allow(non_snake_case)]

use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use rand::Rng;
use serde_json::Value;
use crate::components::env;

#[derive(Deserialize, Serialize, Debug)]
pub struct Photo {
    id: i32,
    sol: i32,
    camera: Camera,
    img_src: String,
    earth_date: String,
    rover: Rover,
}

#[derive(Deserialize, Serialize, Debug)]
struct Camera {
    id: i32,
    name: String,
    rover_id: i32,
    full_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Rover {
    id: i32,
    name: String,
    landing_date: String,
    launch_date: String,
    status: String,
}


#[component]
pub fn MarsMissionComponent() -> Element {
    let mut response = use_signal(Vec::<Photo>::new);

    use_effect(move || {
        spawn(async move {
            let client = Client::new();
            let url = format!(
                "https://api.nasa.gov/mars-photos/api/v1/rovers/curiosity/photos?sol=1000&api_key={}",
                env::API_KEY);

            match client.get(&url).send().await {
                Ok(resp) => {
                    match resp.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<Value>(&text) {
                                Ok(data) => {
                                    if let Some(photos) = data["photos"].as_array() {
                                        match serde_json::from_value::<Vec<Photo>>(Value::Array(photos.to_vec()))
                                        {
                                            Ok(photos) => response.set(photos),
                                            Err(e) => eprintln!("Error deserializing photos: {}", e)
                                        }
                                    } else {
                                        eprintln!("Error: 'photos' field not found in JSON data...")
                                    }
                                }
                                Err(e) => eprintln!("Error parsing JSON: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Error reading response text: {}", e),
                    }
                }
                Err(e) => eprintln!("Error sending request: {}", e),
            }
        });
        (|| ())()
    });


    let random_index = rand::thread_rng().gen_range(0..=50);
    let random_index_two = rand::thread_rng().gen_range(45..=210);
    let random_index_three = rand::thread_rng().gen_range(105..=147);

    rsx! {
         div {
            class: "h-screen overflow-hidden",

            video {
                class: "absolute top-0 left-0 w-full h-full object-cover",
                autoplay:"true",
                muted:"true",
                r#loop: "true",
                src: "https://cdn.pixabay.com/video/2022/12/15/143020-781982535_large.mp4",
            }
            div {
                class: "relative z-10 flex items-center justify-center h-full",
                h1 {
                    class: "text-white text-5xl font-bold font-strait",
                    "Go on exploration on Mars."
                }
            }
        }
       div { class: "bg-stone-900 w-full h-full flex flex-col items-center justify-center text-slate-200",
            h1 {class:"text-2xl font-bold font-strait mt-10 mb-10", "NASA Mars Photos"}
            div {class: "flex gap-6",
                if let Some(photo) = response.get(random_index) {
                    div {class: "flex flex-col items-center",
                        h2 { "{photo.rover.name}"}
                        p { "Date: {photo.earth_date}"}
                        img { src: "{photo.img_src}", alt: "Picture of Mars",
                            class:"rounded-lg shadow-md shadow-slate-600 mb-10",
                            style: "max-width: 500px; height: 500px;", }
                    }
                }
                if let Some(photo) = response.get(random_index_two) {
                    div {class: "flex flex-col items-center",
                        h2 { "{photo.rover.name}"}
                        p { "Date: {photo.earth_date}"}
                        img { src: "{photo.img_src}", alt: "Picture of Mars",
                            class:"rounded-lg shadow-md shadow-slate-600 mb-10",
                            style: "max-width: 500px; height: 500px;", }
                    }
                } else {
                    p {"Data are being loaded, please wait a moment..."}
                }
                if let Some(photo) = response.get(random_index_three) {
                    div {class: "flex flex-col items-center",
                        h2 { "{photo.rover.name}"}
                        p { "Date: {photo.earth_date}"}
                        img { src: "{photo.img_src}", alt: "Picture of Mars",
                            class:"rounded-lg shadow-md shadow-slate-600 mb-10",
                            style: "max-width: 500px; height: 500px;", }
                    }
                } else {
                    p {"Data are being loaded, please wait a moment..."}
                }
            }
        }
    }
}