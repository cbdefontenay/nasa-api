#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn NavbarComponent() -> Element {
    let navigator = use_navigator();

    rsx! {
        nav {
            class: "absolute top-0 left-0 w-full bg-black bg-opacity-50 p-4 z-20 font-strait",
            div {
                class: "container mx-auto flex justify-between items-center",

                Link {
                    to: "/",
                    class: "hidden md:block text-white text-2xl font-bold",
                    "NASA API Discovery"
                }

                div {
                    class: "flex space-x-4 items-center",

                    Link {
                        to: "/",
                        class: "block md:hidden text-white text-lg hover:text-blue-200",
                        "Startseite"
                    }

                    Link {
                        to: "/planets/mars-missions",
                        class: "text-white text-lg hover:text-blue-200",
                        "Mars"
                    }

                    select {
                        class: "text-slate-200 text-lg bg-transparent border border-gray-500 rounded-md px-2 py-1 appearance-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                        onchange: move |e| {
                            let selected_value = e.value().clone();
                            if !selected_value.is_empty() {
                                navigator.push(&selected_value);
                            }
                        },
                        option { value: "", disabled: "true", selected: "true", "Select a Planet" }
                        option { value: "/planets/mars-missions", "Mars" }
                        option { value: "/planets/sun", "Sun" }
                        option { value: "/planets/venus", "Venus" }
                    }
                }
            }
        }
    }
}
