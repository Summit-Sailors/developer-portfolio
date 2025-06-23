#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionTitleProps {
    title: String,
    class: Option<String>,
}

#[component]
pub fn SectionTitle(props: SectionTitleProps) -> Element {
    let additional_class = props.class.unwrap_or_default();
    rsx! {
        div {
            class: "flex flex-col items-center mb-16 animate-fade-in-up {additional_class}",
            h2 { class: "text-4xl font-bold mb-4 text-center", "{props.title}" }
            div { class: "w-16 h-1 bg-purple-500 rounded" }
        }
    }
}
