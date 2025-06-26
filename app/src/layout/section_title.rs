
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SectionTitleProps {
	title: String,
	#[props(default)]
	class_name: String,
}

#[component]
pub fn SectionTitle(props: SectionTitleProps) -> Element {
	let combined_class = format!("flex flex-col items-center mb-16 animate-fade-in-up {}", props.class_name);
	rsx! {
		div { class: "{combined_class}",
			h2 { class: "text-4xl font-bold mb-4 text-center", "{props.title}" }
			div { class: "w-16 h-1 bg-purple-500 rounded" }
		}
	}
}
