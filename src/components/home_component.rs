#![allow(non_snake_case)]

use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::components::env;
// use crate::reqwester;

#[derive(Debug, Deserialize, Serialize)]
pub struct Photo {
    explanation: String,
    title: String,
    hdurl: String,
    date: String,
}

// async fn fetch_and_display_from_api(url: &str, field: &str) -> String {
//     reqwester!(url, field)
// }

#[component]
pub fn HomeComponent() -> Element {
    let response = use_signal(|| None::<Photo>);

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
        div { class: "flex flex-col items-center justify-center w-full h-full bg-stone-900 pt-14 pb-10 font-amsterdam",
            h1 { class: "text-4xl font-bold font-amsterdam mb-10 text-slate-200", "Picture of the Day" }
            if let Some(photo) = photo {
                div { class: "flex flex-row items-start justify-center w-3/4 max-w-4xl",
                    img {
                        class: "rounded-lg shadow-lg mb-4 mt-6 mr-20",
                        src: "{photo.hdurl}",
                        style: "max-width: 400px; height: 400px;",
                        alt: "picture of cosmos"
                    }
                    div { class: "flex flex-col text-gray-200",
                        h2 { class: "text-2xl text-blue-500 font-semibold mb-4", "{photo.title}" }
                        p { class: "text-base text-justify", "{photo.explanation}" }
                    }
                }
            } else {
                p { class: "text-lg text-red-500", "loading data..." }
            }
        }
    }
}
