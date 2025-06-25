use dioxus::prelude::*;

use crate::layout::SectionTitle;

static PLACEHOLDER: Asset = asset!("/assets/placeholder.svg", ImageAssetOptions::new().with_size(ImageSize::Manual { width: 256, height: 256 }));

#[component]
pub fn StoryPage() -> Element {
	rsx! {
		div { class: "py-24",
			div { class: "container mx-auto px-4",
				SectionTitle { title: "My Story".to_string() }
				div { class: "max-w-3xl mx-auto",
					div { class: "flex flex-col md:flex-row items-center gap-12 mb-16",
						div { class: "md:w-1/3 animate-slide-in-left",
							div { class: "relative size-48 md:size-64 mx-auto",
								div { class: "absolute inset-0 rounded-full bg-gradient-to-r from-purple-500 to-cyan-500 blur-xl opacity-30 animate-pulse-slow" }
								div { class: "relative w-full h-full rounded-full overflow-hidden border-4 border-zinc-800",
									img {
										src: PLACEHOLDER,
										alt: "Jeremy Meek",
										class: "object-cover w-full h-full",
									}
								}
							}
						}
						div { class: "md:w-2/3 animate-slide-in-right text-center md:text-left",
							h1 { class: "text-4xl font-bold mb-4", "Jeremy Meek" }
							p { class: "text-xl text-purple-400 mb-6",
								"Self-Starting Polymath Autodidact."
							}
							p { class: "text-zinc-300 leading-relaxed",
								"Hello! I'm Jeremy Meek, a software developer with a unique journey from farm life in Southwest Virginia, through a Bachelors in Mathematics at Virginia Tech, to a Computer Science education from Bloom Tech. I bring several years of full-stack web development experience from the corporate world, ready to tackle virtually any programming challenge."
							}
						}
					}

					// Manually inlined story paragraphs
					div { class: "space-y-6 text-zinc-300 leading-relaxed animate-fade-in-up animation-delay-300",
						p {
							"My path has been unconventional: from tending cows to unraveling calculus, and now, crafting complex computer systems. This diverse background has equipped me with a unique problem-solving perspective and a relentless drive for learning."
						}
						p {
							"While I'm proficient in mainstream frameworks and languages for professional projects, my passion lies with Rust for its performance and safety. My pet projects often involve Rust, sometimes combined with Python. Svelte is my go-to for frontend JavaScript work, and for non-Rust backends, I prefer FastAPI (Python) or NestJS (Node). When I get to use Rust for full-stack, Dioxus is my framework of choice."
						}
						p {
							"I consider myself an 'Artist Of All Trades' in the tech world, competent to satisfy a wide array of programming needs with creativity and precision."
						}
					}

					// Manually inlined stats
					div { class: "mt-16 animate-fade-in-up animation-delay-500",
						h3 { class: "text-2xl font-bold mb-8 text-center", "Key Milestones" }
						div { class: "grid grid-cols-1 sm:grid-cols-2 gap-6",
							div { class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 hover:border-purple-500 transition-colors",
								h4 { class: "text-xl font-bold mb-2 text-purple-400",
									"N+"
								}
								p { class: "text-zinc-300", "Years of Experience" }
							}
							div { class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 hover:border-purple-500 transition-colors",
								h4 { class: "text-xl font-bold mb-2 text-purple-400",
									"M+"
								}
								p { class: "text-zinc-300", "Projects Completed" }
							}
							div { class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 hover:border-purple-500 transition-colors",
								h4 { class: "text-xl font-bold mb-2 text-purple-400",
									"B.S."
								}
								p { class: "text-zinc-300", "Mathematics, Virginia Tech" }
							}
							div { class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 hover:border-purple-500 transition-colors",
								h4 { class: "text-xl font-bold mb-2 text-purple-400",
									"Cert."
								}
								p { class: "text-zinc-300", "Computer Science, Bloom Tech" }
							}
						}
					}
				}
			}
		}
	}
}
