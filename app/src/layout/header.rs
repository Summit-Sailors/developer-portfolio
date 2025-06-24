use dioxus::prelude::*;
use dioxus_free_icons::{
	Icon,
	icons::fi_icons::{FiMenu, FiX},
};
use dioxus_router::prelude::Link;

use crate::Route;

#[component]
pub fn Header() -> Element {
	let mut is_mobile_menu_open = use_signal(|| false);

	rsx! {
		header { class: "fixed top-0 w-full z-50 backdrop-blur-md bg-black/30 border-b border-zinc-800",
			div { class: "container mx-auto px-4 py-4 flex justify-between items-center",
				Link {
					to: Route::HomePage {},
					class: "text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 to-cyan-500",
					"Jeremy.dev"
				}
				// Desktop Nav
				nav { class: "hidden md:flex space-x-6",
					Link {
						to: Route::HomePage {},
						class: "text-zinc-400 hover:text-white transition-colors",
						"Home"
					}
					Link {
						to: Route::ProjectsPage {},
						class: "text-zinc-400 hover:text-white transition-colors",
						"Projects"
					}
					Link {
						to: Route::StoryPage {},
						class: "text-zinc-400 hover:text-white transition-colors",
						"Story"
					}
					Link {
						to: Route::ContactPage {},
						class: "text-zinc-400 hover:text-white transition-colors",
						"Contact"
					}
				}
				// Mobile Menu Button
				div { class: "md:hidden",
					button {
						onclick: move |_| is_mobile_menu_open.toggle(),
						class: "text-zinc-400 hover:text-white p-2",
						aria_label: "Toggle menu",
						if is_mobile_menu_open() {
							Icon { width: 24, height: 24, icon: FiX }
						} else {
							Icon { width: 24, height: 24, icon: FiMenu }
						}
					}
				}
			}
			// Mobile Menu
			if is_mobile_menu_open() {
				div { class: "md:hidden absolute top-full left-0 w-full bg-black/80 backdrop-blur-md border-b border-zinc-800",
					nav { class: "flex flex-col items-center space-y-4 py-4",
						Link {
							to: Route::HomePage {},
							onclick: move |_| is_mobile_menu_open.set(false),
							class: "text-zinc-300 hover:text-white text-lg transition-colors",
							"Home"
						}
						Link {
							to: Route::ProjectsPage {},
							onclick: move |_| is_mobile_menu_open.set(false),
							class: "text-zinc-300 hover:text-white text-lg transition-colors",
							"Projects"
						}
						a {
							href: "/#skills",
							onclick: move |_| is_mobile_menu_open.set(false),
							class: "text-zinc-300 hover:text-white text-lg transition-colors",
							"Skills"
						}
						Link {
							to: Route::StoryPage {},
							onclick: move |_| is_mobile_menu_open.set(false),
							class: "text-zinc-300 hover:text-white text-lg transition-colors",
							"Story"
						}
						Link {
							to: Route::ContactPage {},
							onclick: move |_| is_mobile_menu_open.set(false),
							class: "text-zinc-300 hover:text-white text-lg transition-colors",
							"Contact"
						}
					}
				}
			}
		}
	}
}
