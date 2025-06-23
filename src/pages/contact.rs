#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::constants::PERSONAL_INFO_DATA;
use crate::components::ui::SectionTitle;
use crate::icons::{FiMail, FiPhone, FiMapPin, FiGithub, FiLinkedin, FiTwitter};

#[component]
pub fn ContactPage() -> Element {
    rsx! {
        div { class: "py-24",
            div { class: "container mx-auto px-4",
                SectionTitle { title: "Get In Touch".to_string() }
                div { class: "max-w-4xl mx-auto",
                    div { class: "bg-zinc-900 p-8 md:p-12 rounded-lg border border-zinc-800 shadow-xl animate-fade-in-up",
                        div { class: "grid md:grid-cols-2 gap-10 md:gap-16",
                            // Contact Info Column
                            div { class: "space-y-8",
                                div {
                                    h3 { class: "text-2xl font-bold mb-4 text-white", "Contact Information" }
                                    p { class: "text-zinc-400 mb-6",
                                        "Feel free to reach out via email, phone, or connect with me on social media. I'm always open to discussing new projects, creative ideas, or opportunities to be part of something amazing."
                                    }
                                    div { class: "space-y-5",
                                        // Email
                                        div { class: "flex items-start space-x-4",
                                            div { class: "p-3 bg-zinc-800 rounded-full mt-1", FiMail { size: Some(20), class: Some("text-purple-500".to_string()) } }
                                            div {
                                                p { class: "text-zinc-300 font-medium", "Email" }
                                                a { href: "mailto:{PERSONAL_INFO_DATA.email}", class: "text-purple-400 hover:text-purple-300 transition-colors", "{PERSONAL_INFO_DATA.email}" }
                                            }
                                        }
                                        // Phone
                                        div { class: "flex items-start space-x-4",
                                            div { class: "p-3 bg-zinc-800 rounded-full mt-1", FiPhone { size: Some(20), class: Some("text-purple-500".to_string()) } }
                                            div {
                                                p { class: "text-zinc-300 font-medium", "Phone" }
                                                a { href: "tel:{PERSONAL_INFO_DATA.phone.replace(' ', \"\").replace('(', \"\").replace(')', \"\").replace('-', \"\")}", class: "text-purple-400 hover:text-purple-300 transition-colors", "{PERSONAL_INFO_DATA.phone}" }
                                            }
                                        }
                                        // Location
                                        div { class: "flex items-start space-x-4",
                                            div { class: "p-3 bg-zinc-800 rounded-full mt-1", FiMapPin { size: Some(20), class: Some("text-purple-500".to_string()) } }
                                            div {
                                                p { class: "text-zinc-300 font-medium", "Location" }
                                                p { class: "text-zinc-400", "{PERSONAL_INFO_DATA.location}" }
                                            }
                                        }
                                    }
                                }
                                // Follow Me
                                div {
                                    h3 { class: "text-xl font-bold mb-4 text-white", "Follow Me" }
                                    div { class: "flex space-x-4",
                                        a { href: "{PERSONAL_INFO_DATA.github}", target: "_blank", rel: "noopener noreferrer", class: "p-3 bg-zinc-800 rounded-full text-zinc-400 hover:text-purple-500 hover:bg-zinc-700 transition-all", FiGithub { size: Some(22) } }
                                        a { href: "{PERSONAL_INFO_DATA.linkedin}", target: "_blank", rel: "noopener noreferrer", class: "p-3 bg-zinc-800 rounded-full text-zinc-400 hover:text-purple-500 hover:bg-zinc-700 transition-all", FiLinkedin { size: Some(22) } }
                                        a { href: "{PERSONAL_INFO_DATA.twitter}", target: "_blank", rel: "noopener noreferrer", class: "p-3 bg-zinc-800 rounded-full text-zinc-400 hover:text-purple-500 hover:bg-zinc-700 transition-all", FiTwitter { size: Some(22) } }
                                    }
                                }
                            }
                            // Form Column
                            div {
                                h3 { class: "text-2xl font-bold mb-6 text-white", "Send a Message" }
                                form {
                                    // Prevent default form submission for now, as it's UI only
                                    onsubmit: |ev| ev.stop_propagation(),
                                    class: "space-y-5",
                                    div {
                                        label { class: "block text-sm font-medium text-zinc-300 mb-1", r#for: "name", "Your Name" }
                                        input {
                                            r#type: "text", id: "name", name: "name", placeholder: "e.g. Jane Doe",
                                            class: "w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white"
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-zinc-300 mb-1", r#for: "email", "Your Email" }
                                        input {
                                            r#type: "email", id: "email", name: "email", placeholder: "e.g. jane@example.com",
                                            class: "w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white"
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-zinc-300 mb-1", r#for: "subject", "Subject" }
                                        input {
                                            r#type: "text", id: "subject", name: "subject", placeholder: "e.g. Project Inquiry",
                                            class: "w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white"
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm font-medium text-zinc-300 mb-1", r#for: "message", "Your Message" }
                                        textarea {
                                            id: "message", name: "message", placeholder: "Your message here...", rows: 5,
                                            class: "w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white"
                                        }
                                    }
                                    button {
                                        r#type: "submit",
                                        class: "w-full px-6 py-3 bg-gradient-to-r from-purple-600 to-cyan-600 rounded-lg font-semibold text-white hover:opacity-90 transition-opacity focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 focus:ring-offset-zinc-900",
                                        "Send Message"
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
