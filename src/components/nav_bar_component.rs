#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn NavbarComponent() -> Element {
    rsx! {
        nav {
            class: "absolute top-0 left-0 w-full bg-black bg-opacity-50 p-4 z-20", // Use absolute positioning
            div {
                class: "container mx-auto flex justify-between items-center",

                Link {
                    to: "/",
                    class: "hidden md:block text-white text-2xl font-bold",
                    "NASA API Discovery"
                }

                div {
                    class: "flex space-x-4",
                    Link {
                        to: "/",
                        class: "block md:hidden text-white text-lg hover:text-blue-200",
                        "Startseite"
                    }

                    Link {
                        to: "/mars-missions",
                        class: "text-white text-lg hover:text-blue-200",
                        "Mars"
                    }

                    Link {
                        to: "/ueber",
                        class: "text-white text-lg hover:text-blue-200",
                        "Rover"
                    }
                }
            }
        }
    }
}
