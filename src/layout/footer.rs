use dioxus::prelude::*;
use dioxus_free_icons::{
	Icon,
	icons::fa_brands_icons::{FaGithub, FaLinkedin, FaTwitter},
};
use dioxus_router::prelude::Link;

use crate::Route;

#[component]
pub fn Footer() -> Element {
	let current_year = "2024";

	rsx! {
		footer { class: "py-8 border-t border-zinc-800",
			div { class: "container mx-auto px-4",
				div { class: "flex flex-col md:flex-row justify-between items-center gap-6",
					div { class: "text-center md:text-left",
						Link {
							to: Route::HomePage {},
							class: "text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 to-cyan-500 mb-4 md:mb-0",
							"Jeremy.dev"
						}
						p { class: "text-zinc-400 text-sm mt-1",
							"© {current_year} Jeremy Meek. All rights reserved."
						}
					}
					div { class: "flex space-x-4",
						a {
							href: "https://github.com/your-github-username", // Replace
							target: "_blank",
							rel: "noopener noreferrer",
							class: "text-zinc-400 hover:text-purple-500 transition-colors",
							Icon { width: 24, height: 24, icon: FaGithub }
						}
						a {
							href: "https://linkedin.com/in/your-linkedin-profile", // Replace
							target: "_blank",
							rel: "noopener noreferrer",
							class: "text-zinc-400 hover:text-purple-500 transition-colors",
							Icon { width: 24, height: 24, icon: FaLinkedin }
						}
						a {
							href: "https://twitter.com/your-twitter-handle", // Replace
							target: "_blank",
							rel: "noopener noreferrer",
							class: "text-zinc-400 hover:text-purple-500 transition-colors",
							Icon { width: 24, height: 24, icon: FaTwitter }
						}
					}
				}
			}
		}
	}
}
