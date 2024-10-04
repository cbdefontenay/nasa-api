#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::home_component;
use manganis::*;

#[component]
pub fn HeaderComponent() -> Element {
    const LOGO: ImageAsset = mg!(image("./assets/logo.png"));

    rsx! {
        div {
            class: "main-wrapper h-screen overflow-hidden",

            video {
                class: "absolute top-0 left-0 w-full h-full object-cover",
                autoplay:"true",
                muted:"true",
                r#loop: "loop",
                src: "https://cdn.pixabay.com/video/2023/01/20/147239-791344486_large.mp4",
            }
            div {
                class: "relative z-10 flex items-center justify-center h-full",
                div {
                    class:"flex flex-col items-center text-center space-y-6",

                    img {
                        class:"w-24 h-24 rounded-lg bg-transparent",
                        src:"{LOGO}",
                        alt:"Logo of the website"
                    }

                    h1 {
                        class: "text-white text-5xl font-bold font-strait",
                        "Explore the Universe with the NASA."
                    }
                    h3 {
                        class: "text-white text-2xl font-bold italic font-strait",
                        "This website was made using the NASA Open API."
                    }
                }
            }
        }

        home_component::HomeComponent {}
    }
}
