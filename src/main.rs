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
        #[route("/ueber")]
        About {},
        #[route("/posts")]
        Posts {},
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
        components::home_component::HomeComponent{}
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
fn Posts() -> Element {
    rsx! {
        components::posts_component::PostsComponent {}
    }
}
