
use dioxus::prelude::*;
use dioxus_free_icons::{
	Icon,
	icons::fi_icons::{FiExternalLink, FiGithub},
};

use crate::layout::SectionTitle;

#[component]
pub fn ProjectsPage() -> Element {
	rsx! {
		div { class: "py-24",
			div { class: "container mx-auto px-4",
				SectionTitle { title: "My Projects".to_string() }
				// Manually inlined projects
				div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-10",

					// Project 1: EZPZ Polars Plugin Helper
					div {
						id: "ezpz-polars-plugin-helper",
						class: "group bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden shadow-lg hover:shadow-purple-500/20 transition-all duration-300 ease-in-out flex flex-col animate-fade-in-up",
						style: "animation-delay: 0ms;",
						div { class: "relative w-full h-56",
							img {
								src: "/placeholder.svg?height=224&width=400",
								alt: "EZPZ Polars Plugin Helper",
								class: "object-cover w-full h-full transition-transform group-hover:scale-105 duration-300",
							}
							div { class: "absolute inset-0 bg-gradient-to-t from-black/70 to-transparent" }
							div { class: "absolute top-2 right-2 bg-purple-600 text-white text-xs px-2 py-1 rounded",
								"Pet Project"
							}
						}
						div { class: "p-6 flex flex-col flex-grow",
							h3 { class: "text-2xl font-bold mb-2 text-white",
								"EZPZ Polars Plugin Helper"
							}
							p { class: "text-zinc-400 text-sm mb-4 flex-grow",
								"A Python package enhancing Polars plugin development with type hinting and IDE support. It parses plugin configurations, extracts metadata using libCST, and injects type hints into Polars classes."
							}
							div { class: "mb-4",
								h4 { class: "text-sm font-semibold text-purple-400 mb-1",
									"Key Features:"
								}
								ul { class: "list-disc list-inside text-zinc-400 text-xs space-y-1",
									li { "Provides type hinting and IDE support for Polars plugins." }
									li { "Parses `ezpz_pluginz.toml` for plugin configurations." }
									li {
										"Uses libCST to analyze Python code and extract plugin information."
									}
								}
							}
							div { class: "flex flex-wrap gap-2 mb-6",
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Python"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Rust"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Polars"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"libCST"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"CLI Tool"
								}
							}
							div { class: "mt-auto flex justify-between items-center",
								div { class: "flex space-x-3",
									a {
										href: "https://github.com/Summit-Sailors/EZPZ",
										target: "_blank",
										rel: "noopener noreferrer",
										class: "text-zinc-400 hover:text-purple-500 transition-colors",
										aria_label: "EZPZ Polars Plugin Helper GitHub repository",
										Icon {
											width: 22,
											height: 22,
											icon: FiGithub,
										}
									}
								}
							}
						}
					}

					// Project 2: Lionheart Painting Website
					div {
						id: "lionheart-painting-website",
						class: "group bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden shadow-lg hover:shadow-purple-500/20 transition-all duration-300 ease-in-out flex flex-col animate-fade-in-up",
						style: "animation-delay: 100ms;",
						div { class: "relative w-full h-56",
							img {
								src: "/placeholder.svg?height=224&width=400",
								alt: "Lionheart Painting Website",
								class: "object-cover w-full h-full transition-transform group-hover:scale-105 duration-300",
							}
							div { class: "absolute inset-0 bg-gradient-to-t from-black/70 to-transparent" }
							div { class: "absolute top-2 right-2 bg-purple-600 text-white text-xs px-2 py-1 rounded",
								"Pro Bono Work"
							}
						}
						div { class: "p-6 flex flex-col flex-grow",
							h3 { class: "text-2xl font-bold mb-2 text-white",
								"Lionheart Painting Website"
							}
							p { class: "text-zinc-400 text-sm mb-4 flex-grow",
								"Pro bono website for a painting business that funds a recovery program for convicted felons, offering them work and support."
							}
							div { class: "mb-4",
								h4 { class: "text-sm font-semibold text-purple-400 mb-1",
									"Key Features:"
								}
								ul { class: "list-disc list-inside text-zinc-400 text-xs space-y-1",
									li {
										"Developed a website to showcase the painting business and its mission."
									}
									li {
										"Aims to support a friend's program helping convicted felons reintegrate."
									}
									li {
										"The business provides employment and funding for the recovery program."
									}
								}
							}
							div { class: "flex flex-wrap gap-2 mb-6",
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Web Development"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Pro Bono"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Community"
								}
							}
							div { class: "mt-auto flex justify-between items-center",
								div { class: "flex space-x-3",
									a {
										href: "https://painting.lionheart.church/",
										target: "_blank",
										rel: "noopener noreferrer",
										class: "text-zinc-400 hover:text-purple-500 transition-colors",
										aria_label: "Lionheart Painting Website live demo",
										Icon {
											width: 22,
											height: 22,
											icon: FiExternalLink,
										}
									}
								}
							}
						}
					}

					// Project 3: E-commerce Platform (Example)
					div {
						id: "e-commerce-platform-(example)",
						class: "group bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden shadow-lg hover:shadow-purple-500/20 transition-all duration-300 ease-in-out flex flex-col animate-fade-in-up",
						style: "animation-delay: 200ms;",
						div { class: "relative w-full h-56",
							img {
								src: "/placeholder.svg?height=224&width=400",
								alt: "E-commerce Platform (Example)",
								class: "object-cover w-full h-full transition-transform group-hover:scale-105 duration-300",
							}
							div { class: "absolute inset-0 bg-gradient-to-t from-black/70 to-transparent" }
							div { class: "absolute top-2 right-2 bg-purple-600 text-white text-xs px-2 py-1 rounded",
								"Corporate Experience"
							}
						}
						div { class: "p-6 flex flex-col flex-grow",
							h3 { class: "text-2xl font-bold mb-2 text-white",
								"E-commerce Platform (Example)"
							}
							p { class: "text-zinc-400 text-sm mb-4 flex-grow",
								"A full-featured e-commerce platform built with Next.js and Stripe integration."
							}
							div { class: "mb-4",
								h4 { class: "text-sm font-semibold text-purple-400 mb-1",
									"Key Features:"
								}
								ul { class: "list-disc list-inside text-zinc-400 text-xs space-y-1",
									li { "Example detail 1 for e-commerce." }
									li { "Example detail 2." }
								}
							}
							div { class: "flex flex-wrap gap-2 mb-6",
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Next.js"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"React"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Stripe"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Tailwind CSS"
								}
							}
							div { class: "mt-auto flex justify-between items-center",
								div { class: "flex space-x-3" } // No links for this example
							}
						}
					}

					// Project 4: Task Management App (Example)
					div {
						id: "task-management-app-(example)",
						class: "group bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden shadow-lg hover:shadow-purple-500/20 transition-all duration-300 ease-in-out flex flex-col animate-fade-in-up",
						style: "animation-delay: 300ms;",
						div { class: "relative w-full h-56",
							img {
								src: "/placeholder.svg?height=224&width=400",
								alt: "Task Management App (Example)",
								class: "object-cover w-full h-full transition-transform group-hover:scale-105 duration-300",
							}
							div { class: "absolute inset-0 bg-gradient-to-t from-black/70 to-transparent" }
							div { class: "absolute top-2 right-2 bg-purple-600 text-white text-xs px-2 py-1 rounded",
								"Corporate Experience"
							}
						}
						div { class: "p-6 flex flex-col flex-grow",
							h3 { class: "text-2xl font-bold mb-2 text-white",
								"Task Management App (Example)"
							}
							p { class: "text-zinc-400 text-sm mb-4 flex-grow",
								"A collaborative task management application with real-time updates."
							}
							div { class: "mb-4",
								h4 { class: "text-sm font-semibold text-purple-400 mb-1",
									"Key Features:"
								}
								ul { class: "list-disc list-inside text-zinc-400 text-xs space-y-1",
									li { "Example detail 1 for task app." }
									li { "Example detail 2." }
								}
							}
							div { class: "flex flex-wrap gap-2 mb-6",
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"React"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Firebase"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Tailwind CSS"
								}
								span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300",
									"Redux"
								}
							}
							div { class: "mt-auto flex justify-between items-center",
								div { class: "flex space-x-3" } // No links for this example
							}
						}
					}
				}
			}
		}
	}
}
