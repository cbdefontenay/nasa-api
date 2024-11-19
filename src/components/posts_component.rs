use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Photo {
//     explanation: String,
//     title: String,
//     hdurl: String,
//     date: String,
// }
//
#[component]
pub fn PostsComponent() -> Element {
//     let response = use_signal(|| None::<Photo>);
//
//     use_effect(move || {
//         spawn({
//             let mut response = response.clone();
//             async move {
//                 match nasa_api().await {
//                     Ok(data) => {
//                         log::info!("Data fetched successfully!");
//                         response.set(Some(data));
//                     }
//                     Err(err) => {
//                         log::info!("Data failed to be fetched! {:?}", err);
//                         response.set(None);
//                     }
//                 }
//             }
//         });
//         (|| ())()
//     });
//
//     let photo = response.read_unchecked();
//     let photo = photo.as_ref(); // This reference will live long enough for the rsx! block
//
    rsx! {
        div {
            h1 { "NASA Picture of the Day" }
                // if let Some(photo) = photo {
        //     div {
        //         h2 { "{photo.title}" }
        //         p { "{photo.explanation}" }
        //         img { src: "{photo.hdurl}", alt: "picture of cosmos", style: "max-width: 600px; height: auto;" }
        //     }
        // } else {
        //     p { "No photo available..." }
        // }
        }
    }
}
//
// #[server]
// pub async fn nasa_api() -> Result<Photo, ServerFnError> {
//     let client = Client::new();
//     let url = "https://api.nasa.gov/planetary/apod?api_key=FkPkN10hq7HCUJdK31YREnGXavKLyMALK9ovSFfU";
//
//     let response = client.get(url)
//         .send()
//         .await?;
//
//     let data: Photo = response.json().await?;
//
//     Ok(data)
// }
