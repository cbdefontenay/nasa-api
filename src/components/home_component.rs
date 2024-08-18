#![allow(non_snake_case)]

use dioxus::prelude::*;
use manganis::*;

#[component]
pub fn HomeComponent() -> Element {
    const ZEUS: ImageAsset = mg!(image("./assets/zeus.jpg"));

    rsx! {
       div {
            class: "h-full w-full grid grid-cols-1 md:grid-cols-2 gap-4 items-center justify-center py-10 bg-slate-300",
            img {
                class: "mx-auto w-3/4 md:w-1/2 h-auto rounded-lg shadow-lg",
                src: "{ZEUS}",
                alt:"Zeus et Athéna"
            }
            div {
                class: "flex flex-col items-center text-center p-4",
                h1 { class: "text-3xl md:text-4xl font-bold font-serif", "Apprends le Grec maintenant!" }
                h3 { class: "text-lg md:text-xl font-bold italic mt-2", "Μάθε ελληνικά τώρα!" }
            }
        }
    }
}
