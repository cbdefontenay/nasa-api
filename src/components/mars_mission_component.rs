#![allow(non_snake_case)]

use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use rand::Rng;

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
    let response = use_signal(Vec::<Photo>::new);

    use_effect(move || {
        spawn({
            let mut response = response.clone();
            async move {
                match nasa_api().await {
                    Ok(data) => {
                        log::info!("Data fetched");
                        response.set(data);
                    }
                    Err(err) => {
                        log::info!("Data not fetched");
                        response.set(Vec::new());
                    }
                }
            }
        });
        (|| ())()
    });


    let random_index = rand::thread_rng().gen_range(0..=6);
    let random_index_two = rand::thread_rng().gen_range(0..=6);

    rsx! {
        div { class: "bg-stone-900 w-full h-full flex flex-col items-center justify-center text-slate-200",
            h1 {class:"text-2xl font-bold font-serif mt-10 mb-10", "NASA Mars Photos"}
            if let Some(photo) = response.get(random_index) {
                div {
                    h2 { "{photo.rover.name}"}
                    p { "Date: {photo.earth_date}"}
                    img { src: "{photo.img_src}", alt: "Picture of Mars",
                        class:"rounded-lg shadow-md shadow-slate-600 mb-10",
                        style: "max-width: 500px; height: 500px;", }
                }
            }
            if let Some(photo) = response.get(random_index_two) {
                div {
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

#[server]
pub async fn nasa_api() -> Result<Vec<Photo>, ServerFnError> {
    let client = Client::new();
    let url = "https://api.nasa.gov/mars-photos/api/v1/rovers/curiosity/photos?sol=1000&api_key=FkPkN10hq7HCUJdK31YREnGXavKLyMALK9ovSFfU";

    let response = client.get(url)
        .send()
        .await?;

    let data: serde_json::Value = serde_json::from_str(&response.text().await?)?;
    let photos: Vec<Photo> = serde_json::from_value(data["photos"].clone())?;

    Ok(photos)
}
