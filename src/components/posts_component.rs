#![allow(non_snake_case)]

use dioxus::prelude::*;
use manganis::*;
use regex::Regex;

#[component]
pub fn PostsComponent() -> Element {
    rsx! {
        div {
            h1 { "Posts" }
        }
    }
}

#[server]
async fn fetch_articles() -> Result<String, ServerFnError> {
    let url = "".to_string();

    let response = reqwest::get(&url).await?;
    let status = response.status();

    let html_body = response.text().await?;

    let article_regex = Regex::new(r#"<div\s+class="section-post-filter-item"\s+data-type="post"\s+data-tags="anno-117-en,union-update-en"\s+data-year="2024"\s+data-month="08"\s+data-day="07"\s+data-id="19683"\s+style="">"#)?;

    if status == 200 {
        let article_regex_match = article_regex.captures(&html_body);

        for article in article_regex_match.iter(){

        }
    }

    Ok(String::from("Daten wurden leider nicht gefunden..."))
}