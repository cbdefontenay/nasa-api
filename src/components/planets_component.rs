use dioxus::prelude::*;
use freyr::prelude::*;
use crate::components::card_component;
use manganis::{mg, ImageAsset};

#[component]
pub fn PlanetsComponent() -> Element {
    const _: &str = mg!(file("./assets/planets.css"));
    const MARS: ImageAsset = mg!(image("./assets/marsai.webp"));
    const SUN: ImageAsset = mg!(image("./assets/sun2.webp"));
    const SATURN: ImageAsset = mg!(image("./assets/saturn.webp"));

    rsx! {
        div {
            class: "planets-header",
            h1 { "Learn more about planets." }
        }
        div {
            class: "planets-wrapper",
            card_component::CardComponent {
                title: String::from("Discover Mars"),
                url: String::from("/planets/mars-missions"),
                image: MARS,
            },
            card_component::CardComponent {
                title: String::from("Discover the Sun"),
                url: String::from("/planets/sun"),
                image: SUN,
            },
            card_component::CardComponent {
                title: String::from("Discover Saturn"),
                url: String::from("/planets/saturn"),
                image: SATURN,
            }
        }
    }
}
