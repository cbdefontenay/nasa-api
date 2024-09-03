use dioxus::prelude::*;
use reqwest::Client;
use manganis::*;
use serde::{Deserialize, Serialize};

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

    use_effect(move || {
        spawn({
            let mut response = response.clone();
            async move {
                match nasa_api().await {
                    Ok(data) => {
                        log::info!("Data fetched successfully!");
                        response.set(Some(data));
                    }
                    Err(err) => {
                        log::info!("Data failed to be fetched! {:?}", err);
                        response.set(None);
                    }
                }
            }
        });
        (|| ())()
    });

    let photo = response.read_unchecked();
    let photo = photo.as_ref();

    rsx! {
        div { class: "flex flex-col items-center justify-center w-full h-full bg-stone-900 pt-14 pb-10 font-strait",
            h1 { class: "text-4xl font-bold mb-10 text-slate-200", "Picture of the Day" }
            if let Some(photo) = photo {
                div { class: "flex flex-row items-start justify-center w-3/4 max-w-4xl",
                    img { class: "rounded-lg shadow-lg mb-4 mt-6 mr-20",
                        src: "{photo.hdurl}",
                        style: "max-width: 400px; height: 400px;",
                        alt: "picture of cosmos"
                    }
                    div { class: "flex flex-col text-gray-200",
                        h2 { class: "text-2xl font-semibold mb-4", "{photo.title}" }
                        p { class: "text-base text-justify", "{photo.explanation}" }
                    }
                }
            } else {
                p { class: "text-lg text-red-500", "loading data..." }
            }
        }
    }
}

#[server]
pub async fn nasa_api() -> Result<Photo, ServerFnError> {
    let client = Client::new();
    let url = "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY";

    let response = client.get(url)
        .send()
        .await?;

    let data: Photo = response.json().await?;

    Ok(data)
}
