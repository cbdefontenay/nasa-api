#![allow(non_snake_case)]

use dioxus::prelude::*;
use manganis::{mg, ImageAsset};

#[component]
pub fn CardComponent(title: String, image: ImageAsset, url: String) -> Element {
    const _: &str = mg!(file("./assets/card.css"));

    rsx! {
        div {
            class: "card",
            div {
                class: "card-image",
                img {
                    src: "{image}",
                    alt: "Card Image"
                }
                div {
                    class: "card-title",
                    Link {
                        to: "{url}",
                        "{title}"
                    }
                }
            }
        }
    }
}
