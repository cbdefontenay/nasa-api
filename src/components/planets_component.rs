use dioxus::prelude::*;
use freyr::prelude::*;
use crate::components::card_component;

#[component]
pub fn PlanetsComponent() -> Element {
    const MARS: Asset = asset!("/assets/images/marsai.webp");
    const SUN: Asset = asset!("/assets/images/sun2.webp");
    const SATURN: Asset = asset!("/assets/images/saturn.webp");
    const NEPTUNE: Asset = asset!("/assets/images/neptune.webp");
    const PLUTO: Asset = asset!("/assets/images/pluto.webp");
    const MOON: Asset = asset!("/assets/images/moons.webp");
    const _: Asset = asset!("/assets/planets.css");

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
                title: String::from("Discover the Moon"),
                url: String::from("/planets/moon"),
                image: MOON,
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
            card_component::CardComponent {
                title: String::from("Discover Pluto"),
                url: String::from("/planets/pluto"),
                image: PLUTO,
            }
        }
    }
}
