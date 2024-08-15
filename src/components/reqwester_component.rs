#![allow(non_snake_case)]

use std::panic;
use dioxus::prelude::*;
use regex::Regex;
use reqwest::Error;

#[component]
pub fn Reqwester() -> Element {
    let url = "https://www.finjadefontenay.de/impressum/".to_string();

    rsx! {
        button {
            onclick: move |_| {
                spawn({
                    let url = url.clone();
                    async move {
                        if let Err(err) = ReqwesterComponent(url).await {
                            println!("Error occurred: {}", err);
                        }
                    }
                });
            },
            "Reqwester"
        }
    }
}

pub async fn ReqwesterComponent(url: String) -> Result<(), Error> {
    let response = reqwest::get(&url).await?;
    let status = response.status();

    let body = response.text().await?;

    if status == 200 {
        let email_regex = Regex::new(r"([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})").unwrap();

        if let Some(capture) = email_regex.captures(&body) {
            if let Some(matched_email) = capture.get(0) {
                println!("Email found: {}", matched_email.as_str());

                let result = panic::catch_unwind(|| {
                    assert_eq!(matched_email.as_str(), "kontakt@finjaboerensen.de");
                });

                match result {
                    Ok(_) => println!("Assertion passed!"),
                    Err(_) => println!("Assertion failed!"),
                }
            }
        } else {
            println!("No email found.");
        }
    } else {
        println!("Status code {}", status)
    }
    Ok(())
}
