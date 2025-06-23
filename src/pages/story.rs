#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::constants::{PERSONAL_INFO_DATA, STATS_DATA};
use crate::components::ui::SectionTitle;

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
                                        src: "/placeholder.svg?height=256&width=256",
                                        alt: "{PERSONAL_INFO_DATA.name}",
                                        class: "object-cover w-full h-full"
                                    }
                                }
                            }
                        }
                        div { class: "md:w-2/3 animate-slide-in-right text-center md:text-left",
                            h1 { class: "text-4xl font-bold mb-4", "{PERSONAL_INFO_DATA.name}" }
                            p { class: "text-xl text-purple-400 mb-6", "{PERSONAL_INFO_DATA.title}" }
                            p { class: "text-zinc-300 leading-relaxed", "{PERSONAL_INFO_DATA.story_intro}" }
                        }
                    }

                    div { class: "space-y-6 text-zinc-300 leading-relaxed animate-fade-in-up animation-delay-300",
                        for paragraph in PERSONAL_INFO_DATA.story_detail.iter() {
                            p { "{paragraph}" }
                        }
                    }

                    div { class: "mt-16 animate-fade-in-up animation-delay-500",
                        h3 { class: "text-2xl font-bold mb-8 text-center", "Key Milestones" }
                        div { class: "grid grid-cols-1 sm:grid-cols-2 gap-6",
                            for stat in STATS_DATA.iter() {
                                div {
                                    key: "{stat.label}",
                                    class: "bg-zinc-900 p-6 rounded-lg border border-zinc-800 hover:border-purple-500 transition-colors",
                                    h4 { class: "text-xl font-bold mb-2 text-purple-400", "{stat.value}" }
                                    p { class: "text-zinc-300", "{stat.label}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
