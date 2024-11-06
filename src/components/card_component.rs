#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn CardComponent(title: String, image: Asset, url: String) -> Element {
    const _: Asset = asset!("./assets/card.css");

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
