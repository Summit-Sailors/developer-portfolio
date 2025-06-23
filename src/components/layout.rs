#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use crate::constants::{PERSONAL_INFO_DATA, NAVIGATION_LINKS_DATA};
use crate::icons::{FiGithub, FiLinkedin, FiTwitter, FiMenu, FiX};
use crate::Route; // Import the Route enum from main.rs or where it's defined

#[component]
pub fn Header() -> Element {
    let mut is_mobile_menu_open = use_signal(|| false);
    let nav_links = NAVIGATION_LINKS_DATA;

    rsx! {
        header {
            class: "fixed top-0 w-full z-50 backdrop-blur-md bg-black/30 border-b border-zinc-800",
            div {
                class: "container mx-auto px-4 py-4 flex justify-between items-center",
                Link {
                    to: Route::HomePage {},
                    class: "text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 to-cyan-500",
                    "{PERSONAL_INFO_DATA.name.split(' ').next().unwrap_or(\"Dev\")}.dev"
                }
                nav {
                    class: "hidden md:flex space-x-6",
                    for item in nav_links.iter() {
                        Link {
                            key: "{item.name}",
                            to: item.href_route.clone(),
                            // Special handling for #skills link if it's on the homepage
                            onclick: move |_| {
                                if item.name == "Skills" {
                                    // Attempt to scroll to #skills. This is tricky in pure Dioxus without JS interop.
                                    // For now, it navigates to home, user needs to scroll.
                                    // Or, if on HomePage, could try a JS snippet.
                                    log::info!("Skills link clicked, should scroll to #skills on HomePage");
                                }
                            },
                            class: "text-zinc-400 hover:text-white transition-colors",
                            "{item.name}"
                        }
                    }
                }
                div {
                    class: "md:hidden",
                    button {
                        onclick: move |_| is_mobile_menu_open.toggle(),
                        class: "text-zinc-400 hover:text-white p-2",
                        aria_label: "Toggle menu",
                        if *is_mobile_menu_open.read() { rsx! { FiX { size: Some(24) } } }
                        else { rsx! { FiMenu { size: Some(24) } } }
                    }
                }
            }
            // Mobile Menu
            if *is_mobile_menu_open.read() {
                div {
                    class: "md:hidden absolute top-full left-0 w-full bg-black/80 backdrop-blur-md border-b border-zinc-800",
                    nav {
                        class: "flex flex-col items-center space-y-4 py-4",
                        for item in nav_links.iter() {
                            Link {
                                key: "{item.name}-mobile",
                                to: item.href_route.clone(),
                                onclick: move |_| is_mobile_menu_open.set(false),
                                class: "text-zinc-300 hover:text-white text-lg transition-colors",
                                "{item.name}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    let current_year = 2024; // In a real app, get this dynamically if possible or update manually.
                              // For Wasm, getting current year might need JS interop or a crate.
    rsx! {
        footer {
            class: "py-8 border-t border-zinc-800",
            div {
                class: "container mx-auto px-4",
                div {
                    class: "flex flex-col md:flex-row justify-between items-center gap-6",
                    div {
                        class: "text-center md:text-left",
                        Link {
                            to: Route::HomePage {},
                            class: "text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 to-cyan-500 mb-4 md:mb-0",
                            "{PERSONAL_INFO_DATA.name.split(' ').next().unwrap_or(\"Dev\")}.dev"
                        }
                        p {
                            class: "text-zinc-400 text-sm mt-1",
                            "© {current_year} {PERSONAL_INFO_DATA.name}. All rights reserved."
                        }
                    }
                    div {
                        class: "flex space-x-4",
                        a {
                            href: "{PERSONAL_INFO_DATA.github}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-zinc-400 hover:text-purple-500 transition-colors",
                            aria_label: "GitHub profile",
                            FiGithub { size: Some(24) }
                        }
                        a {
                            href: "{PERSONAL_INFO_DATA.linkedin}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-zinc-400 hover:text-purple-500 transition-colors",
                            aria_label: "LinkedIn profile",
                            FiLinkedin { size: Some(24) }
                        }
                        a {
                            href: "{PERSONAL_INFO_DATA.twitter}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-zinc-400 hover:text-purple-500 transition-colors",
                            aria_label: "Twitter profile",
                            FiTwitter { size: Some(24) }
                        }
                    }
                }
            }
        }
    }
}
