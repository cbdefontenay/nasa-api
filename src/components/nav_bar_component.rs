#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn NavBarComponent() -> Element {
    rsx! {
         nav {
            class: "bg-gray-800 p-4",
            div {
                class: "max-w-7xl mx-auto px-2 sm:px-6 lg:px-8",
                div {
                    class: "flex items-center justify-between h-16",
                    // Logo
                    div {
                        class: "flex-shrink-0 text-white",
                        "Your Logo"
                    },
                    // Navbar items (aligned horizontally)
                    div {
                        class: "flex space-x-4",
                        Link {
                            class: "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium",
                            to: "/",
                            "Home"
                        },
                        Link {
                            class: "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium",
                            to: "/about",
                            "About"
                        },
                        a {
                            class: "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium",
                            href: "#",
                            "Services"
                        },
                        a {
                            class: "text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium",
                            href: "#",
                            "Contact"
                        }
                    }
                }
            }
        }
    }
}
