use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::components::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Photo {
    explanation: String,
    title: String,
    hdurl: String,
    date: String,
}

#[component]
pub fn HomeComponent() -> Element {
    let response = use_signal(|| None::<Photo>);
    const _: Asset = asset!("./assets/main.css");

    use_effect(move || {
        let mut response = response.clone();
        spawn(async move {
            let url = format!("https://api.nasa.gov/planetary/apod?api_key={}", env::APP_API_KEY);

            match reqwest::get(&url).await {
                Ok(res) => {
                    match res.json::<Photo>().await {
                        Ok(photo) => response.set(Some(photo)),
                        Err(err) => eprintln!("Error parsing JSON: {}", err),
                    }
                }
                Err(err) => eprintln!("Error fetching data: {}", err),
            }
        });
        (|| ())()
    });

    let photo = response.read_unchecked();
    let photo = photo.as_ref();

    rsx! {
        div { class: "main-wrapper",
            h1 { class: "title", "Picture of the Day" }
            if let Some(photo) = photo {
                div { class: "content",
                    img { src: "{photo.hdurl}", alt: "picture of cosmos" }
                    div { class: "text",
                        h2 { "{photo.title}" }
                        p { "{photo.explanation}" }
                    }
                }
            } else {
                p { class: "loading", "loading data..." }
            }
        }
    }
}
