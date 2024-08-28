#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn HeaderComponent() -> Element {
    rsx! {
        div {
            class: "h-screen overflow-hidden",

            video {
                class: "absolute top-0 left-0 w-full h-full object-cover",
                autoplay:"true",
                muted:"true",
                r#loop: "loop",
                src: "https://cdn.pixabay.com/video/2023/01/20/147239-791344486_large.mp4",
            }
            div {
                class: "relative z-10 flex items-center justify-center h-full",
                h1 {
                    class: "text-white text-5xl font-bold font-strait",
                    "Discover the Earth and Mars with the NASA."
                }
            }
        }
    }
}