#![allow(non_snake_case)]
use developer_portfolio::{FAVICON, Route, TAILWIND};
use dioxus::prelude::*;

fn main() {
	dioxus::LaunchBuilder::new().launch(App);
}

fn App() -> Element {
	rsx! {
		// {
		// 		#[cfg(not(debug_assertions))]
		// 		{
		// 				rsx! {
		// 					document::Script { src: MATOMO_SCRIPT }
		// 				}
		// 		}
		// }
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
