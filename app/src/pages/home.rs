use dioxus::prelude::*;
use dioxus_free_icons::{
	Icon,
	icons::{
		fa_brands_icons::{FaJsSquare, FaNodeJs, FaPython, FaReact, FaRust},
		fa_solid_icons::{FaAtom, FaPalette, FaServer},
		fi_icons::FiArrowDown,
	},
};
use dioxus_router::prelude::Link;

use crate::{layout::SectionTitle, router::Route};

static HEADSHOT: Asset = asset!("/assets/headshot.png", ImageAssetOptions::new());
static PLACEHOLDER: Asset = asset!("/assets/placeholder.svg", ImageAssetOptions::new().with_size(ImageSize::Manual { width: 600, height: 350 }));

#[component]
pub fn HomePage() -> Element {
	rsx! {
		// Hero Section (no icons here, no changes needed)
		section {
			id: "home",
			class: "min-h-[calc(100vh-4rem)] flex items-center justify-center relative overflow-hidden",
			// ... hero content from previous step ...
			div { class: "absolute inset-0 z-0",
				div { class: "absolute top-1/4 left-1/4 size-96 bg-purple-500 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob" }
				div { class: "absolute top-1/3 right-1/4 size-96 bg-cyan-500 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob animation-delay-2000" }
				div { class: "absolute bottom-1/4 right-1/3 size-96 bg-pink-500 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob animation-delay-4000" }
			}
			div { class: "container mx-auto px-4 z-10",
				div { class: "flex flex-col md:flex-row items-center justify-between gap-12",
					div { class: "md:w-1/2 space-y-6 text-center md:text-left",
						div { class: "text-sm font-mono text-purple-400 animate-fade-in-up animation-delay-100",
							"Hello, my name is"
						}
						h1 { class: "text-5xl md:text-7xl font-bold animate-fade-in-up animation-delay-200",
							"Jeremy Meek"
						}
						h2 { class: "text-3xl md:text-5xl font-bold text-zinc-400 animate-fade-in-up animation-delay-300",
							"Self-Starting Polymath Autodidact."
						}
						p { class: "text-zinc-300 max-w-lg mx-auto md:mx-0 animate-fade-in-up animation-delay-400",
							"I build robust and innovative solutions for the web and beyond. From cows, to calculus, to computers."
						}
						div { class: "flex space-x-4 justify-center md:justify-start animate-fade-in-up animation-delay-500",
							Link {
								to: Route::ProjectsPage {},
								class: "px-6 py-3 bg-purple-600 hover:bg-purple-700 rounded-md font-medium transition-colors",
								"View My Work"
							}
							Link {
								to: Route::ContactPage {},
								class: "px-6 py-3 border border-zinc-700 hover:border-purple-500 rounded-md font-medium transition-colors",
								"Contact Me"
							}
						}
					}
					div { class: "md:w-1/2 flex justify-center animate-fade-in-scale",
						div { class: "relative size-64 md:size-80",
							div { class: "absolute inset-0 rounded-full bg-gradient-to-r from-purple-500 to-cyan-500 blur-xl opacity-30 animate-pulse-slow" }
							div { class: "relative w-full h-full rounded-full overflow-hidden border-4 border-zinc-800",
								img {
									src: HEADSHOT,
									alt: "Jeremy Meek",
									class: "object-cover w-full h-full",
								}
							}
						}
					}
				}
				div { class: "absolute bottom-10 left-1/2 transform -translate-x-1/2 animate-bounce hidden md:block",
					a {
						href: "#featured-projects",
						aria_label: "Scroll to projects",
						class: "text-zinc-400 hover:text-white",
						Icon { width: 24, height: 24, icon: FiArrowDown }
					}
				}
			}
		}

		// Featured Projects Section (no icons here, no changes needed)
		section { id: "featured-projects", class: "py-24",
			// ... featured projects content from previous step ...
			div { class: "container mx-auto px-4",
				SectionTitle { title: "Featured Projects".to_string() }
				div { class: "grid md:grid-cols-2 gap-8",
					div {
						class: "group animate-fade-in-up",
						style: "animation-delay: 0ms;",
						div { class: "relative overflow-hidden rounded-lg border border-zinc-800 group-hover:border-purple-500 transition-colors",
							img {
								src: PLACEHOLDER,
								alt: "EZPZ Polars Plugin Helper",
								class: "w-full h-64 object-cover transition-transform group-hover:scale-105",
							}
							div { class: "absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity z-10 flex flex-col justify-end p-6",
								h3 { class: "text-2xl font-bold text-white mb-2",
									"EZPZ Polars Plugin Helper"
								}
								p { class: "text-zinc-300 text-sm mb-3",
									"A Python package enhancing Polars plugin development with type hinting and IDE support. It parses..."
								}
								a {
									href: "/projects#ezpz-polars-plugin-helper",
									class: "text-purple-400 hover:text-purple-300 font-medium self-start",
									"Learn More →"
								}
							}
						}
					}
					div {
						class: "group animate-fade-in-up",
						style: "animation-delay: 100ms;",
						div { class: "relative overflow-hidden rounded-lg border border-zinc-800 group-hover:border-purple-500 transition-colors",
							img {
								src: PLACEHOLDER,
								alt: "Lionheart Painting Website",
								class: "w-full h-64 object-cover transition-transform group-hover:scale-105",
							}
							div { class: "absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity z-10 flex flex-col justify-end p-6",
								h3 { class: "text-2xl font-bold text-white mb-2",
									"Lionheart Painting Website"
								}
								p { class: "text-zinc-300 text-sm mb-3",
									"Pro bono website for a painting business that funds a recovery program for convicted felons..."
								}
								a {
									href: "/projects#lionheart-painting-website",
									class: "text-purple-400 hover:text-purple-300 font-medium self-start",
									"Learn More →"
								}
							}
						}
					}
				}
				div { class: "text-center mt-12 animate-fade-in-up animation-delay-300",
					Link {
						to: Route::ProjectsPage {},
						class: "px-8 py-3 bg-purple-600 hover:bg-purple-700 rounded-md font-medium transition-colors text-lg",
						"View All Projects"
					}
				}
			}
		}

		// Skills Section - Updated with dioxus-free-icons
		section { id: "skills", class: "py-24 bg-zinc-900/50",
			div { class: "container mx-auto px-4",
				SectionTitle { title: "My Skills".to_string() }
				div { class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6",
					// Skill: Rust
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 0ms;",
						div { class: "mb-3 text-4xl text-orange-500",
							Icon { width: 40, height: 40, icon: FaRust }
						}
						h3 { class: "text-md font-medium", "Rust" }
						p { class: "text-xs text-zinc-400 mt-1", "Favorite" }
					}
					// Skill: Svelte
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 75ms;",
						div { class: "mb-3 text-4xl text-orange-600",
							Icon { width: 40, height: 40, icon: FaRust }
						}
						h3 { class: "text-md font-medium", "Svelte" }
						p { class: "text-xs text-zinc-400 mt-1", "Favorite" }
					}
					// Skill: Dioxus (Placeholder)
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 150ms;",
						div { class: "mb-3 text-4xl text-purple-400",
							Icon { width: 40, height: 40, icon: FaAtom }
						}
						h3 { class: "text-md font-medium", "Dioxus" }
						p { class: "text-xs text-zinc-400 mt-1", "Favorite" }
					}
					// Skill: Python
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 225ms;",
						div { class: "mb-3 text-4xl text-yellow-500",
							Icon { width: 40, height: 40, icon: FaPython }
						}
						h3 { class: "text-md font-medium", "Python" }
						p { class: "text-xs text-zinc-400 mt-1", "Proficient" }
					}
					// Skill: FastAPI (Placeholder)
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 300ms;",
						div { class: "mb-3 text-4xl text-green-500",
							Icon { width: 40, height: 40, icon: FaServer }
						}
						h3 { class: "text-md font-medium", "FastAPI" }
						p { class: "text-xs text-zinc-400 mt-1", "Backend" }
					}
					// Skill: NestJS (Placeholder)
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 375ms;",
						div { class: "mb-3 text-4xl text-red-500",
							Icon { width: 40, height: 40, icon: FaNodeJs }
						}
						h3 { class: "text-md font-medium", "NestJS" }
						p { class: "text-xs text-zinc-400 mt-1", "Backend" }
					}
					// Skill: React
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 450ms;",
						div { class: "mb-3 text-4xl text-cyan-400",
							Icon { width: 40, height: 40, icon: FaReact }
						}
						h3 { class: "text-md font-medium", "React" }
						p { class: "text-xs text-zinc-400 mt-1", "Proficient" }
					}
					// Skill: Next.js (Placeholder)
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 525ms;",
						div { class: "mb-3 text-4xl text-white",
							Icon { width: 40, height: 40, icon: FaNodeJs }
						}
						h3 { class: "text-md font-medium", "Next.js" }
						p { class: "text-xs text-zinc-400 mt-1", "Proficient" }
					}
					// Skill: Tailwind (Placeholder)
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 600ms;",
						div { class: "mb-3 text-4xl text-cyan-500",
							Icon { width: 40, height: 40, icon: FaPalette }
						}
						h3 { class: "text-md font-medium", "Tailwind" }
						p { class: "text-xs text-zinc-400 mt-1", "Proficient" }
					}
					// Skill: TypeScript (Placeholder)
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 675ms;",
						div { class: "mb-3 text-4xl text-blue-500",
							Icon { width: 40, height: 40, icon: FaJsSquare }
						}
						h3 { class: "text-md font-medium", "TypeScript" }
						p { class: "text-xs text-zinc-400 mt-1", "Proficient" }
					}
					// Skill: Node.js
					div {
						class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
						style: "animation-delay: 750ms;",
						div { class: "mb-3 text-4xl text-green-600",
							Icon { width: 40, height: 40, icon: FaNodeJs }
						}
						h3 { class: "text-md font-medium", "Node.js" }
						p { class: "text-xs text-zinc-400 mt-1", "Proficient" }
					}
				}
			}
		}
	}
}
