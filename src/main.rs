#![allow(non_snake_case)]

mod components;

use std::env;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dotenv::dotenv;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/planets/mars-missions")]
        Mars {},
        #[route("/planets/sun")]
        Sun {},
}

fn main() {
    dotenv().ok();
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app.");
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
        components::header_component::HeaderComponent{}
        // components::home_component::HomeComponent{}
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        components::nav_bar_component::NavbarComponent{}
         Outlet::<Route> {}
    }
}

#[component]
fn About() -> Element {
    rsx! {
        components::about_component::AboutComponent {}
    }
}

#[component]
fn Mars() -> Element {
    rsx! {
        components::mars_mission_component::MarsMissionComponent {}
    }
}

#[component]
fn Sun() -> Element {
    rsx! {
        components::sun_component::SunComponent {}
    }
}
