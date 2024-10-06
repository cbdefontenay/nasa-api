#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::home_component;
use manganis::*;

#[component]
pub fn HeaderComponent() -> Element {
    const LOGO: ImageAsset = mg!(image("./assets/logo.png"));

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
                class: "relative flex items-center justify-center h-full",
                div {
                    class:"flex flex-col md:flex-row md:justify-between items-center md:items-start w-full md:w-3/4 px-4 space-y-6 md:space-y-0",

                    div {
                        class:"flex-1 md:mr-8 animate-fade-in-left",
                        h1 {
                            class: "text-white text-3xl md:text-5xl font-bold font-strait mb-4",
                            "Explore the Universe with NASA."
                        }
                        h3 {
                            class: "text-white text-xl md:text-2xl font-bold italic font-strait",
                            "This website was made using the NASA Open API."
                        }
                    }

                    div {
                        class: "hidden md:block h-32 w-px bg-white animate-grow-line"
                    }

                    div {
                        class: "flex-1 flex justify-center items-center md:justify-end animate-fade-in-right",
                        img {
                            class: "w-20 h-20 md:w-24 md:h-24 rounded-lg bg-transparent",
                            src: "{LOGO}",
                            alt: "Logo of the website"
                        }
                    }
                }
            }
        }

        home_component::HomeComponent {}
    }
}
