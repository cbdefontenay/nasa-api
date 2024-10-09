#![allow(non_snake_case)]

mod components;
mod macros;

use std::env;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use freyr::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/planets")]
        Planets {},
        #[route("/about")]
        About {},
        #[route("/planets/mars-missions")]
        Mars {},
        #[route("/planets/sun")]
        Sun {},
}

fn main() {
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
    }
}

#[component]
fn NavBar() -> Element {
    let navbar_config = NavbarConfig {
        background_color: ColorScheme::Dark,
        nav_header: "NASA Discovery".to_string(),
        nav_items: vec!["Home".to_string(), "Planets".to_string()],
        nav_links: vec!["/".to_string(), "/planets".to_string()],
        nav_item_color: NavItemsColor::Light,
        icon_color: IconColor::Custom("#c61aff"),
    };
    rsx! {
        Navbar { navbar_config }
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
fn Planets() -> Element {
    rsx! {
        components::planets_component::PlanetsComponent {}
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
