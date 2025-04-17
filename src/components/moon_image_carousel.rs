use dioxus::prelude::*;
use freyr::prelude::*;

#[component]
pub fn MoonImageCarousel() -> Element {
    const MOON1: Asset = asset!("/assets/images/apollo.webp");
    const MOON2: Asset = asset!("/assets/images/moon-close.webp");
    const MOON3: Asset = asset!("/assets/images/moon-forest.webp");
    const MOON4: Asset = asset!("/assets/images/sky-moon.jpg");

    let items = vec![
        CarouselItem::new(MOON1, String::from("The Moon's eclipse")),
        CarouselItem::new(MOON2, String::from("The Moon seen like walking on it")),
        CarouselItem::new(MOON3, String::from("The Moon seen from a forest")),
        CarouselItem::new(MOON4, String::from("The Moon seen from the city")),
    ];

    let alt = items.clone();

    rsx! {
        div {
            class: "mb-14",
            h1 { class: "text-2xl md:text-3xl font-bold text-center mb-10", "Discover the Moon in Images:" }
            CarouselWithTimer { 
                items, 
                alt, 
                timer_seconds: 7,
                class: Some(String::from("w-[300px] h-[300px] md:w-[500px] md:h-[500px] rounded-lg hover:shadow-md hover:shadow-slate-300")) }
        }
    }
}
