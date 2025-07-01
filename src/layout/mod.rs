mod footer;

use footer::Footer;

mod header;
use header::Header;

mod section_title;
pub use section_title::SectionTitle;

use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
	rsx! {
		div { class: "flex flex-col min-h-screen",
			Header {}
			main { class: "flex-grow pt-16", Outlet::<Route> {} }
			Footer {}
		}
	}
}
