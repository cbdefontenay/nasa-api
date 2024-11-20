#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn PlutoComponent() -> Element {
    let header = "Pluto.";
    let description_text = "Pluto is a dwarf planet located in a distant region of our solar system beyond Neptune known as the Kuiper Belt.";
    let additional_info = "Pluto was long considered our ninth planet, but the International Astronomical Union reclassified Pluto as a dwarf planet in 2006. NASA's New Horizons was the first spacecraft to explore Pluto up close, flying by in 2015. Pluto was discovered in 1930 by astronomer Clyde Tombaugh. It was named by 11-year-old Venetia Burney of Oxford, England.";
    const PLUTO: Asset = asset!("./assets/pluto.webp");

    rsx! {
        div { class: "relative w-full h-[90vh] flex items-center pl-12 overflow-hidden opacity-0 animate-fadeIn animate-slideUp duration-1000",
            img {
                class: "absolute top-0 left-0 w-full h-full object-cover z-[-1] opacity-0 animate-fadeIn animate-duration-[1200ms]",
                src: "{PLUTO}",
            }

            div { class: "z-10 max-w-[800px] text-white bg-black bg-opacity-50 p-5 rounded-lg transform translate-y-[50px] opacity-0 animate-fadeIn animate-slideUp animate-duration-[1500ms]",

                h1 { class: "text-5xl font-bold mb-4", "{header}" }

                p { class: "italic text-xl leading-6 mb-4", "{description_text}" }

                p { class: "text-lg leading-7 text-gray-300", "{additional_info}" }

                div { class: "flex flex-row justify-around items-center mt-5",
                    p { class: "text-white text-2xl mx-5",
                        span { class: "text-[#288bff] text-4xl", "Distance from the Sun: " }
                        "3.266.778.878 km"
                    }
                    p { class: "text-white text-2xl mx-5",
                        span { class: "text-[#288bff] text-4xl", "Planet type: " }
                        "Ice Giant"
                    }
                }
            }
        }
    }
}
