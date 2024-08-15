#![allow(non_snake_case)]

mod components;

use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {class: "absolute h-screen w-full flex items-center justify-center",
            components::reqwester_component::Reqwester{}
        }
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        components::nav_bar_component::NavBarComponent{}
         Outlet::<Route> {}
    }
}

#[component]
fn About() -> Element {
    rsx! {
        components::about_component::AboutComponent {}
    }
}
