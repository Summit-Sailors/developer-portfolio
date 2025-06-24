use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
	let route = segments.join("/");
	rsx! {
		div { class: "container mx-auto px-4 py-24 text-center",
			h1 { class: "text-4xl font-bold text-purple-500 mb-4", "404 - Page Not Found" }
			p { class: "text-zinc-300",
				"Sorry, the page you were looking for at '/{route}' does not exist."
			}
			Link {
				to: Route::HomePage {},
				class: "mt-8 inline-block px-6 py-3 bg-purple-600 hover:bg-purple-700 rounded-md font-medium transition-colors",
				"Go to Homepage"
			}
		}
	}
}
