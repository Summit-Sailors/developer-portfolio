#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use crate::constants::{PERSONAL_INFO_DATA, SKILLS_LIST, PROJECTS_LIST};
use crate::icons::{FiArrowDown, FiGithub, FiExternalLink};
use crate::components::ui::SectionTitle;
use crate::Route;

#[component]
pub fn HomePage() -> Element {
    let featured_projects = &PROJECTS_LIST[0..2.min(PROJECTS_LIST.len())];

    rsx! {
        // Hero Section
        section {
            id: "home",
            class: "min-h-[calc(100vh-4rem)] flex items-center justify-center relative overflow-hidden",
            // Background blobs
            div { class: "absolute inset-0 z-0",
                div { class: "absolute top-1/4 left-1/4 size-96 bg-purple-500 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob" }
                div { class: "absolute top-1/3 right-1/4 size-96 bg-cyan-500 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob animation-delay-2000" }
                div { class: "absolute bottom-1/4 right-1/3 size-96 bg-pink-500 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob animation-delay-4000" }
            }
            div { class: "container mx-auto px-4 z-10",
                div { class: "flex flex-col md:flex-row items-center justify-between gap-12",
                    // Text content
                    div { class: "md:w-1/2 space-y-6 text-center md:text-left",
                        div { class: "text-sm font-mono text-purple-400 animate-fade-in-up animation-delay-100", "Hello, my name is" }
                        h1 { class: "text-5xl md:text-7xl font-bold animate-fade-in-up animation-delay-200", "{PERSONAL_INFO_DATA.name}" }
                        h2 { class: "text-3xl md:text-5xl font-bold text-zinc-400 animate-fade-in-up animation-delay-300", "{PERSONAL_INFO_DATA.title}" }
                        p { class: "text-zinc-300 max-w-lg mx-auto md:mx-0 animate-fade-in-up animation-delay-400", "{PERSONAL_INFO_DATA.short_description}" }
                        div { class: "flex space-x-4 justify-center md:justify-start animate-fade-in-up animation-delay-500",
                            Link { to: Route::ProjectsPage {}, class: "px-6 py-3 bg-purple-600 hover:bg-purple-700 rounded-md font-medium transition-colors", "View My Work" }
                            Link { to: Route::ContactPage {}, class: "px-6 py-3 border border-zinc-700 hover:border-purple-500 rounded-md font-medium transition-colors", "Contact Me" }
                        }
                    }
                    // Image
                    div { class: "md:w-1/2 flex justify-center animate-fade-in-scale",
                        div { class: "relative size-64 md:size-80",
                            div { class: "absolute inset-0 rounded-full bg-gradient-to-r from-purple-500 to-cyan-500 blur-xl opacity-30 animate-pulse-slow" }
                            div { class: "relative w-full h-full rounded-full overflow-hidden border-4 border-zinc-800",
                                img {
                                    src: "/placeholder.svg?height=320&width=320",
                                    alt: "{PERSONAL_INFO_DATA.name}",
                                    class: "object-cover w-full h-full" // Ensure image fills container
                                }
                            }
                        }
                    }
                }
                // Scroll down arrow
                div { class: "absolute bottom-10 left-1/2 transform -translate-x-1/2 animate-bounce hidden md:block",
                    a { href: "#featured-projects", "aria-label": "Scroll to projects", class: "text-zinc-400 hover:text-white", FiArrowDown { size: Some(24) } }
                }
            }
        }

        // Featured Projects Section
        section { id: "featured-projects", class: "py-24",
            div { class: "container mx-auto px-4",
                SectionTitle { title: "Featured Projects".to_string() }
                div { class: "grid md:grid-cols-2 gap-8",
                    for (index, project) in featured_projects.iter().enumerate() {
                        div {
                            key: "{project.title}",
                            class: "group animate-fade-in-up",
                            style: "animation-delay: {index * 100}ms",
                            div { class: "relative overflow-hidden rounded-lg border border-zinc-800 group-hover:border-purple-500 transition-colors",
                                img {
                                    src: "{project.image}",
                                    alt: "{project.title}",
                                    width: "600", // Provide explicit width/height for non-optimized images
                                    height: "350",
                                    class: "w-full h-64 object-cover transition-transform group-hover:scale-105"
                                }
                                div { class: "absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity z-10 flex flex-col justify-end p-6",
                                    h3 { class: "text-2xl font-bold text-white mb-2", "{project.title}" }
                                    p { class: "text-zinc-300 text-sm mb-3", "{project.description.chars().take(100).collect::<String>()}..." }
                                    Link {
                                        // Constructing an ID for linking within the projects page
                                        to: Route::ProjectsPage {}, // TODO: Add fragment for specific project if router supports it well
                                        class: "text-purple-400 hover:text-purple-300 font-medium self-start",
                                        "Learn More →"
                                    }
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

        // Skills Section
        section { id: "skills", class: "py-24 bg-zinc-900/50",
            div { class: "container mx-auto px-4",
                SectionTitle { title: "My Skills".to_string() }
                div { class: "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6",
                    for (index, skill) in SKILLS_LIST.iter().enumerate() {
                        div {
                            key: "{skill.name}",
                            class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up",
                            style: "animation-delay: {index * 75}ms",
                            div { class: "mb-3 {skill.color}", {(skill.icon)()} }
                            h3 { class: "text-md font-medium", "{skill.name}" }
                            p { class: "text-xs text-zinc-400 mt-1", "{skill.category.as_str()}" }
                        }
                    }
                }
            }
        }
    }
}
