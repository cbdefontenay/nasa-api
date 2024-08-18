#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn NavbarComponent() -> Element {
    rsx! {
        nav {
            class: "bg-teal-900 p-4",
            div {
                class: "container mx-auto flex justify-between items-center",

                // Logo or brand name
                Link {
                    to: "/",
                    class: "hidden md:block text-white text-2xl font-bold",
                    "ελληνικά - Grec"
                }

                // Nav items
                div {
                    class: "flex space-x-4",
                    Link {
                        to: "/",
                        class: "block md:hidden text-white text-lg hover:text-blue-200",
                        "Accueil"
                    }

                    Link {
                        to: "/ma-liste",
                        class: "text-white text-lg hover:text-blue-200",
                        "Ma liste"
                    }

                    Link {
                        to: "/a-propos",
                        class: "text-white text-lg hover:text-blue-200",
                        "A propos"
                    }
                }
            }
        }
    }
}
