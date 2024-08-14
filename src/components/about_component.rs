#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn AboutComponent() -> Element {
    rsx! {
        div { class:"flex items-center justify-center w-full h-screen",
        h1 { class:"text-2xl font-sans font-4xl font-bold text-red-500",
            "About Page"}
        }
    }
}