#![allow(non_snake_case)]
use app::Route;
use dioxus::prelude::*;

fn main() {
	dioxus::LaunchBuilder::new().launch(App);
}

#[cfg(not(debug_assertions))]
static MATOMO_SCRIPT: Asset = asset!("/assets/matomo_tag.js");
static FAVICON: Asset = asset!("/assets/favicon.png");
static TAILWIND: Asset = asset!("/assets/tailwind.css");

fn App() -> Element {
	rsx! {
		{
				#[cfg(not(debug_assertions))]
				{
						rsx! {
							document::Script { src: MATOMO_SCRIPT }
						}
				}
		}
		document::Link { rel: "icon", href: FAVICON }
		document::Meta {
			name: "description",
			content: "Jeremy Meek, the Self-Starting Polymath Autodidact Atrist of All Trades.",
		}
		document::Meta { name: "keywords", content: "software engineering, freelancing" }
		document::Link {
			rel: "stylesheet",
			href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;900&display=swap",
		}
		document::Link { rel: "stylesheet", href: TAILWIND }
		Router::<Route> {}
	}
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
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
