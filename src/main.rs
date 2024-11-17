#![allow(non_snake_case)]

mod components;
mod macros;

use std::env;
use dioxus::prelude::*;
use dioxus::document;
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
        #[route("/planets/saturn")]
        Saturn {},
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app.");
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/planets.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/card.css") }
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
    const LOGO: Asset = asset!("./assets/logo.png");
    const _: Asset = asset!("./assets/main.css");

    let navbar_logo_config = NavbarWithLogoConfig {
        background_color: ColorScheme::Dark,
        nav_items: vec![String::from("Home"), String::from("Planets")],
        nav_links: vec![String::from("/"), String::from("/planets")],
        nav_item_color: NavItemsColor::Light,
        icon_color: IconColor::White,
        logo_url: String::from("/"),
        logo_src: LOGO,
        logo_alt: String::from("logo of NASA"),
    };
    rsx! {
        div {
            class: "nav-wrapper",
            NavbarWithLogo  { navbar_logo_config }
        }
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

#[component]
fn Saturn() -> Element {
    rsx! {
        components::saturn_component::SaturnComponent {}
    }
}
