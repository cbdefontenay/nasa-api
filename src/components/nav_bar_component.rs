#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn NavbarComponent() -> Element {
    rsx! {
        nav {
            class: "bg-teal-900 p-4",
            div {
                class: "container mx-auto flex justify-between items-center",

                Link {
                    to: "/",
                    class: "hidden md:block text-white text-2xl font-bold",
                    "Die Welt von Anno"
                }

                div {
                    class: "flex space-x-4",
                    Link {
                        to: "/",
                        class: "block md:hidden text-white text-lg hover:text-blue-200",
                        "Startseite"
                    }

                    Link {
                        to: "/posts",
                        class: "text-white text-lg hover:text-blue-200",
                        "Posts"
                    }

                    Link {
                        to: "/ueber",
                        class: "text-white text-lg hover:text-blue-200",
                        "Über"
                    }
                }
            }
        }
    }
}
