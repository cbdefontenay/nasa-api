#![allow(non_snake_case)]

use dioxus::prelude::*;
use manganis::*;

#[component]
pub fn MyListComponent() -> Element {

    rsx!{
        h1 {"Ma List de vocab!"}
    }
}