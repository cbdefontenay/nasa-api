use dioxus::prelude::*;
use freyr::prelude::*;
use crate::components::card_component;

#[component]
pub fn PlanetsComponent() -> Element {
    const MARS: Asset = asset!("./assets/marsai.webp");
    const SUN: Asset = asset!("./assets/sun2.webp");
    const SATURN: Asset = asset!("./assets/saturn.webp");
    const NEPTUNE: Asset = asset!("./assets/neptune.webp");
    const _: Asset = asset!("./assets/planets.css");

    rsx! {
        div { class: "planets-header",
            h1 { "Learn more about planets." }
        }
        div { class: "planets-wrapper",
            card_component::CardComponent {
                title: String::from("Discover Mars"),
                url: String::from("/planets/mars-missions"),
                image: MARS,
            }
            card_component::CardComponent {
                title: String::from("Discover the Sun"),
                url: String::from("/planets/sun"),
                image: SUN,
            }
            card_component::CardComponent {
                title: String::from("Discover Saturn"),
                url: String::from("/planets/saturn"),
                image: SATURN,
            }
            card_component::CardComponent {
                title: String::from("Discover Neptune"),
                url: String::from("/planets/neptune"),
                image: NEPTUNE,
            }
        }
    }
}
