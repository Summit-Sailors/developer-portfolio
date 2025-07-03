use dioxus::prelude::*;
use dioxus_free_icons::{
	Icon,
	icons::{
		fa_brands_icons::{FaGithub, FaLinkedin, FaTwitter},
		fi_icons::{FiMail, FiMapPin, FiPhone},
	},
};

use crate::layout::SectionTitle;

#[component]
pub fn ContactPage() -> Element {
	// Basic form state handling (example, replace with actual logic)
	let mut name = use_signal(String::new);
	let mut email = use_signal(String::new);
	let mut subject = use_signal(String::new);
	let mut message = use_signal(String::new);

	let handle_submit = move |evt: Event<FormData>| {
		evt.prevent_default();
		name.set(String::new());
		email.set(String::new());
		subject.set(String::new());
		message.set(String::new());
	};

	rsx! {
		div { class: "py-24",
			div { class: "container mx-auto px-4",
				SectionTitle { title: "Get In Touch".to_owned() }
				div { class: "max-w-4xl mx-auto",
					div { class: "bg-zinc-900 p-8 md:p-12 rounded-lg border border-zinc-800 shadow-xl animate-fade-in-up",
						div { class: "grid md:grid-cols-2 gap-10 md:gap-16",
							// Contact Information Side
							div { class: "space-y-8",
								div {
									h3 { class: "text-2xl font-bold mb-4 text-white",
										"Contact Information"
									}
									p { class: "text-zinc-400 mb-6",
										"Feel free to reach out via email, phone, or connect with me on social media. I'm always open to discussing new projects, creative ideas, or opportunities to be part of something amazing."
									}
									div { class: "space-y-5",
										// Email
										div { class: "flex items-start space-x-4",
											div { class: "p-3 bg-zinc-800 rounded-full mt-1",
												Icon {
													width: 20,
													height: 20,
													icon: FiMail,
													class: "text-purple-500",
												}
											}
											div {
												p { class: "text-zinc-300 font-medium",
													"Email"
												}
												a {
													href: "mailto:jeremy.meek@example.com",
													class: "text-purple-400 hover:text-purple-300 transition-colors",
													"jeremy.meek@example.com"
												}
											}
										}
										// Phone
										div { class: "flex items-start space-x-4",
											div { class: "p-3 bg-zinc-800 rounded-full mt-1",
												Icon {
													width: 20,
													height: 20,
													icon: FiPhone,
													class: "text-purple-500",
												}
											}
											div {
												p { class: "text-zinc-300 font-medium",
													"Phone"
												}
												a {
													href: "tel:+15555555555",
													class: "text-purple-400 hover:text-purple-300 transition-colors",
													"+1 (555) 555-5555"
												}
											}
										}
										// Location
										div { class: "flex items-start space-x-4",
											div { class: "p-3 bg-zinc-800 rounded-full mt-1",
												Icon {
													width: 20,
													height: 20,
													icon: FiMapPin,
													class: "text-purple-500",
												}
											}
											div {
												p { class: "text-zinc-300 font-medium",
													"Location"
												}
												p { class: "text-zinc-400", "Southwest Virginia, USA" }
											}
										}
									}
								}
								div {
									h3 { class: "text-xl font-bold mb-4 text-white",
										"Follow Me"
									}
									div { class: "flex space-x-4",
										a {
											href: "https://github.com/your-github-username",
											target: "_blank",
											rel: "noopener noreferrer",
											class: "p-3 bg-zinc-800 rounded-full text-zinc-400 hover:text-purple-500 hover:bg-zinc-700 transition-all",
											Icon {
												width: 22,
												height: 22,
												icon: FaGithub,
											}
										}
										a {
											href: "https://linkedin.com/in/your-linkedin-profile",
											target: "_blank",
											rel: "noopener noreferrer",
											class: "p-3 bg-zinc-800 rounded-full text-zinc-400 hover:text-purple-500 hover:bg-zinc-700 transition-all",
											Icon {
												width: 22,
												height: 22,
												icon: FaLinkedin,
											}
										}
										a {
											href: "https://twitter.com/your-twitter-handle",
											target: "_blank",
											rel: "noopener noreferrer",
											class: "p-3 bg-zinc-800 rounded-full text-zinc-400 hover:text-purple-500 hover:bg-zinc-700 transition-all",
											Icon {
												width: 22,
												height: 22,
												icon: FaTwitter,
											}
										}
									}
								}
							}

							// Contact Form Side
							div {
								h3 { class: "text-2xl font-bold mb-6 text-white", "Send a Message" }
								form {
									onsubmit: handle_submit,
									class: "space-y-5",
									div {
										label {
											r#for: "name",
											class: "block text-sm font-medium text-zinc-300 mb-1",
											"Your Name"
										}
										input {
											r#type: "text",
											id: "name",
											name: "name",
											placeholder: "e.g. Jane Doe",
											class: "w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white",
											value: "{name}",
											oninput: move |evt| name.set(evt.value()),
										}
									}
									div {
										label {
											r#for: "email",
											class: "block text-sm font-medium text-zinc-300 mb-1",
											"Your Email"
										}
										input {
											r#type: "email",
											id: "email",
											name: "email",
											placeholder: "e.g. jane@example.com",
											class: "w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white",
											value: "{email}",
											oninput: move |evt| email.set(evt.value()),
										}
									}
									div {
										label {
											r#for: "subject",
											class: "block text-sm font-medium text-zinc-300 mb-1",
											"Subject"
										}
										input {
											r#type: "text",
											id: "subject",
											name: "subject",
											placeholder: "e.g. Project Inquiry",
											class: "w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white",
											value: "{subject}",
											oninput: move |evt| subject.set(evt.value()),
										}
									}
									div {
										label {
											r#for: "message",
											class: "block text-sm font-medium text-zinc-300 mb-1",
											"Your Message"
										}
										textarea {
											id: "message",
											name: "message",
											placeholder: "Your message here...",
											rows: 5,
											class: "w-full px-4 py-3 bg-zinc-800 border border-zinc-700 rounded-lg focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-white",
											value: "{message}",
											oninput: move |evt| message.set(evt.value()),
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
