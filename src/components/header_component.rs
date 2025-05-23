#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::home_component;

#[component]
pub fn HeaderComponent() -> Element {
    const LOGO: Asset = asset!("./assets/images/logo.png");
    const VIDEO: Asset = asset!("./assets/images/header.mp4");
    const _: Asset = asset!("./assets/home.css");

    rsx! {
        div { class: "home-wrapper h-screen overflow-x-hidden",

            video {
                class: "absolute top-0 left-0 w-full h-full object-cover",
                autoplay: "true",
                muted: "true",
                r#loop: "loop",
                src: "{VIDEO}",
            }
            div { class: "relative flex items-center justify-center h-full",
                div { class: "flex flex-col md:flex-row md:justify-between items-center md:items-start w-full md:w-3/4 px-4 space-y-6 md:space-y-0",

                    div { class: "flex-1 md:mr-8 animate-fade-in-left",
                        h1 { class: "text-white text-3xl md:text-5xl font-bold mb-4",
                            "Explore the Cosmos with the NASA."
                        }
                        h3 { class: "text-white text-xl md:text-2xl font-bold italic",
                            "This website was made using the NASA Open API."
                        }
                    }

                    div { class: "hidden md:block h-32 w-px bg-white animate-grow-line" }

                    div { class: "flex-1 flex justify-center items-center md:justify-end animate-fade-in-right",
                        img {
                            class: "w-20 h-20 md:w-40 md:h-40 md:mr-10 rounded-lg bg-transparent",
                            src: "{LOGO}",
                            alt: "Logo of the website",
                        }
                    }
                }
            }
        }

        home_component::HomeComponent {}
    }
}
