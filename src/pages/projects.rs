#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::constants::PROJECTS_LIST;
use crate::components::ui::SectionTitle;
use crate::icons::{FiGithub, FiExternalLink};

#[component]
pub fn ProjectsPage() -> Element {
    rsx! {
        div { class: "py-24",
            div { class: "container mx-auto px-4",
                SectionTitle { title: "My Projects".to_string() }
                div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-10",
                    for (index, project) in PROJECTS_LIST.iter().enumerate() {
                        div {
                            key: "{project.title}",
                            // id: "{project.title.to_lowercase().replace(' ', \"-\")}", // For potential anchor links
                            class: "group bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden shadow-lg hover:shadow-purple-500/20 transition-all duration-300 ease-in-out flex flex-col animate-fade-in-up",
                            style: "animation-delay: {index * 100}ms",
                            div { class: "relative w-full h-56",
                                img {
                                    src: "{project.image}",
                                    alt: "{project.title}",
                                    class: "object-cover w-full h-full transition-transform group-hover:scale-105 duration-300"
                                }
                                div { class: "absolute inset-0 bg-gradient-to-t from-black/70 to-transparent" }
                                div { class: "absolute top-2 right-2 bg-purple-600 text-white text-xs px-2 py-1 rounded",
                                    "{project.category}"
                                }
                            }
                            div { class: "p-6 flex flex-col flex-grow",
                                h3 { class: "text-2xl font-bold mb-2 text-white", "{project.title}" }
                                p { class: "text-zinc-400 text-sm mb-4 flex-grow", "{project.description}" }

                                if let Some(details) = project.details {
                                    if !details.is_empty() {
                                        div { class: "mb-4",
                                            h4 { class: "text-sm font-semibold text-purple-400 mb-1", "Key Features:" }
                                            ul { class: "list-disc list-inside text-zinc-400 text-xs space-y-1",
                                                for detail_item in details.iter().take(3) {
                                                    li { "{detail_item}" }
                                                }
                                            }
                                        }
                                    }
                                }

                                div { class: "flex flex-wrap gap-2 mb-6",
                                    for tag_item in project.tags.iter() {
                                        span { class: "px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300", "{tag_item}" }
                                    }
                                }
                                div { class: "mt-auto flex justify-between items-center",
                                    div { class: "flex space-x-3",
                                        if let Some(link) = project.github_link {
                                            a { href: "{link}", target: "_blank", rel: "noopener noreferrer", class: "text-zinc-400 hover:text-purple-500 transition-colors", aria_label: "{project.title} GitHub repository", FiGithub { size: Some(22) } }
                                        }
                                        if let Some(link) = project.live_link {
                                            a { href: "{link}", target: "_blank", rel: "noopener noreferrer", class: "text-zinc-400 hover:text-purple-500 transition-colors", aria_label: "{project.title} live demo", FiExternalLink { size: Some(22) } }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
