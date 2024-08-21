use dioxus::prelude::*;
use reqwest;
use scraper::{Html, Selector};
use log::info;

#[component]
pub fn PostsComponent() -> Element {
    let url = "https://www.anno-union.com/blogs/";

    let future = use_resource(move || async move {
        let request = reqwest::get(url).await;
        match request {
            Ok(response) => {
                let body = response.text().await;
                match body {
                    Ok(body_text) => {
                        info!("Body Text: {}", body_text);
                        let document = Html::parse_document(&body_text);
                        let title_selector = Selector::parse(".col-md-4 p").unwrap();
                        let title = document
                            .select(&title_selector)
                            .next()
                            .map(|element| element.inner_html());
                        info!("Extracted Title: {:?}", title);
                        Ok(title)
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    });

    match &*future.read_unchecked() {
        Some(Ok(Some(title))) => rsx! {
            div {
                h1 {class:"text-4xl flex w-full h-screen items-center justify-center", "{title}" }
            }
        },
        Some(Ok(None)) => rsx! { div { "No title found." } },
        Some(Err(_)) => rsx! { div { "Loading data failed" } },
        None => rsx! { div { "Loading..." } },
    }
}
